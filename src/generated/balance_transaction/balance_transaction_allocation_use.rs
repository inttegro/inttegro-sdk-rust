//! Public refund or payout allocation details.

use crate::BalanceTransactionAmount;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BalanceTransactionAllocationUse {
	pub id: String,
	pub amount: BalanceTransactionAmount,
}
