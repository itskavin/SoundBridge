let broadcasterPc = null;
let listenerPc = null;
let broadcasterStream = null;

const logEl = document.getElementById("log");
const remoteAudio = document.getElementById("remoteAudio");
const broadcasterState = document.getElementById("broadcasterState");
const listenerState = document.getElementById("listenerState");

const startBroadcasterBtn = document.getElementById("startBroadcaster");
const createAnswerBtn = document.getElementById("createAnswer");
const applyAnswerBtn = document.getElementById("applyAnswer");
const copyOfferBtn = document.getElementById("copyOffer");
const copyAnswerBtn = document.getElementById("copyAnswer");
const resetAllBtn = document.getElementById("resetAll");
const clearLogBtn = document.getElementById("clearLog");

const offerOut = document.getElementById("offerOut");
const offerIn = document.getElementById("offerIn");
const answerOut = document.getElementById("answerOut");
const answerIn = document.getElementById("answerIn");

startBroadcasterBtn.addEventListener("click", startBroadcaster);
createAnswerBtn.addEventListener("click", createListenerAnswer);
applyAnswerBtn.addEventListener("click", applyListenerAnswer);
copyOfferBtn.addEventListener("click", () => copyText(offerOut.value, "offer"));
copyAnswerBtn.addEventListener("click", () => copyText(answerOut.value, "answer"));
resetAllBtn.addEventListener("click", resetSession);
clearLogBtn.addEventListener("click", () => {
  logEl.textContent = "";
});

function log(message) {
  const ts = new Date().toISOString();
  logEl.textContent += `[${ts}] ${message}\n`;
  logEl.scrollTop = logEl.scrollHeight;
}

function updateState(target, state) {
  target.textContent = state;
}

function encodeSessionDescription(desc) {
  return btoa(unescape(encodeURIComponent(JSON.stringify(desc))));
}

function decodeSessionDescription(raw) {
  try {
    return JSON.parse(decodeURIComponent(escape(atob(raw.trim()))));
  } catch {
    throw new Error("invalid encoded session description");
  }
}

function createPeerConnection(label) {
  const pc = new RTCPeerConnection({
    iceServers: [{ urls: "stun:stun.l.google.com:19302" }],
  });

  pc.onconnectionstatechange = () => {
    log(`${label} connection state: ${pc.connectionState}`);
    if (label === "broadcaster") updateState(broadcasterState, pc.connectionState);
    if (label === "listener") updateState(listenerState, pc.connectionState);
  };

  pc.oniceconnectionstatechange = () => {
    log(`${label} ICE state: ${pc.iceConnectionState}`);
  };

  return pc;
}

function waitForIceComplete(pc) {
  if (pc.iceGatheringState === "complete") {
    return Promise.resolve();
  }

  return new Promise((resolve) => {
    const check = () => {
      if (pc.iceGatheringState === "complete") {
        pc.removeEventListener("icegatheringstatechange", check);
        resolve();
      }
    };
    pc.addEventListener("icegatheringstatechange", check);
  });
}

async function startBroadcaster() {
  try {
    if (broadcasterPc) {
      closePeer(broadcasterPc, "broadcaster");
      broadcasterPc = null;
    }
    broadcasterPc = createPeerConnection("broadcaster");

    broadcasterStream = await navigator.mediaDevices.getUserMedia({ audio: true, video: false });
    broadcasterStream
      .getTracks()
      .forEach((track) => broadcasterPc.addTrack(track, broadcasterStream));

    const offer = await broadcasterPc.createOffer();
    await broadcasterPc.setLocalDescription(offer);
    await waitForIceComplete(broadcasterPc);

    offerOut.value = encodeSessionDescription(broadcasterPc.localDescription);
    log("Broadcaster offer created. Send this to listener.");
  } catch (error) {
    log(`Broadcaster start failed: ${error.message}`);
  }
}

async function createListenerAnswer() {
  try {
    if (listenerPc) {
      closePeer(listenerPc, "listener");
      listenerPc = null;
    }

    const encodedOffer = offerIn.value;
    if (!encodedOffer.trim()) {
      log("Listener offer input is empty.");
      return;
    }

    const offer = decodeSessionDescription(encodedOffer);
    listenerPc = createPeerConnection("listener");

    listenerPc.ontrack = (event) => {
      const [stream] = event.streams;
      if (stream) {
        remoteAudio.srcObject = stream;
        log("Listener attached incoming audio stream.");
      }
    };

    await listenerPc.setRemoteDescription(offer);
    const answer = await listenerPc.createAnswer();
    await listenerPc.setLocalDescription(answer);
    await waitForIceComplete(listenerPc);

    answerOut.value = encodeSessionDescription(listenerPc.localDescription);
    log("Listener answer created. Send this to broadcaster.");
  } catch (error) {
    log(`Listener answer failed: ${error.message}`);
  }
}

async function applyListenerAnswer() {
  try {
    if (!broadcasterPc) {
      log("Broadcaster is not started yet.");
      return;
    }

    const encodedAnswer = answerIn.value;
    if (!encodedAnswer.trim()) {
      log("Broadcaster answer input is empty.");
      return;
    }

    const answer = decodeSessionDescription(encodedAnswer);
    await broadcasterPc.setRemoteDescription(answer);
    log("Broadcaster applied listener answer. Peer connection should establish shortly.");
  } catch (error) {
    log(`Applying listener answer failed: ${error.message}`);
  }
}

function closePeer(pc, label) {
  try {
    pc.getSenders().forEach((sender) => {
      if (sender.track) sender.track.stop();
    });
    pc.close();
    log(`${label} peer closed`);
  } catch (error) {
    log(`${label} close warning: ${error.message}`);
  }
}

function resetSession() {
  if (broadcasterPc) {
    closePeer(broadcasterPc, "broadcaster");
    broadcasterPc = null;
  }
  if (listenerPc) {
    closePeer(listenerPc, "listener");
    listenerPc = null;
  }

  if (broadcasterStream) {
    broadcasterStream.getTracks().forEach((track) => track.stop());
    broadcasterStream = null;
  }

  remoteAudio.srcObject = null;
  offerOut.value = "";
  offerIn.value = "";
  answerOut.value = "";
  answerIn.value = "";

  updateState(broadcasterState, "idle");
  updateState(listenerState, "idle");
  log("Session reset complete");
}

async function copyText(text, label) {
  if (!text.trim()) {
    log(`No ${label} to copy yet`);
    return;
  }

  try {
    await navigator.clipboard.writeText(text);
    log(`${label} copied to clipboard`);
  } catch (error) {
    log(`Copy failed (${label}): ${error.message}`);
  }
}
