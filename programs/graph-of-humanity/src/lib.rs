use anchor_lang::prelude::*;
pub mod event;
pub mod instructions;
pub mod state;
pub mod error;

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

    pub fn edit_username(ctx: Context<EditUsername>, new_username: String) -> Result<()> {
        edit_username::handler(ctx,new_username)
    }

    pub fn apply_citizenship(ctx: Context<ApplyCitizenship>, video_link: String, other_verifying_links: Option<String>) -> Result<()>{
        apply_citizenship::handler(ctx, video_link, other_verifying_links)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
