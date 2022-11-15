use crate::p2p::{self, MyBehaviour};
use colored::*;
use libp2p::{gossipsub::GossipsubMessage, Multiaddr};
use log::info;
use pea_block::Block;
use pea_stake::Stake;
use pea_transaction::Transaction;
use std::error::Error;
pub fn handler(behaviour: &mut MyBehaviour, message: GossipsubMessage) -> Result<(), Box<dyn Error>> {
    match message.topic.as_str() {
        "block" => {
            let block: Block = bincode::deserialize(&message.data)?;
            behaviour.blockchain.pending_blocks_push(block.clone())?;
            let hash = behaviour.blockchain.block_accept(&block);
            info!(
                "{} {} {}",
                "Accept".green(),
                behaviour.blockchain.tree.height(&block.previous_hash).to_string().yellow(),
                hex::encode(hash)
            );
        }
        "stake" => {
            let stake: Stake = bincode::deserialize(&message.data)?;
            behaviour.blockchain.pending_stakes_push(stake)?;
        }
        "transaction" => {
            let transaction: Transaction = bincode::deserialize(&message.data)?;
            behaviour.blockchain.pending_transactions_push(transaction)?;
        }
        "multiaddr" => {
            let multiaddr: Multiaddr = bincode::deserialize(&message.data)?;
            if let Some(multiaddr) = p2p::multiaddr_ip(multiaddr) {
                behaviour.new_multiaddrs.insert(multiaddr.clone());
                info!("{} {} {}", "Multiaddr".cyan(), behaviour.new_multiaddrs.len().to_string().yellow(), multiaddr.to_string().magenta());
            }
        }
        _ => {}
    };
    Ok(())
}
