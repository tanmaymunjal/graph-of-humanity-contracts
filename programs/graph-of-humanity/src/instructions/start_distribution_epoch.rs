use crate::error::GraphOfHumanityErrors;
use crate::event::DistributionEpochStarted;
use crate::state::{DistributionEpoch, Treasury};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct StartDistribution<'info> {
    #[account(mut)]
    pub cranker: Signer<'info>,
    #[account(
        mut,
        seeds = [
            b"treasury"
        ],
        bump=treasury.bump,
        constraint = treasury.distribution_active == false @GraphOfHumanityErrors::CanNotStartDistributionEpochWhenOneIsAlreadyRunning
    )]
    pub treasury: Account<'info, Treasury>,
    #[account(
        init,
        payer = cranker,
        space = 8 + DistributionEpoch::INIT_SPACE,
        seeds = [
            &treasury.distributions.to_le_bytes(),
            b"di_epoch",
        ],
        bump
    )]
    pub epoch: Account<'info, DistributionEpoch>,
    #[account(
        associated_token::mint = usdc_mint,
        associated_token::authority = treasury
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    // #[account(address=USDC)]
    pub usdc_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler(ctx: Context<StartDistribution>) -> Result<()> {
    let treasury_token_account = &ctx.accounts.treasury_token_account;
    let treasury = &mut ctx.accounts.treasury;
    let epoch = &mut ctx.accounts.epoch;

    let base_amount = treasury.num_of_citizens * 1000;
    let ratio = treasury_token_account.amount as f64 / base_amount as f64;

    let multiplier = if ratio < 0.2 {
        0.0
    } else if ratio > 1.1 {
        1.1
    } else {
        (ratio * 10.0).floor() / 10.0
    };

    require!(
        multiplier > 0.0,
        GraphOfHumanityErrors::NotEnoughFundsToStartEpoch
    );
    let num_of_users_to_distribute = (multiplier * (treasury.num_of_citizens as f64)) as u64;

    epoch.bump = ctx.bumps.epoch;
    epoch.num_of_users_distributed = 0;
    epoch.num_of_users_to_distribute = num_of_users_to_distribute;
    epoch.distribution_max_user_ind = treasury.num_of_citizens;

    treasury.distribution_active = true;

    emit!(DistributionEpochStarted {
        num_of_users_to_distribute,
        started: Clock::get()?.unix_timestamp
    });

    Ok(())
}
