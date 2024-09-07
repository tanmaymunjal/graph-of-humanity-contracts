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

pub fn handler(ctx: Context<BecomeMember>, username: String, bio: String) -> Result<()> {
    let member = &mut ctx.accounts.member;
    let member_creator = &ctx.accounts.member_creator;

    member.member_creator.key();
    member.username = username.clone();
    member.bio = bio.clone();
    member.citizen = false;
    member.num_of_appeals = 0;

    emit!(MemberRegistered {
        member_creator: member_creator.key(),
        username: username,
        bio: bio
    });

    Ok(())
}
