let broadcasterPc = null;
let listenerPc = null;

const logEl = document.getElementById("log");
const remoteAudio = document.getElementById("remoteAudio");

const startBroadcasterBtn = document.getElementById("startBroadcaster");
const createAnswerBtn = document.getElementById("createAnswer");
const applyAnswerBtn = document.getElementById("applyAnswer");

const offerOut = document.getElementById("offerOut");
const offerIn = document.getElementById("offerIn");
const answerOut = document.getElementById("answerOut");
const answerIn = document.getElementById("answerIn");

startBroadcasterBtn.addEventListener("click", startBroadcaster);
createAnswerBtn.addEventListener("click", createListenerAnswer);
applyAnswerBtn.addEventListener("click", applyListenerAnswer);

function log(message) {
  const ts = new Date().toISOString();
  logEl.textContent += `[${ts}] ${message}\n`;
  logEl.scrollTop = logEl.scrollHeight;
}

function encodeSessionDescription(desc) {
  return btoa(unescape(encodeURIComponent(JSON.stringify(desc))));
}

function decodeSessionDescription(raw) {
  return JSON.parse(decodeURIComponent(escape(atob(raw.trim()))));
}

function createPeerConnection(label) {
  const pc = new RTCPeerConnection({
    iceServers: [{ urls: "stun:stun.l.google.com:19302" }],
  });

  pc.onconnectionstatechange = () => {
    log(`${label} connection state: ${pc.connectionState}`);
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
    broadcasterPc = createPeerConnection("broadcaster");

    const stream = await navigator.mediaDevices.getUserMedia({ audio: true, video: false });
    stream.getTracks().forEach((track) => broadcasterPc.addTrack(track, stream));

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
