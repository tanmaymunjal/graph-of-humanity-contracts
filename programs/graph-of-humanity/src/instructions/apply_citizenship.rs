use crate::error::GraphOfHumanityErrors;
use crate::event::CitizenshipApplied;
use crate::state::{CitizenshipApplication, Member};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ApplyCitizenship<'info> {
    #[account(mut)]
    pub member_creator: Signer<'info>,
    /// CHECK: account which is vouching for the creator, this will not be verfied till voucher pays for the vouch!
    pub member_voucher: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [
            member_creator.key().as_ref(),
            b"member"
        ],
        bump=member.bump,
        constraint = member.appeal_pending == false @GraphOfHumanityErrors::CitizenshipApplicationPending,
        constraint = member.citizen == false @GraphOfHumanityErrors::DontReapplyForCitizenWhenAlreadyOne
    )]
    pub member: Account<'info, Member>,
    #[account(
        seeds = [
            member_voucher.key().as_ref(),
            b"member"
        ],
        constraint = member_voucher_account.citizen == true @GraphOfHumanityErrors::NonMemberCantVouch,
        bump=member_voucher_account.bump
    )]
    pub member_voucher_account: Account<'info, Member>,
    #[account(
        init,
        payer=member_creator,
        space=8+CitizenshipApplication::INIT_SPACE,
        seeds = [
            member.key().as_ref(),
            &member.num_of_appeals.to_le_bytes(),
            b"citizenship_appl"
        ],
        bump
    )]
    pub citizenship_appl: Account<'info, CitizenshipApplication>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<ApplyCitizenship>,
    video_link: String,
    other_verifying_links: Option<String>,
) -> Result<()> {
    let member = &mut ctx.accounts.member;
    let member_voucher = &ctx.accounts.member_voucher;
    let citizenship_appl = &mut ctx.accounts.citizenship_appl;

    citizenship_appl.bump = ctx.bumps.citizenship_appl;
    citizenship_appl.member = member.key();
    citizenship_appl.voucher_member = member_voucher.key();
    citizenship_appl.video_link = video_link.clone();
    citizenship_appl.other_verifying_links = other_verifying_links.clone();
    citizenship_appl.fee_paid = false;
    citizenship_appl.voucher_fee_paid = false;
    citizenship_appl.appeal_number = member.num_of_appeals;
    citizenship_appl.judges = vec![];
    citizenship_appl.votes = 0;
    citizenship_appl.randomness_account = None;
    citizenship_appl.voting_started = None;
    citizenship_appl.citizen_index = None;

    emit!(CitizenshipApplied {
        member: member.key(),
        voucher_member: member_voucher.key(),
        video_link: video_link,
        other_verifying_links: other_verifying_links,
        appeal_number: member.num_of_appeals - 1
    });

    Ok(())
}
