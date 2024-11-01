//! Chain specification for the Base Mainnet network.

use alloc::{sync::Arc, vec};

use alloy_chains::Chain;
use alloy_primitives::{b256, U256};
use reth_chainspec::{once_cell_set, BaseFeeParams, BaseFeeParamsKind, ChainSpec};
use reth_ethereum_forks::EthereumHardfork;
use reth_optimism_forks::OptimismHardfork;

use crate::{LazyLock, OpChainSpec};

/// The Base mainnet spec
pub static BLAST_MAINNET: LazyLock<Arc<OpChainSpec>> = LazyLock::new(|| {
    OpChainSpec {
        inner: ChainSpec {
            chain: Chain::base_mainnet(),
            genesis: serde_json::from_str(include_str!("../res/genesis/blast.json"))
                .expect("Can't deserialize Base genesis json"),
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
        },
    }
    .into()
});
