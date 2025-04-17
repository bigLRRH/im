use std::{
    error::Error,
    hash::{DefaultHasher, Hash, Hasher},
    time::Duration,
};

use libp2p::{gossipsub, identity::Keypair, mdns, swarm::NetworkBehaviour};
use tokio::{
    io,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
};

use super::messaging::{P2PCommand, P2PMessage};

#[derive(NetworkBehaviour)]
pub struct ChatBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
}

// pub async fn build_behaviour(
//     keypair: &Keypair,
// ) -> Result<
//     (
//         ChatBehaviour,
//         UnboundedReceiver<P2PMessage>,
//         UnboundedSender<P2PCommand>,
//     ),
//     Box<dyn Error>,
// > {
//     let (_msg_tx, msg_rx) = unbounded_channel();
//     let (cmd_tx, _cmd_rx) = unbounded_channel();
//     // To content-address message, we can take the hash of message and use it as an ID.
//     let message_id_fn = |message: &gossipsub::Message| {
//         let mut s = DefaultHasher::new();
//         message.data.hash(&mut s);
//         gossipsub::MessageId::from(s.finish().to_string())
//     };
//     // Set a custom gossipsub configuration
//     let gossipsub_config = gossipsub::ConfigBuilder::default()
//         .heartbeat_interval(Duration::from_secs(10))
//         .validation_mode(gossipsub::ValidationMode::Strict)
//         .message_id_fn(message_id_fn)
//         .build()
//         .map_err(io::Error::other)?;
//     // build a gossipsub network behaviour
//     let gossipsub = gossipsub::Behaviour::new(
//         gossipsub::MessageAuthenticity::Signed(keypair.clone()),
//         gossipsub_config,
//     )?;
//     let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), keypair.public().to_peer_id())?;

//     Ok((ChatBehaviour { gossipsub, mdns }, msg_rx, cmd_tx))
// }
