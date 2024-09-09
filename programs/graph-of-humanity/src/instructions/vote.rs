use crate::error::GraphOfHumanityErrors;
use crate::event::CommitteeVoted;
use crate::state::{CitizenshipApplication, CommitteeVotes, Member};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct VoteCitizenship<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,
    #[account(
        seeds = [
            voter.key().as_ref(),
            b"member"
        ],
        bump=voter_member.bump,
        constraint = voter_member.citizen == true @GraphOfHumanityErrors::CanNotVoteAsANonCitizen
    )]
    pub voter_member: Account<'info, Member>,
    #[account(
        seeds = [
            voter_member.key().as_ref(),
            &voter_member.num_of_appeals.to_le_bytes(),
            b"citizenship_appl"
        ],
        bump=voter_citizenship.bump
    )]
    pub voter_citizenship: Account<'info, CitizenshipApplication>,
    #[account(constraint = member.citizen == false @GraphOfHumanityErrors::DontReapplyForCitizenWhenAlreadyOne)]
    pub member: Account<'info, Member>,
    #[account(
        mut,
        seeds = [
            member.key().as_ref(),
            &member.num_of_appeals.to_le_bytes(),
            b"citizenship_appl"
        ],
        bump=member_citizenship_appl.bump,
        constraint = member_citizenship_appl.judges.contains(&voter_citizenship.citizen_index.unwrap()) @ GraphOfHumanityErrors::VoterNotInJudges
    )]
    pub member_citizenship_appl: Account<'info, CitizenshipApplication>,
    #[account(
        init,
        space = 8+CommitteeVotes::INIT_SPACE,
        payer = voter,
        seeds = [
            b"vote",
            voter.key().as_ref(),
            member_citizenship_appl.key().as_ref()
        ],
        bump
    )]
    pub vote_acc: Account<'info, CommitteeVotes>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<VoteCitizenship>, accept: bool, reason: Option<String>) -> Result<()> {
    let voter = &ctx.accounts.voter;
    let member_citizenship_appl = &mut ctx.accounts.member_citizenship_appl;
    let vote_acc = &mut ctx.accounts.vote_acc;
    if accept {
        member_citizenship_appl.votes += 1;
    };

    vote_acc.bump = ctx.bumps.vote_acc;
    vote_acc.voter = voter.key();
    vote_acc.citizenship_appl = member_citizenship_appl.key();
    vote_acc.accept = accept;
    vote_acc.claimed = false;

    emit!(CommitteeVoted {
        citizenship_appl: member_citizenship_appl.key(),
        voter: voter.key(),
        accept: accept,
        reason: reason
    });
    Ok(())
}
