use crate::constants::NUM_OF_JUDGES;
use crate::error::GraphOfHumanityErrors;
use crate::event::JudgeRandomnessRevealed;
use crate::state::{CitizenshipApplication, Member, Treasury};
use anchor_lang::prelude::*;
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use switchboard_on_demand::accounts::RandomnessAccountData;

#[derive(Accounts)]
pub struct RevealRandomnessJudges<'info> {
    pub cranker: Signer<'info>,
    #[account(
        seeds = [
            cranker.key().as_ref(),
            b"member"
        ],
        bump=cranker_member.bump,
        constraint = cranker_member.citizen == true @GraphOfHumanityErrors::CanNotUseCrankAsANonCitizen
    )]
    pub cranker_member: Account<'info, Member>,
    #[account(
        mut,
        constraint = citizenship_appl.fee_paid == true && citizenship_appl.voucher_fee_paid == true @GraphOfHumanityErrors::CanNotAssignJudgesBeforeFeePaid,
        constraint = citizenship_appl.judges.len() as u64 == NUM_OF_JUDGES || citizenship_appl.judges.len() as u64 == treasury.num_of_citizens @GraphOfHumanityErrors::CitizenApplJudgesAlreadyAssigned
    )]
    pub citizenship_appl: Account<'info, CitizenshipApplication>,
    /// CHECK: The account's data is validated manually within the handler.
    #[account(constraint=randomness_account_data.key()==citizenship_appl.randomness_account.unwrap() @GraphOfHumanityErrors::JudgeRandomnessNotStoredHere)]
    pub randomness_account_data: AccountInfo<'info>,
    #[account(
        seeds = [
            b"treasury"
        ],
        bump=treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RevealRandomnessJudges>) -> Result<()> {
    let clock = Clock::get()?;
    let randomness_account = &ctx.accounts.randomness_account_data;
    let citizenship_appl = &mut ctx.accounts.citizenship_appl;
    let treasury = &ctx.accounts.treasury;

    let randomness_data = RandomnessAccountData::parse(randomness_account.data.borrow()).unwrap();
    let revealed_random_value = randomness_data.get_value(&clock);
    let citizens = treasury.num_of_citizens;

    citizenship_appl.randomness_account = None;
    let mut judges = citizenship_appl.judges.clone();
    match revealed_random_value {
        Err(_) => {}
        Ok(random_val) => {
            if citizens < NUM_OF_JUDGES {
                judges = (0..citizens).collect();
            } else {
                let numbers: Vec<u64> = (0..NUM_OF_JUDGES as usize)
                    .map(|i| {
                        let start = i * 6;
                        random_val[start..start + 6]
                            .iter()
                            .fold(0u64, |acc, &x| (acc << 8) | x as u64)
                    })
                    .collect();

                let seed = (random_val[30] as u64) * (random_val[31] as u64);

                let mut unique_judges = HashSet::new();
                for num in numbers.iter() {
                    if judges.len() as u64 == NUM_OF_JUDGES {
                        break;
                    }
                    let hashed = sha256_hash(*num, seed);
                    let judge_index = hashed % citizens;
                    if unique_judges.insert(judge_index) {
                        judges.push(judge_index);
                    }
                }
            }

            citizenship_appl.judges = judges;
        }
    };

    if citizenship_appl.judges.len() as u64 == NUM_OF_JUDGES
        || citizenship_appl.judges.len() as u64 == citizens
    {
        let current_time = Clock::get()?.unix_timestamp;
        citizenship_appl.voting_started = Some(current_time);
    };

    emit!(JudgeRandomnessRevealed {
        citizenship_appl: citizenship_appl.key(),
        choosen_judges: citizenship_appl.judges.clone()
    });

    Ok(())
}

fn sha256_hash(input: u64, seed: u64) -> u64 {
    let mut hasher = Sha256::new();
    hasher.update(input.to_le_bytes());
    hasher.update(seed.to_le_bytes());
    let result = hasher.finalize();
    u64::from_le_bytes(result[0..8].try_into().unwrap())
}
