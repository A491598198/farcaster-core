//! Defines and implements all the traits for Monero

use monero::cryptonote::hash::Hash;
use monero::network::Network;
use monero::util::key::PrivateKey;
use monero::util::key::PublicKey;

use crate::blockchain::Blockchain;
use crate::role::{Accordant};
use crate::crypto::{Group};

#[derive(Clone, Copy)]
pub struct Monero;

impl Blockchain for Monero {
    /// Type for the traded asset unit
    type AssetUnit = u64;

    /// Type of the blockchain identifier
    type Id = String;

    /// Type of the chain identifier
    type ChainId = Network;

    /// Returns the blockchain identifier
    fn id(&self) -> String {
        String::from("xmr")
    }

    /// Returns the chain identifier
    fn chain_id(&self) -> Network {
        Network::Mainnet
    }

    /// Create a new Bitcoin blockchain
    fn new() -> Self {
        Monero {}
    }
}

pub struct Ed25519;

impl Group for Monero {
    type Group = Ed25519;
}

impl Accordant for Monero {
    type PrivateKey = PrivateKey;
    type PublicKey = PublicKey;
    type Commitment = Hash;
}
