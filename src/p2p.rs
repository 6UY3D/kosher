use libp2p::{
    gossipsub::{self, IdentTopic as Topic},
    mdns::tokio::Behaviour as Mdns,
    swarm::NetworkBehaviour,
    PeerId,
};
use serde::{Deserialize, Serialize};
use crate::block::Block;

// The messages we'll send across the network.
#[derive(Debug, Serialize, Deserialize)]
pub enum ChainMessage {
    Block(Block), // Announce a new block to the network
}

// The topic for our gossipsub protocol. All nodes must subscribe to the same topic.
pub const CHAIN_TOPIC: Topic = Topic::new("kosher-chain-blocks");

// This is the main network behaviour struct that combines all P2P protocols.
#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "ChainBehaviourEvent")]
pub struct ChainBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: Mdns,
}

// Events that our ChainBehaviour can produce.
#[derive(Debug)]
pub enum ChainBehaviourEvent {
    Gossipsub(gossipsub::Event),
    Mdns(libp2p::mdns::Event),
}

impl From<gossipsub::Event> for ChainBehaviourEvent {
    fn from(event: gossipsub::Event) -> Self {
        ChainBehaviourEvent::Gossipsub(event)
    }
}

impl From<libp2p::mdns::Event> for ChainBehaviourEvent {
    fn from(event: libp2p::mdns::Event) -> Self {
        ChainBehaviourEvent::Mdns(event)
    }
}
