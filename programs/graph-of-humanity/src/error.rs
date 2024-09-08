use anchor_lang::error_code;

#[error_code]
pub enum GraphOfHumanityErrors {
    // 6000
    #[msg("Your current citizenship application is already pending, please fund it or wait for it to be decided!")]
    CitizenshipApplicationPending,

    // 6001
    #[msg("Can not fund voucher when you are not a member yourselves!")]
    NonMemberCantVouch,

    // 6002
    #[msg("Can not fulfill somebody else's voucher")]
    CanNotFullfillSomebodyElseVoucher,
}
