// src/p2p/mod.rs
use anyhow::Result;
// use behaviour::{build_behaviour, ChatBehaviour};
use futures::StreamExt;
use libp2p::{
    gossipsub, identity::Keypair, mdns, noise, swarm::NetworkBehaviour, tcp, yamux, Multiaddr,
    Swarm, SwarmBuilder,
};
use messaging::{P2PCommand, P2PMessage};
use peer::build_peer;
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    time::Duration,
};
use tokio::{
    io,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
};

// pub mod behaviour;
pub mod messaging;
pub mod peer;

pub struct P2PNode {
    pub peer_id: libp2p::PeerId,
    pub listen_addr: Multiaddr,
    pub swarm: Swarm<ChatBehaviour>,
    pub command_sender: UnboundedSender<P2PCommand>,
    pub message_receiver: UnboundedReceiver<P2PMessage>,
}
// We create a custom network behaviour that combines Gossipsub and Mdns.
#[derive(NetworkBehaviour)]
struct ChatBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

impl P2PNode {
    pub async fn new(listen_addr: Multiaddr) -> Result<Self> {
        let (peer_id, keypair) = build_peer();
        // let (behaviour, message_receiver, command_sender) = build_behaviour(&keypair).await?;
        let (_, message_receiver) = unbounded_channel();
        let (command_sender, _) = unbounded_channel();

        let mut swarm = SwarmBuilder::with_existing_identity(keypair)
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_quic()
            .with_behaviour(|key| {
                // To content-address message, we can take the hash of message and use it as an ID.
                let message_id_fn = |message: &gossipsub::Message| {
                    let mut s = DefaultHasher::new();
                    message.data.hash(&mut s);
                    gossipsub::MessageId::from(s.finish().to_string())
                };

                // Set a custom gossipsub configuration
                let gossipsub_config = gossipsub::ConfigBuilder::default()
                    .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
                    .validation_mode(gossipsub::ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message
                    // signing)
                    .message_id_fn(message_id_fn) // content-address messages. No two messages of the same content will be propagated.
                    .build()
                    .map_err(io::Error::other)?; // Temporary hack because `build` does not return a proper `std::error::Error`.

                // build a gossipsub network behaviour
                let gossipsub = gossipsub::Behaviour::new(
                    gossipsub::MessageAuthenticity::Signed(key.clone()),
                    gossipsub_config,
                )?;

                let mdns = mdns::tokio::Behaviour::new(
                    mdns::Config::default(),
                    key.public().to_peer_id(),
                )?;
                Ok(ChatBehaviour { gossipsub, mdns })
            })?
            .build();
        swarm.listen_on(listen_addr.clone())?;
        Ok(Self {
            peer_id,
            listen_addr,
            swarm,
            command_sender,
            message_receiver,
        })
    }

    pub fn command_sender(&self) -> UnboundedSender<P2PCommand> {
        self.command_sender.clone()
    }

    pub fn message_receiver(&mut self) -> &mut UnboundedReceiver<P2PMessage> {
        &mut self.message_receiver
    }

    pub async fn run(mut self) -> Result<()> {
        loop {
            let event = self.swarm.select_next_some().await;
            println!("[Swarm Event] {:?}", event);
        }
    }
}
