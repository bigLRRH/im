// src/p2p/peer.rs
use libp2p::{identity, PeerId};

pub fn build_peer() -> (PeerId, identity::Keypair) {
    let keypair = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(keypair.public());
    (peer_id, keypair)
}