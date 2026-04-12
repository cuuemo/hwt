// AT Web UI - Shared JavaScript utilities

// ─── i18n ──────────────────────────────────────────────────────────
var I18N = {
    en: {
        // Login
        'login.title': 'Net Admin Server',
        'login.heading': 'Login',
        'login.account': 'Account',
        'login.password': 'Password',
        'login.ph_account': 'Enter account',
        'login.ph_password': 'Enter password',
        'login.btn': 'Login',
        'login.no_account': 'No account? Register now',
        'login.network_error': 'Network error',
        // Register
        'reg.heading': 'Register',
        'reg.account': 'Account',
        'reg.password': 'Password',
        'reg.confirm': 'Confirm Password',
        'reg.ph_account': '2-64 chars',
        'reg.ph_password': 'At least 6 chars',
        'reg.ph_confirm': 'Re-enter password',
        'reg.btn': 'Register',
        'reg.has_account': 'Already have an account? Login',
        'reg.err_short_user': 'Account must be at least 2 chars',
        'reg.err_short_pass': 'Password must be at least 6 chars',
        'reg.err_mismatch': 'Passwords do not match',
        'reg.network_error': 'Network error',
        // Dashboard
        'dash.title': 'Net Admin Server',
        'dash.running': 'Running',
        'dash.auth_heading': 'Authorization',
        'dash.status': 'Status',
        'dash.license': 'License',
        'dash.expires': 'Expires',
        'dash.machine_code': 'Machine Code',
        'dash.last_verify': 'Last Verified',
        'dash.clients_heading': 'Online Clients',
        'dash.th_ip': 'IP',
        'dash.th_client_id': 'Client ID',
        'dash.th_connected': 'Connected',
        'dash.log_heading': 'Log',
        'dash.no_logs': 'No logs yet',
        'dash.authorized': 'Authorized',
        'dash.not_authorized': 'Not Authorized',
        'dash.permanent': 'Permanent',
        // Client
        'client.title': 'AT Client',
        'client.initializing': 'Initializing',
        'client.connected': 'Connected',
        'client.searching': 'Searching',
        'client.disconnected': 'Disconnected',
        'client.status_heading': 'Status',
        'client.connection': 'Connection',
        'client.authorization': 'Authorization',
        'client.cleanup': 'Cleanup',
        'client.heartbeat': 'Heartbeat',
        'client.log_heading': 'Log',
        'client.no_logs': 'Waiting for events...',
        'client.pending': 'Pending',
        // Log levels
        'log.info': 'INFO',
        'log.success': 'OK',
        'log.warn': 'WARN',
        'log.error': 'ERR',
        // WS Status
        'ws.connecting': 'Connecting...',
        'ws.connected': 'Online',
        // Lang toggle
        'lang.toggle': 'CN'
    },
    zh: {
        'login.title': 'Net Admin Server',
        'login.heading': '\u767b\u5f55',
        'login.account': '\u8d26\u53f7',
        'login.password': '\u5bc6\u7801',
        'login.ph_account': '\u8bf7\u8f93\u5165\u8d26\u53f7',
        'login.ph_password': '\u8bf7\u8f93\u5165\u5bc6\u7801',
        'login.btn': '\u767b \u5f55',
        'login.no_account': '\u6ca1\u6709\u8d26\u53f7\uff1f\u7acb\u5373\u6ce8\u518c',
        'login.network_error': '\u7f51\u7edc\u9519\u8bef',
        'reg.heading': '\u6ce8\u518c',
        'reg.account': '\u8d26\u53f7',
        'reg.password': '\u5bc6\u7801',
        'reg.confirm': '\u786e\u8ba4\u5bc6\u7801',
        'reg.ph_account': '2-64\u4e2a\u5b57\u7b26',
        'reg.ph_password': '\u81f3\u5c116\u4e2a\u5b57\u7b26',
        'reg.ph_confirm': '\u518d\u6b21\u8f93\u5165\u5bc6\u7801',
        'reg.btn': '\u6ce8 \u518c',
        'reg.has_account': '\u5df2\u6709\u8d26\u53f7\uff1f\u8fd4\u56de\u767b\u5f55',
        'reg.err_short_user': '\u8d26\u53f7\u81f3\u5c112\u4e2a\u5b57\u7b26',
        'reg.err_short_pass': '\u5bc6\u7801\u81f3\u5c116\u4e2a\u5b57\u7b26',
        'reg.err_mismatch': '\u4e24\u6b21\u5bc6\u7801\u4e0d\u4e00\u81f4',
        'reg.network_error': '\u7f51\u7edc\u9519\u8bef',
        'dash.title': 'Net Admin Server',
        'dash.running': '\u8fd0\u884c\u4e2d',
        'dash.auth_heading': '\u6388\u6743\u4fe1\u606f',
        'dash.status': '\u72b6\u6001',
        'dash.license': '\u6388\u6743\u7c7b\u578b',
        'dash.expires': '\u8fc7\u671f\u65f6\u95f4',
        'dash.machine_code': '\u673a\u5668\u7801',
        'dash.last_verify': '\u4e0a\u6b21\u9a8c\u8bc1',
        'dash.clients_heading': '\u5728\u7ebf\u5ba2\u6237\u7aef',
        'dash.th_ip': 'IP',
        'dash.th_client_id': '\u5ba2\u6237\u7aefID',
        'dash.th_connected': '\u8fde\u63a5\u65f6\u95f4',
        'dash.log_heading': '\u65e5\u5fd7',
        'dash.no_logs': '\u6682\u65e0\u65e5\u5fd7',
        'dash.authorized': '\u5df2\u6388\u6743',
        'dash.not_authorized': '\u672a\u6388\u6743',
        'dash.permanent': '\u6c38\u4e45',
        'client.title': 'AT \u5ba2\u6237\u7aef',
        'client.initializing': '\u521d\u59cb\u5316',
        'client.connected': '\u5df2\u8fde\u63a5',
        'client.searching': '\u641c\u7d22\u4e2d',
        'client.disconnected': '\u5df2\u65ad\u5f00',
        'client.status_heading': '\u72b6\u6001',
        'client.connection': '\u8fde\u63a5',
        'client.authorization': '\u6388\u6743',
        'client.cleanup': '\u6e05\u7406',
        'client.heartbeat': '\u5fc3\u8df3',
        'client.log_heading': '\u65e5\u5fd7',
        'client.no_logs': '\u7b49\u5f85\u4e8b\u4ef6...',
        'client.pending': '\u7b49\u5f85\u4e2d',
        'log.info': '\u4fe1\u606f',
        'log.success': '\u6210\u529f',
        'log.warn': '\u8b66\u544a',
        'log.error': '\u9519\u8bef',
        'ws.connecting': '\u8fde\u63a5\u4e2d...',
        'ws.connected': '\u5728\u7ebf',
        'lang.toggle': 'EN'
    }
};

var currentLang = localStorage.getItem('at-lang') || 'en';

function t(key) {
    return (I18N[currentLang] && I18N[currentLang][key]) || (I18N.en[key]) || key;
}

function applyI18n() {
    document.querySelectorAll('[data-i18n]').forEach(function(el) {
        el.textContent = t(el.getAttribute('data-i18n'));
    });
    document.querySelectorAll('[data-i18n-ph]').forEach(function(el) {
        el.placeholder = t(el.getAttribute('data-i18n-ph'));
    });
    document.querySelectorAll('[data-i18n-empty]').forEach(function(el) {
        el.setAttribute('data-empty', t(el.getAttribute('data-i18n-empty')));
    });
}

function toggleLang() {
    currentLang = currentLang === 'en' ? 'zh' : 'en';
    localStorage.setItem('at-lang', currentLang);
    applyI18n();
}

// ─── WebSocket ─────────────────────────────────────────────────────

function updateWSStatus(connected) {
    var el = document.getElementById('ws-status');
    if (!el) return;
    el.className = 'status-indicator ' + (connected ? 'online' : 'offline');
    var textEl = document.getElementById('ws-status-text');
    if (textEl) textEl.textContent = t(connected ? 'ws.connected' : 'ws.connecting');
}

function connectWS(onMessage) {
    var proto = location.protocol === 'https:' ? 'wss:' : 'ws:';
    var url = proto + '//' + location.host + '/ws';
    var ws = new WebSocket(url);
    
    ws.onopen = function() {
        updateWSStatus(true);
    };
    
    ws.onmessage = function(e) {
        try { onMessage(JSON.parse(e.data)); } catch(err) { console.error(err); }
    };
    
    ws.onclose = function() {
        updateWSStatus(false);
        setTimeout(function() { connectWS(onMessage); }, 3000);
    };
    
    ws.onerror = function() { 
        ws.close(); 
    };
    
    return ws;
}

// ─── Log rendering ─────────────────────────────────────────────────

function appendLog(container, timestamp, level, message) {
    container.classList.remove('empty');
    var safeLevel = ['info','success','warn','error'].indexOf(level) >= 0 ? level : 'info';

    var line = document.createElement('div');
    line.className = 'log-line level-' + safeLevel;

    var timeEl = document.createElement('span');
    timeEl.className = 'log-time';
    timeEl.textContent = timestamp;

    var tag = document.createElement('span');
    tag.className = 'log-level ' + safeLevel;
    tag.textContent = t('log.' + safeLevel);

    var text = document.createElement('span');
    text.className = 'log-text';
    text.textContent = message;

    line.appendChild(timeEl);
    line.appendChild(tag);
    line.appendChild(text);
    container.prepend(line);

    while (container.children.length > 100) {
        container.removeChild(container.lastElementChild);
    }
}

// ─── Utilities ─────────────────────────────────────────────────────

function updateCurrentTime(el) {
    function tick() {
        var now = new Date();
        el.textContent = now.toLocaleString('zh-CN', {
            year: 'numeric', month: '2-digit', day: '2-digit',
            hour: '2-digit', minute: '2-digit', second: '2-digit'
        });
    }
    tick();
    setInterval(tick, 1000);
}

function showLoading(show) {
    var el = document.getElementById('loading');
    if (el) el.classList.toggle('show', show);
}
