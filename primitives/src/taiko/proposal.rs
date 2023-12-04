use alloy_sol_types::{sol, SolCall};
use anyhow::{Context, Result};

sol! {
    function proposeBlock(
        bytes calldata input,
        bytes calldata assignment,
        bytes calldata txList
    )
    {}
}

pub fn decode_propose_block_call_args(data: &[u8]) -> Result<proposeBlockCall> {
    let propose_block_call = proposeBlockCall::abi_decode(&data, false)
        .context("failed to decode propose block call")?;
    Ok(propose_block_call)
}
