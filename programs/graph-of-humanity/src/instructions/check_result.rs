use crate::constants::{DAY, NUM_OF_JUDGES};
use crate::error::GraphOfHumanityErrors;
use crate::event::CitizenshipResultDeclared;
use crate::state::{CitizenshipApplication, Member, Treasury};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CheckVoteResult<'info> {
    pub cranker: Signer<'info>,
    #[account(mut,constraint = member.citizen == false @GraphOfHumanityErrors::DontReapplyForCitizenWhenAlreadyOne)]
    pub member: Account<'info, Member>,
    #[account(
        mut,
        seeds = [
            member.key().as_ref(),
            member_citizenship_appl.appl_id.as_bytes(),
            b"citizenship_appl"
        ],
        bump=member_citizenship_appl.bump,
        constraint = member_citizenship_appl.voting_started.is_some() @ GraphOfHumanityErrors::VotingNotStarted,
        constraint = Clock::get()?.unix_timestamp - member_citizenship_appl.voting_started.unwrap() > DAY @ GraphOfHumanityErrors::VotingStillOngoing
    )]
    pub member_citizenship_appl: Account<'info, CitizenshipApplication>,
    #[account(
        mut,
        seeds = [
            b"treasury"
        ],
        bump=treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CheckVoteResult>) -> Result<()> {
    let citizenship_appl = &mut ctx.accounts.member_citizenship_appl;
    let member = &mut ctx.accounts.member;
    let treasury = &mut ctx.accounts.treasury;

    let mut threshhold;
    threshhold = (NUM_OF_JUDGES / 2) + 1;
    if (treasury.num_of_citizens < threshhold) {
        threshhold = treasury.num_of_citizens;
    };

    // Check if the number of votes is over half
    let mut accepted = false;
    if (citizenship_appl.accept_vote as u64) >= threshhold {
        member.citizen = true;
        member.citizen_index = Some(treasury.num_of_citizens);
        treasury.num_of_citizens += 1;
        accepted = true;
    } else {
        member.num_of_appeals += 1;
        member.appeal_pending = false;
    };

    emit!(CitizenshipResultDeclared {
        citizenship_appl: citizenship_appl.key(),
        accepted: accepted
    });

    Ok(())
}
