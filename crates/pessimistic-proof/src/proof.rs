use crate::{
    bridge_exit::NetworkId, batch_header::BatchHeader, keccak::Digest,
    local_state::LocalNetworkState,
};

/// Represents all errors that can occur while generating the proof.
#[derive(Debug)]
pub enum ProofError {
    InvalidLocalExitRoot { got: Digest, expected: Digest },
    HasDebt { network: NetworkId },
}

pub type ExitRoot = Digest;
pub type BalanceRoot = Digest;
pub type LeafProofOutput = (ExitRoot, BalanceRoot);

/// Proves that the given [`BatchHeader`] can be applied on the given [`LocalNetworkState`].
pub fn generate_leaf_proof(
    initial_network_state: LocalNetworkState,
    batch_header: &BatchHeader,
) -> Result<LeafProofOutput, ProofError> {
    let mut network_state = initial_network_state;

    let (new_exit_root, new_balance_root) = network_state.apply_batch_header(batch_header)?;

    Ok((new_exit_root, new_balance_root))
}
