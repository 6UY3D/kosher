use libp2p::{
    gossipsub::{self, IdentTopic as Topic},
    mdns::tokio::Behaviour as Mdns,
    swarm::NetworkBehaviour,
    PeerId,
};
use serde::{Deserialize, Serialize};
use crate::block::{Block, Transaction};
use std::collections::HashMap;

// --- Messages and Topics remain the same ---
#[derive(Debug, Serialize, Deserialize)]
pub enum ChainMessage {
    Block(Block),
    Transaction(Transaction),
}

pub const CHAIN_TOPIC: Topic = Topic::new("kosher-chain-blocks");
pub const TRANSACTION_TOPIC: Topic = Topic::new("kosher-chain-transactions");

// --- NetworkBehaviour remains the same ---
#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "ChainBehaviourEvent")]
pub struct ChainBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: Mdns,
}

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


// --- New Peer Management Structures ---

const INITIAL_SCORE: i32 = 0;
const BAN_THRESHOLD: i32 = -100;

#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub score: i32,
}

impl Default for PeerInfo {
    fn default() -> Self {
        Self { score: INITIAL_SCORE }
    }
}

// Manages the reputation of all connected peers.
#[derive(Debug, Default)]
pub struct PeerManager {
    peers: HashMap<PeerId, PeerInfo>,
}

impl PeerManager {
    pub fn add_peer(&mut self, peer_id: PeerId) {
        self.peers.entry(peer_id).or_default();
    }
    
    pub fn remove_peer(&mut self, peer_id: &PeerId) {
        self.peers.remove(peer_id);
    }
    
    // Apply a positive score adjustment.
    pub fn reward_peer(&mut self, peer_id: &PeerId, points: i32) {
        if let Some(peer) = self.peers.get_mut(peer_id) {
            peer.score += points;
            println!("[PeerManager] Rewarded peer {}. New score: {}", peer_id, peer.score);
        }
    }
    
    // Apply a negative score adjustment and check if the peer should be banned.
    pub fn penalize_peer(&mut self, peer_id: &PeerId, points: i32) -> bool {
        if let Some(peer) = self.peers.get_mut(peer_id) {
            peer.score -= points;
            println!("[PeerManager] Penalized peer {}. New score: {}", peer_id, peer.score);
            if peer.score < BAN_THRESHOLD {
                println!("[PeerManager] 🚨 Peer {} has crossed the ban threshold!", peer_id);
                return true; // Indicates the peer should be banned
            }
        }
        false
    }
}
