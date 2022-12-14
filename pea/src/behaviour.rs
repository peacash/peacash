use libp2p::{
    autonat,
    gossipsub::{Gossipsub, GossipsubConfigBuilder, GossipsubEvent, MessageAuthenticity},
    identify, identity, mdns, ping,
    swarm::NetworkBehaviour,
};
use pea_core::*;
use std::error::Error;
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "OutEvent")]
pub struct Behaviour {
    pub mdns: mdns::tokio::Behaviour,
    pub identify: identify::Behaviour,
    pub gossipsub: Gossipsub,
    pub autonat: autonat::Behaviour,
}
impl Behaviour {
    pub async fn new(local_key: identity::Keypair) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            mdns: mdns::tokio::Behaviour::new(mdns::Config::default())?,
            identify: identify::Behaviour::new(identify::Config::new(PROTOCOL_VERSION.to_string(), local_key.public())),
            gossipsub: Gossipsub::new(MessageAuthenticity::Signed(local_key.clone()), GossipsubConfigBuilder::default().build()?)?,
            autonat: autonat::Behaviour::new(local_key.public().to_peer_id(), autonat::Config::default()),
        })
    }
}
#[derive(Debug)]
pub enum OutEvent {
    Gossipsub(GossipsubEvent),
    Mdns(mdns::Event),
    Ping(ping::Event),
    Identify(identify::Event),
    Autonat(autonat::Event),
}
impl From<mdns::Event> for OutEvent {
    fn from(v: mdns::Event) -> Self {
        Self::Mdns(v)
    }
}
impl From<GossipsubEvent> for OutEvent {
    fn from(v: GossipsubEvent) -> Self {
        Self::Gossipsub(v)
    }
}
impl From<identify::Event> for OutEvent {
    fn from(v: identify::Event) -> Self {
        Self::Identify(v)
    }
}
impl From<autonat::Event> for OutEvent {
    fn from(v: autonat::Event) -> Self {
        Self::Autonat(v)
    }
}
