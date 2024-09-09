use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Member {
    pub bump: u8,
    pub member_creator: Pubkey,
    #[max_len(40)]
    pub citizen_name: String,
    #[max_len(100)]
    pub bio_link: String,
    pub citizen: bool,
    pub num_of_appeals: u8,
    pub appeal_pending: bool,
}

#[account]
#[derive(InitSpace)]
pub struct Treasury {
    pub bump: u8,
    pub num_of_citizens: u64,
}

#[account]
#[derive(InitSpace)]
pub struct CitizenshipApplication {
    pub bump: u8,
    pub member: Pubkey,
    pub voucher_member: Pubkey,
    #[max_len(100)]
    pub video_link: String,
    #[max_len(100)]
    pub other_verifying_links: Option<String>,
    pub fee_paid: bool,
    pub voucher_fee_paid: bool,
    pub appeal_number: u8,
    #[max_len(5)]
    pub judges: Vec<u64>,
    pub votes: u8,
    pub randomness_account: Option<Pubkey>,
    pub voting_started: Option<i64>,
    pub citizen_index: Option<u64>,
}

#[account]
#[derive(InitSpace)]
pub struct CommitteeVotes {
    pub bump: u8,
    pub voter: Pubkey,
    pub citizenship_appl: Pubkey,
    pub accept: bool,
    pub claimed: bool,
}
