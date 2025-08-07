mod block;
mod blockchain;
mod wallet;
mod p2p;

use block::{Block, BlockHeader, Transaction};
use blockchain::Blockchain;
use wallet::Wallet;
use p2p::{ChainBehaviour, ChainBehaviourEvent, ChainMessage, CHAIN_TOPIC};

use libp2p::{
    identity, PeerId, Swarm,
    swarm::{SwarmBuilder, SwarmEvent},
    gossipsub, mdns, tcp,
};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use tokio::io::{self, AsyncBufReadExt};

const CHAIN_FILE: &str = "chain.json";

#[tokio::main]
async fn main() {
    println!("--- Kosher Chain: Phase 4 ---");

    // --- 1. Setup Wallets and Blockchain State ---
    let rabbi_a_wallet = Wallet::new();
    let mut validator_set = HashSet::new();
    validator_set.insert(rabbi_a_wallet.public_key_hex());
    
    // Use Arc<Mutex> for safe shared access to the blockchain across async tasks
    let blockchain = Arc::new(Mutex::new(Blockchain::load_from_file(
        CHAIN_FILE,
        validator_set,
    )));

    // --- 2. Create a new Swarm for P2P networking ---
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    println!("Local peer ID: {}", peer_id);

    let transport = tcp::tokio::Transport::new(tcp::Config::default())
        .upgrade(libp2p::core::upgrade::Version::V1Lazy)
        .authenticate(libp2p::noise::Config::new(&id_keys).unwrap())
        .multiplex(libp2p::yamux::Config::default())
        .boxed();
    
    let gossipsub_config = gossipsub::Config::default();
    let mut gossipsub = gossipsub::Behaviour::new(
        gossipsub::MessageAuthenticity::Signed(id_keys),
        gossipsub_config,
    ).unwrap();
    gossipsub.subscribe(&CHAIN_TOPIC).unwrap();
    
    let mdns = mdns::Behaviour::new(mdns::Config::default(), peer_id).unwrap();
    
    let behaviour = ChainBehaviour { gossipsub, mdns };

    let mut swarm = SwarmBuilder::with_tokio_executor(transport, behaviour, peer_id).build();
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();
    
    // --- 3. Start a task to read user input for creating new blocks ---
    let bc_clone = Arc::clone(&blockchain);
    tokio::spawn(async move {
        let mut stdin = io::BufReader::new(io::stdin()).lines();
        loop {
            if let Ok(Some(line)) = stdin.next_line().await {
                if line == "create" {
                    println!("Creating a new block...");
                    let new_tx = vec![Transaction {
                        sender: "system".into(),
                        recipient: "p2p_node".into(),
                        amount: 1.0,
                    }];
                    
                    let mut chain = bc_clone.lock().unwrap();
                    let prev_block = chain.blocks.last().unwrap();
                    
                    let mut new_block = Block {
                        header: BlockHeader {
                            id: prev_block.header.id + 1,
                            timestamp: chrono::Utc::now().timestamp(),
                            previous_hash: prev_block.calculate_header_hash(),
                            validator_pubkey: rabbi_a_wallet.public_key_hex(),
                            transactions_hash: Block::hash_transactions(&new_tx),
                        },
                        transactions: new_tx,
                        signature: libp2p::identity::ed25519::Signature::new([0; 64]), // Dummy
                    };
                    
                    let hash = new_block.calculate_header_hash();
                    // In a real PoA system, the wallet would be managed securely.
                    // new_block.signature = rabbi_a_wallet.sign(hash.as_bytes()); // This needs type conversion
                    
                    println!("New block created. It will be gossiped to the network.");
                    
                    // Announce the block via gossipsub
                    if let Err(e) = swarm.behaviour_mut().gossipsub.publish(
                        CHAIN_TOPIC.clone(),
                        serde_json::to_string(&ChainMessage::Block(new_block.clone())).unwrap()
                    ) {
                        eprintln!("Error publishing block: {:?}", e);
                    }
                    
                    // Add the block to our own chain
                    // For simplicity, we skip the signing and validation for now
                    chain.blocks.push(new_block);

                }
            }
        }
    });


    // --- 4. Main network event loop ---
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::Behaviour(ChainBehaviourEvent::Mdns(event)) => match event {
                libp2p::mdns::Event::Discovered(list) => {
                    for (peer_id, _multiaddr) in list {
                        println!("mDNS discovered a new peer: {}", peer_id);
                        swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                    }
                }
                libp2p::mdns::Event::Expired(list) => {
                    for (peer_id, _multiaddr) in list {
                        println!("mDNS peer has expired: {}", peer_id);
                        swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                    }
                }
            },
            SwarmEvent::Behaviour(ChainBehaviourEvent::Gossipsub(event)) => {
                if let gossipsub::Event::Message { message, .. } = event {
                    if let Ok(msg) = serde_json::from_slice::<ChainMessage>(&message.data) {
                        match msg {
                            ChainMessage::Block(block) => {
                                println!("Received new block #{} from a peer.", block.header.id);
                                let mut chain = blockchain.lock().unwrap();
                                // In a real implementation, you would run the full
                                // is_block_valid check from Phase 3 here.
                                if block.header.previous_hash == chain.blocks.last().unwrap().calculate_header_hash() {
                                    println!("✅ Block is valid. Adding to our chain.");
                                    chain.blocks.push(block);
                                } else {
                                    println!("❌ Block is invalid or for a different fork.");
                                }
                            }
                        }
                    }
                }
            }
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Node listening on {:?}", address);
            }
            _ => {}
        }
    }
}    let mut rogue_block = Block {
        header: BlockHeader {
            id: previous_block.header.id + 1,
            timestamp: chrono::Utc::now().timestamp(),
            previous_hash: previous_block.calculate_header_hash(),
            validator_pubkey: unauthorized_wallet.public_key_hex(),
            transactions_hash: Block::hash_transactions(&rogue_transactions),
        },
        transactions: rogue_transactions,
        signature: Signature::from_bytes(&[0; 64]).unwrap(),
    };
    let rogue_hash = rogue_block.calculate_header_hash();
    rogue_block.signature = unauthorized_wallet.sign(rogue_hash.as_bytes());

    match kosher_chain.add_block(rogue_block) {
        Ok(_) => println!("✅ SUCCESS: Block added by rogue actor."),
        Err(e) => eprintln!("❌ FAILURE: {}. Block was correctly rejected.", e),
    }

    println!("\nFinal block count: {}", kosher_chain.blocks.len());

    // Save the updated chain
    kosher_chain.save_to_file(CHAIN_FILE).unwrap();
}
