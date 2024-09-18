use crate::constants::UBI_USERS_PER_ACC;
use crate::error::GraphOfHumanityErrors;
use crate::event::UBISelection;
use crate::state::{DistributionEpoch, Treasury, UBIRandomnessAccount};
use crate::utils::sha256_hash;
use anchor_lang::prelude::*;
use std::collections::HashSet;
use switchboard_on_demand::accounts::RandomnessAccountData;

#[derive(Accounts)]
pub struct RevealRandomnessUBI<'info> {
    pub cranker: Signer<'info>,
    /// CHECK: The account's data is validated manually within the handler.
    pub randomness_account_data: AccountInfo<'info>,
    #[account(
        seeds = [
            b"treasury"
        ],
        bump=treasury.bump,
        constraint = treasury.distribution_active == true @GraphOfHumanityErrors::NoEpochStarted
    )]
    pub treasury: Account<'info, Treasury>,
    #[account(
        seeds = [
            &treasury.distributions.to_le_bytes(),
            b"di_epoch",
        ],
        bump=epoch.bump
    )]
    pub epoch: Account<'info, DistributionEpoch>,
    #[account(
        mut,
        constraint = ubi_randomness_acc.accounts.len() <UBI_USERS_PER_ACC as usize @GraphOfHumanityErrors::RandomnessFull,
        constraint = ubi_randomness_acc.epoch == epoch.key() @GraphOfHumanityErrors::WrongEpochSelected,
        constraint = ubi_randomness_acc.randomness_account == randomness_account_data.key() @GraphOfHumanityErrors::WrongRandomnessAccountSelected
    )]
    pub ubi_randomness_acc: Account<'info, UBIRandomnessAccount>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RevealRandomnessUBI>) -> Result<()> {
    let clock = Clock::get()?;
    let randomness_account = &ctx.accounts.randomness_account_data;
    let epoch = &ctx.accounts.epoch;
    let ubi_randomness_acc = &mut ctx.accounts.ubi_randomness_acc;

    let randomness_data = RandomnessAccountData::parse(randomness_account.data.borrow()).unwrap();
    let revealed_random_value = randomness_data.get_value(&clock);
    let citizens = epoch.distribution_max_user_ind;
    let mut recipients = ubi_randomness_acc.accounts.clone();

    match revealed_random_value {
        Err(_) => {}
        Ok(random_val) => {
            if citizens < UBI_USERS_PER_ACC {
                recipients = (0..citizens).collect();
            } else {
                let numbers: Vec<u64> = (0..UBI_USERS_PER_ACC as usize)
                    .map(|i| {
                        let start = i * 6;
                        random_val[start..start + 6]
                            .iter()
                            .fold(0u64, |acc, &x| (acc << 8) | x as u64)
                    })
                    .collect();

                let seed = (random_val[30] as u64) * (random_val[31] as u64);

                let mut unique_recipients = HashSet::new();
                for num in numbers.iter() {
                    if recipients.len() as u64 == UBI_USERS_PER_ACC {
                        break;
                    }
                    let hashed = sha256_hash(*num, seed);
                    let recipient_index = hashed % citizens;
                    if unique_recipients.insert(recipient_index) {
                        recipients.push(recipient_index);
                    }
                }
            }
            ubi_randomness_acc.accounts = recipients;
        }
    };

    emit!(UBISelection {
        accounts: ubi_randomness_acc.accounts.clone()
    });

    Ok(())
}
