use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token, metadata::{
        mpl_token_metadata::instructions::{
            ThawDelegatedAccountCpi, ThawDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata, MetadataAccount,
    }, token::{Mint, Token, TokenAccount}
};

use crate::{StakeAccount, StakeConfig, UserAccount};
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>,
    pub collection: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user
    )]
    pub user_mint_ata: Account<'info, TokenAccount>,
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        seeds::program = metadata_program.key(),
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true,
        bump,
    )]
    pub metadata: Account<'info, MetadataAccount>,
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition",
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub edition: Account<'info, MasterEditionAccount>,

    #[account(
        mut,
        seeds = [b"user".as_ref() , user.key().as_ref()],
        bump = UserAccount.bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        seeds = [b"config".as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, StakeConfig>,

    #[account(
        mut,
        close = user,
        seeds = [b"stake".as_ref() , mint.key().as_ref() , config.key().as_ref() ],
        bump = stake_account.bump ,
    )]
    pub stake_account: Account<'info, StakeAccount>,
    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Unstake<'info>{
    pub fn unstake(&mut self) -> Result<()> {
    
        let timePassed = ((Clock::get()?.unix_timestamp - self.stake_account.last_update)/86400) as u32;
        require!(timePassed > self.config.freeze_period , ErrorCode::Locked);
        self.user_account.points += timePassed * (self.config.points_per_stake as u32);

        ThawDelegatedAccountCpi::new(&&self.metadata_program, ThawDelegatedAccountCpiAccounts { 
            delegate:&self.stake_account.to_account_info(),
            token_account: &self.user_mint_ata.to_account_info(),
            edition: &self.edition.to_account_info(), 
            mint: &self.mint.to_account_info(), 
            token_program: &self.token_program.to_account_info()
        }).invoke()?;

        self.user_account.amount_staked -= 1;

        Ok(())
    }
}