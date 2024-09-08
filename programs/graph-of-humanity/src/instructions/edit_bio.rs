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

pub fn handler(ctx: Context<EditBio>, new_bio_link: String) -> Result<()> {
    let member = &mut ctx.accounts.member;

    member.bio_link = new_bio_link.clone();

    emit!(BioEdited {
        member: member.key(),
        new_bio_link: new_bio_link,
    });

    Ok(())
}
