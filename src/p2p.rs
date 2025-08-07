use crate::block::{Block, Transaction};
use crate::blockchain::Blockchain;
use crate::config::P2pConfig;
use crate::mempool::Mempool;

use libp2p::{
    identity, noise, yamux, PeerId, Swarm, tcp,
    gossipsub::{self, IdentTopic as Topic, MessageAuthenticity},
    mdns,
    swarm::{SwarmBuilder, SwarmEvent},
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;


#[derive(Debug, Serialize, Deserialize)]
pub enum ChainMessage {
    Block(Block),
    Transaction(Transaction),
}

pub const CHAIN_TOPIC: Topic = Topic::new("kosher-chain-blocks");
pub const TRANSACTION_TOPIC: Topic = Topic::new("kosher-chain-transactions");

#[derive(libp2p::NetworkBehaviour)]
#[behaviour(to_swarm = "ChainBehaviourEvent")]
pub struct ChainBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
}
//... Event wrappers from previous phases ...

// Peer Management structs (PeerInfo, PeerManager) remain the same

pub async fn run_p2p_network(
    config: P2pConfig,
    blockchain: Arc<Mutex<Blockchain>>,
    mempool: Arc<Mutex<Mempool>>,
    peer_manager: Arc<Mutex<PeerManager>>,
    mut p2p_rx: mpsc::Receiver<ChainMessage>,
) {
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    println!("[P2P] Local peer ID: {}", peer_id);

    let transport = tcp::tokio::Transport::new(tcp::Config::default())
        .upgrade(libp2p::core::upgrade::Version::V1Lazy)
        .authenticate(noise::Config::new(&id_keys).unwrap())
        .multiplex(yamux::Config::default())
        .boxed();

    let gossipsub_config = gossipsub::Config::default();
    let mut gossipsub = gossipsub::Behaviour::new(
        MessageAuthenticity::Signed(id_keys),
        gossipsub_config,
    ).unwrap();
    gossipsub.subscribe(&CHAIN_TOPIC).unwrap();
    gossipsub.subscribe(&TRANSACTION_TOPIC).unwrap();

    let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), peer_id).unwrap();
    let behaviour = ChainBehaviour { gossipsub, mdns };
    let mut swarm = SwarmBuilder::with_tokio_executor(transport, behaviour, peer_id).build();
    swarm.listen_on(config.listen_address.parse().unwrap()).unwrap();

    loop {
        tokio::select! {
            Some(msg_to_gossip) = p2p_rx.recv() => {
                let topic = match &msg_to_gossip {
                    ChainMessage::Block(_) => CHAIN_TOPIC.clone(),
                    ChainMessage::Transaction(_) => TRANSACTION_TOPIC.clone(),
                };
                if let Ok(json_msg) = serde_json::to_string(&msg_to_gossip) {
                    if let Err(e) = swarm.behaviour_mut().gossipsub.publish(topic, json_msg.as_bytes()) {
                        eprintln!("[P2P] Failed to publish message: {:?}", e);
                    }
                }
            }
            event = swarm.select_next_some() => match event {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("[P2P] Listening on {}", address);
                }
                SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                    println!("[P2P] Connection established with: {}", peer_id);
                    peer_manager.lock().unwrap().add_peer(peer_id);
                }
                SwarmEvent::ConnectionClosed { peer_id, .. } => {
                    println!("[P2P] Connection closed with: {}", peer_id);
                    peer_manager.lock().unwrap().remove_peer(&peer_id);
                }
                SwarmEvent::Behaviour(ChainBehaviourEvent::Gossipsub(
                    gossipsub::Event::Message { message, .. }
                )) => {
                    let source_peer = message.source.unwrap();
                    if let Ok(msg) = serde_json::from_slice::<ChainMessage>(&message.data) {
                        handle_gossip_message(msg, &blockchain, &mempool, &peer_manager, &source_peer, &mut swarm);
                    }
                }
                _ => {}
            }
        }
    }
}

fn handle_gossip_message(
    msg: ChainMessage,
    blockchain: &Arc<Mutex<Blockchain>>,
    mempool: &Arc<Mutex<Mempool>>,
    peer_manager: &Arc<Mutex<PeerManager>>,
    source_peer: &PeerId,
    swarm: &mut Swarm<ChainBehaviour>,
) {
    match msg {
        ChainMessage::Block(block) => {
            let mut chain = blockchain.lock().unwrap();
            if chain.validate_and_add_block(block).is_ok() {
                peer_manager.lock().unwrap().reward_peer(source_peer, 10);
            } else {
                if peer_manager.lock().unwrap().penalize_peer(source_peer, 50) {
                    swarm.ban_peer(*source_peer);
                }
            }
        }
        ChainMessage::Transaction(tx) => {
            let mut mempool = mempool.lock().unwrap();
            if mempool.add_transaction(tx).is_ok() {
                 peer_manager.lock().unwrap().reward_peer(source_peer, 1);
            } else {
                if peer_manager.lock().unwrap().penalize_peer(source_peer, 5) {
                    swarm.ban_peer(*source_peer);
                }
            }
        }
    }
}    peers: HashMap<PeerId, PeerInfo>,
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
