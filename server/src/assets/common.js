// HWT Web UI - Shared JavaScript utilities

function connectWS(onMessage) {
    const proto = location.protocol === 'https:' ? 'wss:' : 'ws:';
    const url = proto + '//' + location.host + '/ws';
    const ws = new WebSocket(url);
    ws.onmessage = function(e) {
        try { onMessage(JSON.parse(e.data)); } catch(err) { console.error(err); }
    };
    ws.onclose = function() {
        setTimeout(function() { connectWS(onMessage); }, 3000);
    };
    ws.onerror = function() { ws.close(); };
    return ws;
}

function appendLog(container, timestamp, level, message) {
    container.classList.remove('empty');
    var safeLevel = ['info','success','warn','error'].indexOf(level) >= 0 ? level : 'info';
    var levelTextMap = { info: '\u4fe1\u606f', success: '\u6210\u529f', warn: '\u8b66\u544a', error: '\u9519\u8bef' };

    var line = document.createElement('div');
    line.className = 'log-line level-' + safeLevel;

    var timeEl = document.createElement('span');
    timeEl.className = 'log-time';
    timeEl.textContent = timestamp;

    var tag = document.createElement('span');
    tag.className = 'log-level ' + safeLevel;
    tag.textContent = levelTextMap[safeLevel];

    var text = document.createElement('span');
    text.className = 'log-text';
    text.textContent = message;

    line.appendChild(timeEl);
    line.appendChild(tag);
    line.appendChild(text);
    container.prepend(line);

    // Keep max 50 log lines
    while (container.children.length > 50) {
        container.removeChild(container.lastElementChild);
    }
}

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
