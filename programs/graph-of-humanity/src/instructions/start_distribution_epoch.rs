use crate::error::GraphOfHumanityErrors;
use crate::event::DistributionEpochStarted;
use crate::state::{DistributionEpoch, Treasury};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint::USDC,
    token_interface::{transfer, Mint, TokenAccount, TokenInterface, Transfer},
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
            treasury.distributions.to_string().as_bytes(),
            b"di_epoch",
        ],
        bump
    )]
    pub epoch: Account<'info, DistributionEpoch>,
    #[account(
        token::mint = usdc_mint,
        token::authority = treasury,
        token::token_program=token_program
    )]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,
    // #[account(address=USDC)]
    pub usdc_mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler(ctx: Context<StartDistribution>) -> Result<()> {
    let treasury_token_account = &ctx.accounts.treasury_token_account;
    let treasury = &mut ctx.accounts.treasury;
    let epoch = &mut ctx.accounts.epoch;
    let usdc_mint = &ctx.accounts.usdc_mint;

    let base_amount = treasury.num_of_citizens * 1000 * 10u64.pow(usdc_mint.decimals as u32);
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
