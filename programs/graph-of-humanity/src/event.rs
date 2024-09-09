use anchor_lang::prelude::*;

#[event]
pub struct MemberRegistered {
    pub member_creator: Pubkey,
    pub citizen_name: String,
    pub bio_link: String,
}

#[event]
pub struct UsernameEdited {
    pub member: Pubkey,
    pub new_username: String,
}

#[event]
pub struct BioEdited {
    pub member: Pubkey,
    pub new_bio_link: String,
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

#[event]
pub struct JudgeRandomnessRequested {
    pub citizenship_appl: Pubkey,
}

#[event]
pub struct JudgeRandomnessRevealed {
    pub citizenship_appl: Pubkey,
    pub choosen_judges: Vec<u64>,
}

#[event]
pub struct CommitteeVoted {
    pub citizenship_appl: Pubkey,
    pub voter: Pubkey,
    pub accept: bool,
    pub reason: Option<String>,
}

#[event]
pub struct CitizenshipResultDeclared {
    pub citizenship_appl: Pubkey,
    pub accepted: bool,
}

#[event]
pub struct RewardClaimed {
    pub citizenship_appl: Pubkey,
    pub voter: Pubkey,
}
