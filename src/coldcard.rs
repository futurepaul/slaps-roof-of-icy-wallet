// use anyhow::Result;
use std::str::FromStr;

use bdk::bitcoin::{
    util::bip32::{ChildNumber, DerivationPath, ExtendedPubKey},
    Network,
};
use bdk::{descriptor::Descriptor, miniscript::DescriptorPublicKey};

use serde::Deserialize;

// use crate::{util, Desc};

#[derive(Clone, Debug, Deserialize)]
pub struct Bip84Json {
    _pub: String,
    pub deriv: String,
    pub first: String,
    name: String,
    xfp: String,
    pub xpub: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ColdcardJson {
    pub chain: String,
    pub xfp: String,
    xpub: String,
    // TODO: use the account, yes?
    account: u64,
    // TODO: use other address types?
    pub bip84: Bip84Json,
}

impl ColdcardJson {
    pub fn get_network(&self) -> Result<Network, bdk::Error> {
        // TODO: figure out what coldcard's regest and signet flags are
        let network = match &self.chain[..] {
            "XTN" => Network::Testnet,
            "BTC" => Network::Bitcoin,
            _ => panic!("Didn't expect that network"),
        };

        Ok(network)
    }

    pub fn build_descriptor(
        &self,
        is_change: bool,
    ) -> Result<Descriptor<DescriptorPublicKey>, bdk::Error> {
        if self.get_network()? != Network::Testnet {
            panic!("We only support tpub right now")
        }

        let change_path: DerivationPath = vec![ChildNumber::Normal {
            index: is_change as u32,
        }]
        .into();

        let derivation_path = DerivationPath::from_str(&self.bip84.deriv)?;

        // TODO: AM I DOING THIS RIGHT?
        let _path = derivation_path.extend(change_path.clone());

        let xpub = ExtendedPubKey::from_str(&self.bip84.xpub)?;

        let (desc, _, _) = bdk::descriptor!(wpkh((xpub, change_path)))?;

        Ok(desc)
    }

    //     pub fn get_first_addresss(&self) -> Result<Address> {
    //         let address = Address::from_str(&self.bip84.first)?;
    //         Ok(address)
    //     }
}

impl FromStr for ColdcardJson {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}
