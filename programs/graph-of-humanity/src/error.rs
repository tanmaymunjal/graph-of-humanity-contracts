use anchor_lang::error_code;

#[error_code]
pub enum GraphOfHumanityErrors{

    // 6000
    #[msg("Your current citizenship application is already pending, please fund it or wait for it to be decided!")]
    CitizenshipApplicationPending,
    
}
