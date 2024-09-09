use anchor_lang::prelude::*;
pub mod constants;
pub mod error;
pub mod event;
pub mod instructions;
pub mod state;

use instructions::*;
declare_id!("GigY2BgaW1iJn5JQS2JbBCDkVbPeLErNpJqpwoQY63YD");

#[program]
pub mod graph_of_humanity {
    use super::*;

    pub fn initialize(ctx: Context<InitializeContract>, initialize_message: String) -> Result<()> {
        initialize::handler(ctx, initialize_message)
    }

    pub fn register_member(
        ctx: Context<BecomeMember>,
        citizen_name: String,
        bio_link: String,
    ) -> Result<()> {
        become_member::handler(ctx, citizen_name, bio_link)
    }

    pub fn edit_bio(ctx: Context<EditBio>, new_bio_link: String) -> Result<()> {
        edit_bio::handler(ctx, new_bio_link)
    }

    pub fn apply_citizenship(
        ctx: Context<ApplyCitizenship>,
        video_link: String,
        other_verifying_links: Option<String>,
    ) -> Result<()> {
        apply_citizenship::handler(ctx, video_link, other_verifying_links)
    }

    pub fn fund_voucher(ctx: Context<FundVoucher>) -> Result<()> {
        fund_voucher::handler(ctx)
    }

    pub fn request_randomness_voters(ctx:Context<RequestRandomnessJudges>) -> Result<()>{
        request_randomness_voters::handler(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
