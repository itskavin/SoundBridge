const g = globalThis;

if (!g.__soundbridgeSignalStore) {
  g.__soundbridgeSignalStore = {
    peers: new Map(),
    queues: new Map(),
  };
}

export function store() {
  return g.__soundbridgeSignalStore;
}

export function enqueue(toPeerId, message) {
  const s = store();
  const queue = s.queues.get(toPeerId) || [];
  queue.push(message);
  s.queues.set(toPeerId, queue);
}

export function pull(peerId) {
  const s = store();
  const queue = s.queues.get(peerId) || [];
  s.queues.set(peerId, []);
  return queue;
}
