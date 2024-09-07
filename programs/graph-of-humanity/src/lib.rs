use anchor_lang::prelude::*;
pub mod event;
pub mod instructions;
pub mod state;

use instructions::*;
declare_id!("GigY2BgaW1iJn5JQS2JbBCDkVbPeLErNpJqpwoQY63YD");

#[program]
pub mod graph_of_humanity {
    use super::*;

    pub fn register_member(
        ctx: Context<BecomeMember>,
        username: String,
        bio: String,
    ) -> Result<()> {
        become_member::handler(ctx, username, bio)
    }

    pub fn edit_bio(ctx: Context<EditBio>, new_bio: String) -> Result<()> {
        edit_bio::handler(ctx, new_bio)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
