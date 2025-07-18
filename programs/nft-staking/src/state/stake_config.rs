use anchor_lang::prelude::*;

//this state contains all info of top level stake configuration which is initialized by admin

#[account]
#[derive(InitSpace)]
pub struct StakeConfig{
    pub points_per_stake:u8,
    pub max_stake:u8,
    pub freeze_period:u32,
    pub bump:u8,
    pub reward_bump:u8,
}
