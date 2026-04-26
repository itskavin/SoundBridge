"use client";

import { useEffect, useMemo, useRef, useState } from "react";

const ICE_SERVERS = [{ urls: "stun:stun.l.google.com:19302" }];

function randomName() {
  const suffix = Math.random().toString(36).slice(2, 6);
  return `device-${suffix}`;
}

export default function Home() {
  const [peerId, setPeerId] = useState("");
  const [deviceName, setDeviceName] = useState(randomName());
  const [role, setRole] = useState("listener");
  const [publicIp, setPublicIp] = useState("unknown");
  const [status, setStatus] = useState("idle");
  const [logs, setLogs] = useState([]);
  const [peers, setPeers] = useState([]);
  const [selectedPeerId, setSelectedPeerId] = useState("");

  const pcRef = useRef(null);
  const localStreamRef = useRef(null);
  const audioRef = useRef(null);

  const selectedPeer = useMemo(
    () => peers.find((p) => p.peerId === selectedPeerId) || null,
    [peers, selectedPeerId]
  );

  function log(msg) {
    const ts = new Date().toISOString();
    setLogs((prev) => [...prev.slice(-80), `[${ts}] ${msg}`]);
  }

  async function register() {
    const res = await fetch("/api/signal", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ type: "register", peerId, deviceName, role }),
    });
    const data = await res.json();

    if (data.peerId) {
      setPeerId(data.peerId);
      setPublicIp(data.publicIp || "unknown");
      log(`registered as ${deviceName} (${data.peerId})`);
    }
  }

  async function heartbeat() {
    if (!peerId) return;
    await fetch("/api/signal", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ type: "heartbeat", peerId, deviceName, role }),
    });
  }

  async function refreshPeers() {
    if (!peerId) return;
    const res = await fetch(`/api/signal?mode=peers&peerId=${peerId}`);
    const data = await res.json();
    setPeers(data.peers || []);
  }

  async function pollMessages() {
    if (!peerId) return;
    const res = await fetch(`/api/signal?mode=messages&peerId=${peerId}`);
    const data = await res.json();

    for (const msg of data.messages || []) {
      await onSignalMessage(msg);
    }
  }

  async function onSignalMessage(msg) {
    const payload = msg.payload;
    if (!payload) return;

    if (payload.type === "offer") {
      setStatus("offer-received");
      log(`offer received from ${msg.fromPeerId}`);
      await ensurePeerConnection();
      await pcRef.current.setRemoteDescription(payload.sdp);
      const answer = await pcRef.current.createAnswer();
      await pcRef.current.setLocalDescription(answer);
      await waitForIceComplete(pcRef.current);

      await sendSignal(msg.fromPeerId, {
        type: "answer",
        sdp: pcRef.current.localDescription,
      });
      setStatus("answer-sent");
      log("answer sent");
    }

    if (payload.type === "answer") {
      log(`answer received from ${msg.fromPeerId}`);
      if (!pcRef.current) return;
      await pcRef.current.setRemoteDescription(payload.sdp);
      setStatus("connected");
    }
  }

  async function sendSignal(toPeerId, payload) {
    await fetch("/api/signal", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({
        type: "signal",
        fromPeerId: peerId,
        toPeerId,
        payload,
      }),
    });
  }

  async function ensurePeerConnection() {
    if (pcRef.current) return;

    const pc = new RTCPeerConnection({ iceServers: ICE_SERVERS });
    pc.onconnectionstatechange = () => {
      setStatus(pc.connectionState || "connecting");
      log(`connection state -> ${pc.connectionState}`);
    };

    pc.ontrack = (event) => {
      const [stream] = event.streams;
      if (stream && audioRef.current) {
        audioRef.current.srcObject = stream;
        log("remote audio track received");
      }
    };

    pcRef.current = pc;
  }

  async function startCall() {
    if (!selectedPeerId) {
      log("select a peer first");
      return;
    }

    await ensurePeerConnection();
    setRole("broadcaster");

    const stream = await navigator.mediaDevices.getUserMedia({ audio: true, video: false });
    localStreamRef.current = stream;
    stream.getTracks().forEach((track) => pcRef.current.addTrack(track, stream));

    const offer = await pcRef.current.createOffer();
    await pcRef.current.setLocalDescription(offer);
    await waitForIceComplete(pcRef.current);

    await sendSignal(selectedPeerId, { type: "offer", sdp: pcRef.current.localDescription });
    setStatus("offer-sent");
    log(`offer sent to ${selectedPeerId}`);
  }

  function stopSession() {
    if (pcRef.current) {
      pcRef.current.close();
      pcRef.current = null;
    }
    if (localStreamRef.current) {
      localStreamRef.current.getTracks().forEach((t) => t.stop());
      localStreamRef.current = null;
    }
    if (audioRef.current) {
      audioRef.current.srcObject = null;
    }
    setStatus("idle");
    log("session stopped");
  }

  useEffect(() => {
    fetch("/api/network")
      .then((r) => r.json())
      .then((d) => setPublicIp(d.ip || "unknown"))
      .catch(() => setPublicIp("unknown"));
  }, []);

  useEffect(() => {
    register().catch((e) => log(`register failed: ${e.message}`));
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    if (!peerId) return;
    const t1 = setInterval(() => heartbeat().catch(() => {}), 4000);
    const t2 = setInterval(() => refreshPeers().catch(() => {}), 3000);
    const t3 = setInterval(() => pollMessages().catch(() => {}), 1200);

    return () => {
      clearInterval(t1);
      clearInterval(t2);
      clearInterval(t3);
    };
  }, [peerId, deviceName, role]);

  useEffect(() => {
    return () => {
      if (peerId) {
        fetch(`/api/signal?peerId=${peerId}`, { method: "DELETE" }).catch(() => {});
      }
      stopSession();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [peerId]);

  return (
    <main className="container">
      <h1>SoundBridge Web v1</h1>
      <p className="subtitle">
        Same-network auto discovery with browser device names and one-click onboarding.
      </p>

      <section className="card">
        <h2>My Device</h2>
        <label>
          Device name
          <input value={deviceName} onChange={(e) => setDeviceName(e.target.value)} />
        </label>
        <div className="meta">
          <span>Peer ID: {peerId || "registering..."}</span>
          <span>Network hint (public IP): {publicIp}</span>
          <span>Status: {status}</span>
        </div>
      </section>

      <section className="card">
        <h2>Nearby Peers (same network hint)</h2>
        <div className="peer-list">
          {peers.length === 0 && <div className="peer-row">No peers found yet.</div>}
          {peers.map((peer) => (
            <button
              key={peer.peerId}
              className={`peer-row ${selectedPeerId === peer.peerId ? "selected" : ""}`}
              onClick={() => setSelectedPeerId(peer.peerId)}
            >
              <span>{peer.deviceName}</span>
              <span>{peer.peerId}</span>
            </button>
          ))}
        </div>
        <div className="actions">
          <button onClick={startCall}>Start Call as Broadcaster</button>
          <button className="secondary" onClick={stopSession}>Stop Session</button>
          <button className="secondary" onClick={() => refreshPeers().catch(() => {})}>Refresh Peers</button>
        </div>
        <p className="hint">
          {selectedPeer
            ? `Selected peer: ${selectedPeer.deviceName}`
            : "Select a peer to start a call."}
        </p>
      </section>

      <section className="card">
        <h2>Receiver Audio</h2>
        <audio ref={audioRef} autoPlay controls />
      </section>

      <section className="card">
        <h2>Onboarding (for users)</h2>
        <ol>
          <li>Open the web app on two devices on same Wi-Fi.</li>
          <li>Set easy-to-recognize device names.</li>
          <li>Select the other device in Nearby Peers.</li>
          <li>Click Start Call as Broadcaster.</li>
          <li>Accept microphone permission when asked.</li>
        </ol>
      </section>

      <section className="card">
        <h2>Debug Log</h2>
        <pre>{logs.join("\n")}</pre>
      </section>
    </main>
  );
}

async function waitForIceComplete(pc) {
  if (pc.iceGatheringState === "complete") return;

  await new Promise((resolve) => {
    const onChange = () => {
      if (pc.iceGatheringState === "complete") {
        pc.removeEventListener("icegatheringstatechange", onChange);
        resolve();
      }
    };
    pc.addEventListener("icegatheringstatechange", onChange);
  });
}
