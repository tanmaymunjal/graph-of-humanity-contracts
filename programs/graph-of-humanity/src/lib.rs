use anchor_lang::prelude::*;
pub mod constants;
pub mod error;
pub mod event;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;
declare_id!("sqy5Z44qPEs9jtSy5RKY8v7xNJcdZgXTvy9m6hgGjry");

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

    pub fn edit_user(
        ctx: Context<EditUser>,
        new_bio_link: String,
        new_username: String,
    ) -> Result<()> {
        edit_user::handler(ctx, new_bio_link, new_username)
    }

    pub fn apply_citizenship(
        ctx: Context<ApplyCitizenship>,
        citizenship_id: String,
        video_link: String,
        other_verifying_links: Option<String>,
    ) -> Result<()> {
        apply_citizenship::handler(ctx, citizenship_id, video_link, other_verifying_links)
    }

    pub fn fund_citizenship_appl(ctx: Context<FundCitizenshipAppl>) -> Result<()> {
        fund_citizenship_appl::handler(ctx)
    }

    pub fn fund_voucher(ctx: Context<FundVoucher>) -> Result<()> {
        fund_voucher::handler(ctx)
    }

    pub fn request_randomness_voters(
        ctx: Context<RequestRandomnessJudges>,
        force: [u8; 32],
    ) -> Result<()> {
        request_randomness_voters::handler(ctx, force)
    }

    pub fn reveal_randomness_voters(ctx: Context<RevealRandomnessJudges>) -> Result<(Vec<u64>)> {
        reveal_randomness_voters::handler(ctx)
    }

    pub fn vote_citizen(
        ctx: Context<VoteCitizenship>,
        accept: bool,
        reason: Option<String>,
    ) -> Result<()> {
        vote::handler(ctx, accept, reason)
    }

    pub fn check_result(ctx: Context<CheckVoteResult>) -> Result<()> {
        check_result::handler(ctx)
    }

    pub fn claim_reward(ctx: Context<ClaimVoteReward>) -> Result<()> {
        claim_reward::handler(ctx)
    }

    pub fn claim_ubi(ctx: Context<ClaimUBI>) -> Result<()> {
        claim_ubi::handler(ctx)
    }

    pub fn start_distribution_epoch(ctx: Context<StartDistribution>) -> Result<()> {
        start_distribution_epoch::handler(ctx)
    }

    pub fn donate_money(ctx: Context<DonateMoney>, amount: u64) -> Result<()> {
        donate_money::handler(ctx, amount)
    }

    pub fn request_ubi_randomness(
        ctx: Context<RequestRandomnessUBI>,
        force: [u8; 32],
    ) -> Result<()> {
        request_ubi_randomness::handler(ctx, force)
    }

    pub fn reveal_ubi_randomness(ctx: Context<RevealRandomnessUBI>) -> Result<()> {
        reveal_ubi_randomness::handler(ctx)
    }
}
