use at_protocol::frame::{read_frame, write_frame};
use at_protocol::Message;
use std::io::{Error, ErrorKind, Result};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::Semaphore;

const SCAN_PORT: u16 = 19800;
const SCAN_TIMEOUT: Duration = Duration::from_millis(200);
const SCAN_CONCURRENCY: usize = 64;

/// Scan the local network segment(s) and return the first AT server found.
pub async fn find_server() -> Result<IpAddr> {
    let subnets = get_local_subnets()?;
    if subnets.is_empty() {
        return Err(Error::new(
            ErrorKind::NotFound,
            "No usable network interfaces found",
        ));
    }

    for (ip, prefix_len) in &subnets {
        log::info!(
            "Scanning subnet from interface {} /{} on port {}",
            ip,
            prefix_len,
            SCAN_PORT
        );

        match scan_subnet(*ip, *prefix_len).await {
            Ok(server_ip) => return Ok(IpAddr::V4(server_ip)),
            Err(e) => {
                log::debug!("Subnet scan for {} failed: {}", ip, e);
                continue;
            }
        }
    }

    Err(Error::new(
        ErrorKind::NotFound,
        "No AT server found on any local subnet",
    ))
}

/// Get local IPv4 addresses with their prefix lengths.
/// Filters out loopback (127.x) and link-local (169.254.x) addresses.
fn get_local_subnets() -> Result<Vec<(Ipv4Addr, u32)>> {
    let ifaces = if_addrs::get_if_addrs()
        .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to get interfaces: {}", e)))?;

    let mut subnets = Vec::new();
    for iface in ifaces {
        if iface.is_loopback() {
            continue;
        }
        if let std::net::IpAddr::V4(ipv4) = iface.ip() {
            // Skip link-local
            if ipv4.octets()[0] == 169 && ipv4.octets()[1] == 254 {
                continue;
            }
            // Extract prefix length from the netmask
            let prefix_len = match &iface.addr {
                if_addrs::IfAddr::V4(v4) => {
                    let mask = v4.netmask;
                    let mask_bits = u32::from(mask);
                    mask_bits.count_ones()
                }
                _ => continue,
            };
            subnets.push((ipv4, prefix_len));
        }
    }

    Ok(subnets)
}

/// Scan a /prefix_len subnet for the AT server.
async fn scan_subnet(local_ip: Ipv4Addr, prefix_len: u32) -> Result<Ipv4Addr> {
    let ip_u32 = u32::from(local_ip);
    let mask = if prefix_len >= 32 {
        0xFFFF_FFFFu32
    } else {
        !((1u32 << (32 - prefix_len)) - 1)
    };
    let network = ip_u32 & mask;
    let broadcast = network | !mask;

    // Don't scan subnets larger than /16 (65534 hosts) to avoid flooding
    let host_count = broadcast.saturating_sub(network).saturating_sub(1);
    if host_count == 0 || host_count > 65534 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Subnet too large or invalid: {} hosts", host_count),
        ));
    }

    let semaphore = Arc::new(Semaphore::new(SCAN_CONCURRENCY));
    let (result_tx, mut result_rx) = tokio::sync::mpsc::channel::<Ipv4Addr>(1);
    let cancelled = Arc::new(AtomicBool::new(false));

    let mut handles = Vec::new();

    // Scan network+1 to broadcast-1 (skip network and broadcast addresses)
    for host_u32 in (network + 1)..broadcast {
        let target_ip = Ipv4Addr::from(host_u32);
        // Skip our own IP
        if target_ip == local_ip {
            continue;
        }

        let permit = semaphore
            .clone()
            .acquire_owned()
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("Semaphore error: {}", e)))?;
        let tx = result_tx.clone();
        let flag = cancelled.clone();

        let handle = tokio::spawn(async move {
            let _permit = permit;
            if flag.load(Ordering::Relaxed) {
                return;
            }
            if let Ok(true) = try_handshake(target_ip).await {
                let _ = tx.send(target_ip).await;
            }
        });
        handles.push(handle);
    }

    // Drop our copy so the channel closes when all tasks finish
    drop(result_tx);

    // Wait for first result or all tasks to complete
    let result = result_rx.recv().await;

    // Signal remaining tasks to skip work
    cancelled.store(true, Ordering::Relaxed);

    match result {
        Some(ip) => Ok(ip),
        None => Err(Error::new(
            ErrorKind::NotFound,
            "No AT server found in subnet",
        )),
    }
}

/// Try to connect to an IP on SCAN_PORT and perform a handshake.
/// Returns Ok(true) if it is a valid AT server.
async fn try_handshake(ip: Ipv4Addr) -> Result<bool> {
    let addr = std::net::SocketAddr::new(IpAddr::V4(ip), SCAN_PORT);

    // Connect with timeout
    let mut stream = match tokio::time::timeout(SCAN_TIMEOUT, TcpStream::connect(addr)).await {
        Ok(Ok(s)) => s,
        _ => return Ok(false),
    };

    // Send handshake message
    let handshake = Message::Handshake;
    let json = serde_json::to_vec(&handshake)
        .map_err(|e| Error::new(ErrorKind::Other, format!("serialize error: {}", e)))?;

    if tokio::time::timeout(SCAN_TIMEOUT, write_frame(&mut stream, &json))
        .await
        .is_err()
    {
        return Ok(false);
    }

    // Read response with timeout
    let response_data =
        match tokio::time::timeout(Duration::from_millis(500), read_frame(&mut stream)).await {
            Ok(Ok(data)) => data,
            _ => return Ok(false),
        };

    // Parse response
    match serde_json::from_slice::<Message>(&response_data) {
        Ok(Message::HandshakeResponse { .. }) => {
            log::info!("Found AT server at {}", ip);
            Ok(true)
        }
        _ => Ok(false),
    }
}
