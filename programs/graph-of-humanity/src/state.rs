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
}

#[account]
#[derive(InitSpace)]
pub struct CommitteeVoters {
    pub bump: u8,
    pub voter: Pubkey,
    pub committee: Pubkey,
    pub voted: bool,
    pub claimed: bool,
}

#[account]
#[derive(InitSpace)]
pub struct CitizenshipCommittee {
    pub bump: u8,
    pub appl: Pubkey,
    pub instantiated: i64,
    #[max_len(5)]
    pub voters: Vec<Pubkey>,
    pub accept_votes: u8,
    pub reject_votes: u8,
    #[max_len(100)]
    pub rejection_reason: Option<String>,
}
