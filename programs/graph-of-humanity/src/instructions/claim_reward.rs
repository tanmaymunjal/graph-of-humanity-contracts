use crate::constants::{CITIZENSHIP_FEE, DAY};
use crate::error::GraphOfHumanityErrors;
use crate::event::RewardClaimed;
use crate::state::{CitizenshipApplication, CommitteeVotes, Member, Treasury};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint::USDC,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct ClaimVoteReward<'info> {
    pub voter: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = voter
    )]
    pub voter_token_account: Account<'info, TokenAccount>,
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
        constraint = member_citizenship_appl.judges.contains(&voter_member.citizen_index.unwrap()) @ GraphOfHumanityErrors::VoterNotInJudges,
        constraint = member_citizenship_appl.voting_started.is_some() @ GraphOfHumanityErrors::VotingNotStarted,
        constraint = Clock::get()?.unix_timestamp - member_citizenship_appl.voting_started.unwrap() > DAY @ GraphOfHumanityErrors::VotingStillOngoing
    )]
    pub member_citizenship_appl: Account<'info, CitizenshipApplication>,
    #[account(
        mut,
        seeds = [
            b"vote",
            voter.key().as_ref(),
            member_citizenship_appl.key().as_ref()
        ],
        bump=vote_acc.bump,
        constraint = vote_acc.claimed == false @GraphOfHumanityErrors::AlreadyClaimedVoterMoney
    )]
    pub vote_acc: Account<'info, CommitteeVotes>,
    #[account(
        seeds = [
            b"treasury"
        ],
        bump=treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    #[account(
        mut,
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

pub fn handler(ctx: Context<ClaimVoteReward>) -> Result<()> {
    let treasury = &ctx.accounts.treasury;
    let treasury_token_account = &mut ctx.accounts.treasury_token_account;
    let voter_token_account = &mut ctx.accounts.voter_token_account;
    let vote_acc = &mut ctx.accounts.vote_acc;
    let member_citizenship_appl = &ctx.accounts.member_citizenship_appl;
    let voter = &ctx.accounts.voter;

    vote_acc.claimed = true;
    let accept: bool = member_citizenship_appl.accept_vote > member_citizenship_appl.reject_votes;
    let total_fee = (2u64.pow(member_citizenship_appl.appeal_number as u32) + 1) * CITIZENSHIP_FEE;
    let voter_claim = 16 * (total_fee / 100);
    if vote_acc.accept == accept {
        let signer_seeds: &[&[&[u8]]] = &[&[b"treasury", &[treasury.bump]]];
        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: treasury_token_account.to_account_info(),
                    to: voter_token_account.to_account_info(),
                    authority: treasury.to_account_info(),
                },
                signer_seeds,
            ),
            voter_claim,
        )?;
    };

    emit!(RewardClaimed {
        citizenship_appl: member_citizenship_appl.key(),
        voter: voter.key()
    });

    Ok(())
}
