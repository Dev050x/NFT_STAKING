use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata, MetadataAccount,
    },
    token::{approve, Approve, Mint, Token, TokenAccount},
    associated_token
};

use crate::error::ErrorCode;
use crate::{StakeAccount, StakeConfig, UserAccount};

#[derive(Accounts)]
pub struct Stake<'info> {
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
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        seeds = [b"config".as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, StakeConfig>,

    #[account(
        init,
        payer = user,
        seeds = [b"stake".as_ref() , mint.key().as_ref() , config.key().as_ref() ],
        bump ,
        space = 8 + StakeAccount::INIT_SPACE,
    )]
    pub stake_account: Account<'info, StakeAccount>,
    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self, bumps: &StakeBumps) -> Result<()> {
        require!(
            self.user_account.amount_staked < self.config.max_stake,
            ErrorCode::MaxStake
        );

        let cpicontext = CpiContext::new(
            self.token_program.to_account_info(),
            Approve {
                to: self.user_mint_ata.to_account_info(),
                delegate: self.stake_account.to_account_info(),
                authority: self.user.to_account_info(),
            },
        );

        approve(cpicontext, 1)?;

        FreezeDelegatedAccountCpi::new(
            &self.metadata_program,
            FreezeDelegatedAccountCpiAccounts {
                delegate: &self.stake_account.to_account_info(),
                token_account: &self.user_mint_ata.to_account_info(),
                edition: &self.edition.to_account_info(),
                mint: &self.mint.to_account_info(),
                token_program: &self.token_program.to_account_info(),
            },
        )
        .invoke()?;

        self.stake_account.set_inner(StakeAccount {
            owner: self.user.key(),
            mint: self.mint.key(),
            last_update: Clock::get()?.unix_timestamp,
            bump: bumps.stake_account,
        });

        self.user_account.amount_staked += 1;

        Ok(())
    }
}
