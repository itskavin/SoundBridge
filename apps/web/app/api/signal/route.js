import { NextResponse } from "next/server";
import { enqueue, pull, store } from "../../lib/signalStore";

export const dynamic = "force-dynamic";

function now() {
  return Date.now();
}

function randomId() {
  return Math.random().toString(36).slice(2, 10);
}

export async function GET(req) {
  const { searchParams } = new URL(req.url);
  const mode = searchParams.get("mode") || "peers";
  const peerId = searchParams.get("peerId") || "";

  const s = store();

  if (mode === "messages") {
    if (!peerId) {
      return NextResponse.json({ error: "peerId required" }, { status: 400 });
    }

    const messages = pull(peerId);
    return NextResponse.json({ messages });
  }

  const self = s.peers.get(peerId);
  const peers = [...s.peers.values()]
    .filter((p) => p.peerId !== peerId)
    .filter((p) => {
      if (!self) return true;
      return p.publicIp === self.publicIp;
    })
    .map((p) => ({
      peerId: p.peerId,
      deviceName: p.deviceName,
      role: p.role,
      publicIp: p.publicIp,
      lastSeenAt: p.lastSeenAt,
    }));

  return NextResponse.json({ peers });
}

export async function POST(req) {
  const body = await req.json();
  const type = body.type;
  const s = store();

  if (type === "register") {
    const peerId = body.peerId || randomId();
    const xff = req.headers.get("x-forwarded-for") || "";
    const ip = xff.split(",")[0].trim() || "unknown";

    s.peers.set(peerId, {
      peerId,
      deviceName: body.deviceName || `device-${peerId.slice(0, 4)}`,
      role: body.role || "listener",
      publicIp: ip,
      lastSeenAt: now(),
    });

    return NextResponse.json({ peerId, publicIp: ip });
  }

  if (type === "heartbeat") {
    const peer = s.peers.get(body.peerId);
    if (peer) {
      peer.lastSeenAt = now();
      peer.role = body.role || peer.role;
      peer.deviceName = body.deviceName || peer.deviceName;
      s.peers.set(body.peerId, peer);
    }
    return NextResponse.json({ ok: true });
  }

  if (type === "signal") {
    if (!body.toPeerId || !body.fromPeerId || !body.payload) {
      return NextResponse.json({ error: "fromPeerId, toPeerId, payload required" }, { status: 400 });
    }

    enqueue(body.toPeerId, {
      fromPeerId: body.fromPeerId,
      payload: body.payload,
      createdAt: now(),
    });

    return NextResponse.json({ ok: true });
  }

  return NextResponse.json({ error: "unsupported type" }, { status: 400 });
}

export async function DELETE(req) {
  const { searchParams } = new URL(req.url);
  const peerId = searchParams.get("peerId") || "";
  const s = store();

  if (peerId) {
    s.peers.delete(peerId);
    s.queues.delete(peerId);
  }

  return NextResponse.json({ ok: true });
}
