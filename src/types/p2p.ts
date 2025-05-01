// src/types/p2p.ts

export type P2PCommand =
  | { type: 'subscribeTopic'; topic: string }
  | { type: 'publishMessage'; topic: string; message: string };

export type P2PEvent =
  | { type: 'messageReceived'; topic: string; data: string }
  | { type: 'peerDiscovered'; peer_id: string; multiaddr: string };
