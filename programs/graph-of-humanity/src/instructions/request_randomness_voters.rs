use crate::error::GraphOfHumanityErrors;
use crate::event::JudgeRandomnessRequested;
use crate::state::{CitizenshipApplication, Member, Treasury};
use crate::constants::NUM_OF_JUDGES;
use anchor_lang::prelude::*;
use switchboard_on_demand::accounts::RandomnessAccountData;

#[derive(Accounts)]
pub struct RequestRandomnessJudges<'info> {
    pub cranker: Signer<'info>,
    #[account(
        seeds = [
            cranker.key().as_ref(),
            b"member"
        ],
        bump=cranker_member.bump,
        constraint = cranker_member.citizen == true @GraphOfHumanityErrors::CanNotUseCrankAsANonCitizen
    )]
    pub cranker_member: Account<'info, Member>,
    #[account(
        mut,
        constraint = citizenship_appl.fee_paid == true && citizenship_appl.voucher_fee_paid == true @GraphOfHumanityErrors::CanNotAssignJudgesBeforeFeePaid,
        constraint = citizenship_appl.judges.len() as u64 == NUM_OF_JUDGES || citizenship_appl.judges.len() as u64 == treasury.num_of_citizens @GraphOfHumanityErrors::CitizenApplJudgesAlreadyAssigned,
        constraint = citizenship_appl.randomness_account == None @GraphOfHumanityErrors::RandomnessJudgeAlreadyRequested
    )]
    pub citizenship_appl: Account<'info, CitizenshipApplication>,
    /// CHECK: The account's data is validated manually within the handler.
    pub randomness_account_data: AccountInfo<'info>,
    #[account(
        seeds = [
            b"treasury"
        ],
        bump=treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RequestRandomnessJudges>) -> Result<()> {
    let citizenship_appl = &mut ctx.accounts.citizenship_appl;
    let randomness_account = &ctx.accounts.randomness_account_data;
    let clock = Clock::get()?;
    let randomness_data = RandomnessAccountData::parse(randomness_account.data.borrow()).unwrap();

    require!(
        randomness_data.seed_slot != clock.slot - 1,
        GraphOfHumanityErrors::RandomnessAlreadyRevealed
    );
    citizenship_appl.randomness_account = Some(randomness_account.key());
    emit!(JudgeRandomnessRequested {
        citizenship_appl: citizenship_appl.key()
    });
    Ok(())
}
