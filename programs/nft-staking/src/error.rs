use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("maximum stake reached")]
    MaxStake,
    #[msg("nft is locked untill passed freeze period")]
    Locked,
}