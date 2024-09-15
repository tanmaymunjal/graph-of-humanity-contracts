use crate::event::MemberRegistered;
use crate::state::Member;
use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct BecomeMember<'info> {
    #[account(mut)]
    pub member_creator: Signer<'info>,
    #[account(
        init,
        payer=member_creator,
        space=8+Member::INIT_SPACE,
        seeds = [
            member_creator.key().as_ref(),
            b"member"
        ],
        bump
    )]
    pub member: Account<'info, Member>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<BecomeMember>, citizen_name: String, bio_link: String) -> Result<()> {
    let member = &mut ctx.accounts.member;
    let member_creator = &ctx.accounts.member_creator;

    member.bump = ctx.bumps.member;
    member.member_creator = member_creator.key();
    member.citizen_name = citizen_name.clone();
    member.bio_link = bio_link.clone();
    member.citizen = false;
    member.num_of_appeals = 0;
    member.appeal_pending = false;
    member.citizen_index = None;

    emit!(MemberRegistered {
        member_creator: member_creator.key(),
        citizen_name: citizen_name,
        bio_link: bio_link
    });

    Ok(())
}
