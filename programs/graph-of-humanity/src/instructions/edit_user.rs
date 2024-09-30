use crate::event::UserEdited;
use crate::state::Member;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct EditUser<'info> {
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

pub fn handler(ctx: Context<EditUser>, new_bio_link: String, new_username: String) -> Result<()> {
    let member = &mut ctx.accounts.member;

    member.bio_link = new_bio_link.clone();
    member.citizen_name = new_username.clone();

    emit!(UserEdited {
        member: member.key(),
        new_bio_link: new_bio_link,
        new_username: new_username
    });

    Ok(())
}
