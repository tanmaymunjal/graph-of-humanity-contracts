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

    // 6003
    #[msg("Can not choose judges before user has paid citizenship fee and so has his voucher")]
    CanNotAssignJudgesBeforeFeePaid,

    // 6004
    #[msg("Can only apply crank as a member citizen")]
    CanNotUseCrankAsANonCitizen,

    // 6005
    #[msg("Can not fund already funded citizenship appl")]
    CitizenshipApplAlreadyFunded,

    // 6006
    #[msg("Can not fund already funded citizenship voucher")]
    CitizenshipVoucherAlreadyFunded,

    // 6007
    #[msg("Judges already assigned")]
    CitizenApplJudgesAlreadyAssigned,

    // 6008
    #[msg("Randomness already revealed")]
    RandomnessAlreadyRevealed,

    // 6009
    #[msg("Judge randomness is not stored in this account")]
    JudgeRandomnessNotStoredHere,

    // 6010
    #[msg("Randomness already requested, pls use it first")]
    RandomnessJudgeAlreadyRequested,

    // 6010
    #[msg("You are already a citizien, why are you re-applying?")]
    DontReapplyForCitizenWhenAlreadyOne,
}
