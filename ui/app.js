const invoke = window.__TAURI__.invoke

var CPS;
var RAND;
var TKEY;

function bstart_clicked() {
    invoke('start', {
        cps: parseInt(CPS),
        rand: parseInt(RAND)
    })
}

function storeCPS() {
    CPS = document.getElementById("icps").value
}

function storeRand() {
    RAND = document.getElementById("irand").value
}

function storeToggleKey() {
    console.log("called");
    TKEY = document.getElementById("itogglekey").value
    if (typeof TKEY == "string") {
        invoke('bind_key', { key: TKEY }).then((message) => document.getElementById("itogglekey").value = message)
    }
}