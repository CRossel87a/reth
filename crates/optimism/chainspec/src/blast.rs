//! Chain specification for the Base Mainnet network.

use alloc::{sync::Arc, vec};

use alloy_chains::{Chain, NamedChain};
use alloy_primitives::{address, b256, Address, Bytes, B256, U256};
use reth_chainspec::{once_cell_set, BaseFeeParams, BaseFeeParamsKind, ChainSpec};
use reth_ethereum_forks::EthereumHardfork;
use reth_optimism_forks::OptimismHardfork;
use alloy_serde::storage::deserialize_storage_map;
use alloy_genesis::ChainConfig;

use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

use crate::{LazyLock, OpChainSpec};

pub const BLAST_SHARES_ADDRESS: Address = address!("4300000000000000000000000000000000000000");
pub const SHARE_PRICE_SLOT: B256 = b256!("0000000000000000000000000000000000000000000000000000000000000001");
pub const SHARE_COUNT_SLOT: B256 = b256!("0000000000000000000000000000000000000000000000000000000000000033");

//BlastGasAddress                  = common.HexToAddress("0x4300000000000000000000000000000000000001")
//BlastAccountConfigurationAddress = common.HexToAddress("0x4300000000000000000000000000000000000002")

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct BlastGenesisAccount {
    /// The nonce of the account at genesis.
    #[serde(skip_serializing_if = "Option::is_none", with = "alloy_serde::quantity::opt", default)]
    pub nonce: Option<u64>,
    /// The balance of the account at genesis.
    pub balance: U256,
    /// The account's bytecode at genesis.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<Bytes>,
    /// The account's storage at genesis.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_storage_map"
    )]
    pub storage: Option<BTreeMap<B256, B256>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<u8>
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
struct BlastGenesis {
    /// The fork configuration for this network.
    #[serde(default)]
    pub config: ChainConfig,
    /// The genesis header nonce.
    #[serde(with = "alloy_serde::quantity")]
    pub nonce: u64,
    /// The genesis header timestamp.
    #[serde(with = "alloy_serde::quantity")]
    pub timestamp: u64,
    /// The genesis header extra data.
    pub extra_data: Bytes,
    /// The genesis header gas limit.
    #[serde(with = "alloy_serde::quantity")]
    pub gas_limit: u64,
    /// The genesis header difficulty.
    pub difficulty: U256,
    /// The genesis header mix hash.
    pub mix_hash: B256,
    /// The genesis header coinbase address.
    pub coinbase: Address,
    /// The initial state of accounts in the genesis block.
    pub alloc: BTreeMap<Address, BlastGenesisAccount>,
    // NOTE: the following fields:
    // * base_fee_per_gas
    // * excess_blob_gas
    // * blob_gas_used
    // * number
    // should NOT be set in a real genesis file, but are included here for compatibility with
    // consensus tests, which have genesis files with these fields populated.
    /// The genesis header base fee
    #[serde(default, skip_serializing_if = "Option::is_none", with = "alloy_serde::quantity::opt")]
    pub base_fee_per_gas: Option<u128>,
    /// The genesis header excess blob gas
    #[serde(default, skip_serializing_if = "Option::is_none", with = "alloy_serde::quantity::opt")]
    pub excess_blob_gas: Option<u128>,
    /// The genesis header blob gas used
    #[serde(default, skip_serializing_if = "Option::is_none", with = "alloy_serde::quantity::opt")]
    pub blob_gas_used: Option<u128>,
    /// The genesis block number
    #[serde(default, skip_serializing_if = "Option::is_none", with = "alloy_serde::quantity::opt")]
    pub number: Option<u64>,
}

/// The Base mainnet spec
pub static BLAST_MAINNET: LazyLock<Arc<OpChainSpec>> = LazyLock::new(|| {


    let genesis_str = include_str!("../res/genesis/blast.json");

    let genesis: BlastGenesis = serde_json::from_str(&genesis_str).expect("Can't deserialize Blast genesis json");

    let bs = genesis.alloc.get(&BLAST_SHARES_ADDRESS).expect("expect account");
    let share_price_hash = bs.storage.as_ref().expect("expect storage").get(&SHARE_PRICE_SLOT).expect("expect share price slot");
    let share_price = U256::from_be_bytes(share_price_hash.0);


    let chainspec = ChainSpec {
        chain: Chain::from_named(NamedChain::Blast),
        genesis: serde_json::from_str(genesis_str)
            .expect("Can't deserialize Blast genesis json"),
        genesis_hash: once_cell_set(b256!(
            "b689b35ef29d0bec5816938e0e52683c7257d2e325420ea69b739a2be4754b89"
        )),
        paris_block_and_final_difficulty: Some((0, U256::from(0))),
        hardforks: OptimismHardfork::blast_mainnet(),
        base_fee_params: BaseFeeParamsKind::Variable(
            vec![
                (EthereumHardfork::London.boxed(), BaseFeeParams::optimism()),
                (OptimismHardfork::Canyon.boxed(), BaseFeeParams::optimism_canyon()),
            ]
            .into(),
        ),
        max_gas_limit: crate::constants::BASE_MAINNET_MAX_GAS_LIMIT,
        prune_delete_limit: 10000,
        ..Default::default()
    };

    OpChainSpec {
        inner: chainspec,
    }
    .into()
});


#[cfg(test)]
mod tests {
    use alloy_primitives::{B256, U256};

    use crate::blast::SHARE_PRICE_SLOT;

    use super::{BlastGenesis, BLAST_SHARES_ADDRESS};


    
    #[test]
    fn test_load_blast_genesis() {

        let genesis_str = include_str!("../res/genesis/blast.json");

        let genesis: BlastGenesis = serde_json::from_str(&genesis_str).unwrap();


        let bs = genesis.alloc.get(&BLAST_SHARES_ADDRESS).unwrap();
        dbg!(&bs);


        let share_price_hash = bs.storage.as_ref().unwrap().get(&SHARE_PRICE_SLOT).unwrap();
        let share_price = U256::from_be_bytes(share_price_hash.0);
        dbg!(share_price);

        //dbg!(genesis);

    }
}