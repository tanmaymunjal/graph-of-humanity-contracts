use crate::constants::UBI_USERS_PER_ACC;
use crate::error::GraphOfHumanityErrors;
use crate::state::{DistributionEpoch, Treasury, UBIRandomnessAccount};
use anchor_lang::prelude::*;
use switchboard_on_demand::accounts::RandomnessAccountData;

#[derive(Accounts)]
#[instruction(randomness_id: String)]
pub struct RequestRandomnessUBI<'info> {
    #[account(mut)]
    pub cranker: Signer<'info>,
    /// CHECK: The account's data is validated manually within the handler.
    pub randomness_account_data: AccountInfo<'info>,
    #[account(
        seeds = [
            b"treasury"
        ],
        bump=treasury.bump,
        constraint = treasury.distribution_active == true @GraphOfHumanityErrors::NoEpochStarted
    )]
    pub treasury: Account<'info, Treasury>,
    #[account(
        mut,
        seeds = [
            &treasury.distributions.to_le_bytes(),
            b"di_epoch",
        ],
        bump=epoch.bump
    )]
    pub epoch: Account<'info, DistributionEpoch>,
    #[account(
        init_if_needed,
        payer = cranker,
        space = 8+UBIRandomnessAccount::INIT_SPACE,
        seeds = [
            epoch.key().as_ref(),
            randomness_id.as_bytes(),
            b"ubi_randomness_acc"
        ],
        bump,
        constraint = ubi_randomness_acc.accounts.len() <UBI_USERS_PER_ACC as usize @GraphOfHumanityErrors::RandomnessFull
    )]
    pub ubi_randomness_acc: Account<'info, UBIRandomnessAccount>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RequestRandomnessUBI>, _randomness_id: String) -> Result<()> {
    let randomness_account = &ctx.accounts.randomness_account_data;
    let epoch = &ctx.accounts.epoch;
    let ubi_randomness_acc = &mut ctx.accounts.ubi_randomness_acc;
    let clock = Clock::get()?;
    let randomness_data = RandomnessAccountData::parse(randomness_account.data.borrow()).unwrap();

    require!(
        randomness_data.seed_slot != clock.slot - 1,
        GraphOfHumanityErrors::RandomnessAlreadyRevealed
    );

    ubi_randomness_acc.bump = ctx.bumps.ubi_randomness_acc;
    ubi_randomness_acc.epoch = epoch.key();
    ubi_randomness_acc.randomness_account = randomness_account.key();
    ubi_randomness_acc.accounts = vec![];

    Ok(())
}
