use anchor_lang::prelude::*;

#[event]
pub struct MemberRegistered {
    pub member_creator: Pubkey,
    pub username: String,
    pub bio: String,
}

#[event]
pub struct UsernameEdited {
    pub member: Pubkey,
    pub new_username: String,
}

#[event]
pub struct BioEdited {
    pub member: Pubkey,
    pub new_bio: String,
}

#[event]
pub struct CitizenshipApplied {
    pub member: Pubkey,
    pub voucher_member: Pubkey,
    pub video_link: String,
    pub other_verifying_links: Option<String>,
    pub appeal_number: u8,
}

#[event]
pub struct CitizenshipFeePaid {
    pub citizenship_appl: Pubkey,
}

#[event]
pub struct CitizenshipVoucherFeeApplied {
    pub citizenship_appl: Pubkey,
}

#[event]
pub struct ContractInitialized {
    pub message: String,
}
