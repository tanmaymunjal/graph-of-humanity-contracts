use crate::event::BioEdited;
use crate::state::Member;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct EditBio<'info> {
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

pub fn handler(ctx: Context<EditBio>, new_bio: String) -> Result<()> {
    let member = &mut ctx.accounts.member;

    member.bio = new_bio.clone();

    emit!(BioEdited {
        member: member.key(),
        new_bio: new_bio,
    });

    Ok(())
}
