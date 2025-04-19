// im/src-tauri/src/p2p/mod.rs

pub mod behaviour;
pub mod messaging;
pub mod peer;

use anyhow::Result;
use behaviour::{ChatBehaviour, ChatBehaviourEvent};
use futures::StreamExt;
use libp2p::{
    gossipsub, mdns, noise, swarm::SwarmEvent, tcp, yamux, Multiaddr, Swarm, SwarmBuilder,
};
// * 如果 messaging 模块将来继续增长，可以考虑将 P2PCommand 和 P2PEvent 拆分到各自模块中提升可维护性
use messaging::{P2PCommand, P2PEvent};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    time::Duration,
};
use tokio::{
    io, select,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
};

pub struct P2PNode {
    pub swarm: Swarm<ChatBehaviour>,
    pub command_sender: UnboundedSender<P2PCommand>,
    command_receiver: UnboundedReceiver<P2PCommand>,
    pub event_sender: UnboundedSender<P2PEvent>,
}

impl P2PNode {
    pub async fn new(
        listen_addr: Multiaddr,
        event_sender: UnboundedSender<P2PEvent>,
    ) -> Result<Self> {
        let (command_sender, command_receiver) = unbounded_channel();
        // * SwarmBuilder 中未提供自定义密钥对配置选项，当前仅适用于临时网络或非身份绑定场景
        let mut swarm = SwarmBuilder::with_new_identity()
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
            swarm,
            command_sender,
            command_receiver,
            event_sender,
        })
    }

    pub fn command_sender(&self) -> UnboundedSender<P2PCommand> {
        self.command_sender.clone()
    }

    pub async fn run(mut self) -> Result<()> {
        let topic = gossipsub::IdentTopic::new("chat");
        self.swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

        loop {
            // * run 函数可拆分为 handle_command 与 handle_event 两个子函数以提升可读性
            select! {
                Some(command)=self.command_receiver.recv()=>{
                    match command{
                        P2PCommand::SubscribeTopic(topic) => {
                            println!("Subscribing to topic: {topic}");
                            let topic = gossipsub::IdentTopic::new(topic);
                            // * 每次收到 P2PCommand::SubscribeTopic 都会重新构造 IdentTopic，建议缓存 topic 对象
                            self.swarm.behaviour_mut().gossipsub.subscribe(&topic)?;
                        }
                        P2PCommand::PublishMessage { topic, message } => {
                            println!("Publishing message: {message} to topic: {topic}");
                            let topic = gossipsub::IdentTopic::new(topic);
                            self.swarm.behaviour_mut().gossipsub.publish(topic,message.as_bytes())?;
                        }
                    }
                }
                event = self.swarm.select_next_some()=>{
                    match event {
                        SwarmEvent::Behaviour(ChatBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                            for (peer_id, _multiaddr) in list {
                                println!("mDNS discovered a new peer: {peer_id}");
                                self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);

                            }
                        },
                        SwarmEvent::Behaviour(ChatBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                            for (peer_id, _multiaddr) in list {
                                println!("mDNS discover peer has expired: {peer_id}");
                                self.swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                            }
                        },
                        SwarmEvent::Behaviour(ChatBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                            propagation_source: _,
                            message_id: _,
                            message,
                        })) => {
                            if let Ok(data) = String::from_utf8(message.data.clone()) {
                                println!("Received message: {data}");
                                // 将消息发送到前端
                                self.event_sender.send(P2PEvent::MessageReceived {
                                    topic: message.topic.to_string(),
                                    data: data,
                                }).unwrap_or_else(|e| eprintln!("事件发送失败: {}", e));
                            } else {
                                println!("Received invalid UTF-8 message");
                            }
                        }
                        SwarmEvent::NewListenAddr { address, .. } => {
                            println!("Local node is listening on {address}");
                        }
                        _ => {}
                    }
                }

            }
        }
    }
}
