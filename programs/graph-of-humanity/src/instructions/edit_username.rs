use crate::event::UsernameEdited;
use crate::state::Member;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct EditUsername<'info> {
    pub member_creator: Signer<'info>,
    #[account(
        mut,
        seeds = [
            member_creator.key().as_ref(),
            b"member"
        ],
        bump=member.bump
    )]
    pub member: Account<'info, Member>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<EditUsername>, new_username: String) -> Result<()> {
    let member = &mut ctx.accounts.member;

    member.username = new_username.clone();

    emit!(UsernameEdited {
        member: member.key(),
        new_username: new_username
    });

    Ok(())
}
