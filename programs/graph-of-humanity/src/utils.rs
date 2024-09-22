use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_error::ProgramError;
use orao_solana_vrf::state::RandomnessAccountData;
use sha2::{Digest, Sha256};

pub fn get_random_account_data(account_info: &AccountInfo) -> Result<RandomnessAccountData> {
    if account_info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount.into());
    }

    let account = RandomnessAccountData::try_deserialize(&mut &account_info.data.borrow()[..])?;
    Ok(account)
}

pub fn sha256_hash(input: u64, seed: u64) -> u64 {
    let mut hasher = Sha256::new();
    hasher.update(input.to_le_bytes());
    hasher.update(seed.to_le_bytes());
    let result = hasher.finalize();
    u64::from_le_bytes(result[0..8].try_into().unwrap())
}
