use serde::{Deserialize, Serialize};

use crate::{
    certificate::Certificate,
    local_balance_tree::BalanceTree,
    local_exit_tree::{hasher::Keccak256Hasher, LocalExitTree},
    proof::{BalanceRoot, ExitRoot},
    ProofError,
};

/// Local state of one network.
/// The AggLayer tracks the [`LocalNetworkState`] for all networks.
/// Eventually, this state will be entirely tracked by the networks themselves.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocalNetworkState {
    /// Commitment to the [`BridgeExit`].
    pub exit_tree: LocalExitTree<Keccak256Hasher>,
    /// Commitment to the balance for each token.
    pub balance_tree: BalanceTree,
}

impl LocalNetworkState {
    /// Apply the [`Certificate`] on the current [`State`].
    /// Returns the commitment on the resulting state if successful.
    pub fn apply_certificate(
        &mut self,
        certificate: &Certificate,
    ) -> Result<(ExitRoot, BalanceRoot), ProofError> {
        // Check the initial state
        let computed_root = self.exit_tree.get_root();
        if computed_root != certificate.prev_local_exit_root {
            return Err(ProofError::InvalidLocalExitRoot {
                got: computed_root,
                expected: certificate.prev_local_exit_root,
            });
        }

        // Apply the bridge exits
        certificate.bridge_exits.iter().for_each(|bridge_exit| {
            self.exit_tree.add_leaf(bridge_exit.hash());
            self.balance_tree.withdraw(bridge_exit.token_info, bridge_exit.amount);
        });

        // Check whether the origin network has some debt
        if self.balance_tree.has_debt() {
            return Err(ProofError::HasDebt {
                network: certificate.origin_network,
            });
        }

        Ok((self.exit_tree.get_root(), self.balance_tree.hash()))
    }
}
