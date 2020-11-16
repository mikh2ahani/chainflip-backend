use std::{fmt::Display, str::FromStr};

use crate::transactions::{
    OutputSentTx, OutputTx, PoolChangeTx, QuoteTx, StakeQuoteTx, StakeTx, UnstakeRequestTx,
    UnstakeTx, WitnessTx,
};

#[derive(Debug, Eq, PartialEq)]
pub enum TransactionType {
    PoolChange,
    SwapQuote,
    StakeQuote,
    Witness,
    Stake,
    Output,
    Sent,
    UnstakeRequest,
    Unstake,
}

impl Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Invalid coin literal error
pub const PARSING_ERROR: &'static str = "failed to parse transaction type";

impl FromStr for TransactionType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PoolChange" => Ok(TransactionType::PoolChange),
            "SwapQuote" => Ok(TransactionType::SwapQuote),
            "StakeQuote" => Ok(TransactionType::StakeQuote),
            "Witness" => Ok(TransactionType::Witness),
            "Stake" => Ok(TransactionType::Stake),
            "Output" => Ok(TransactionType::Output),
            "Sent" => Ok(TransactionType::Sent),
            "Unstake" => Ok(TransactionType::Unstake),
            "UnstakeRequest" => Ok(TransactionType::UnstakeRequest),
            _ => Err(PARSING_ERROR),
        }
    }
}

impl From<&PoolChangeTx> for TransactionType {
    fn from(_: &PoolChangeTx) -> Self {
        TransactionType::PoolChange
    }
}

impl From<&QuoteTx> for TransactionType {
    fn from(_: &QuoteTx) -> Self {
        TransactionType::SwapQuote
    }
}

impl From<&StakeQuoteTx> for TransactionType {
    fn from(_: &StakeQuoteTx) -> Self {
        TransactionType::StakeQuote
    }
}

impl From<&StakeTx> for TransactionType {
    fn from(_: &StakeTx) -> Self {
        TransactionType::Stake
    }
}

impl From<&WitnessTx> for TransactionType {
    fn from(_: &WitnessTx) -> Self {
        TransactionType::Witness
    }
}

impl From<&OutputTx> for TransactionType {
    fn from(_: &OutputTx) -> Self {
        TransactionType::Output
    }
}

impl From<&OutputSentTx> for TransactionType {
    fn from(_: &OutputSentTx) -> Self {
        TransactionType::Sent
    }
}

impl From<&UnstakeRequestTx> for TransactionType {
    fn from(_: &UnstakeRequestTx) -> Self {
        TransactionType::UnstakeRequest
    }
}

impl From<&UnstakeTx> for TransactionType {
    fn from(_: &UnstakeTx) -> Self {
        TransactionType::Unstake
    }
}
