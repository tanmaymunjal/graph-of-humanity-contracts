use crate::constants::NUM_OF_JUDGES;
use crate::error::GraphOfHumanityErrors;
use crate::event::JudgeRandomnessRequested;
use crate::state::{CitizenshipApplication, Member, Treasury};
use anchor_lang::prelude::*;
use orao_solana_vrf::program::OraoVrf;
use orao_solana_vrf::state::NetworkState;
use orao_solana_vrf::CONFIG_ACCOUNT_SEED;
use orao_solana_vrf::RANDOMNESS_ACCOUNT_SEED;

#[derive(Accounts)]
#[instruction(force: [u8; 32])]
pub struct RequestRandomnessJudges<'info> {
    #[account(mut)]
    pub cranker: Signer<'info>,
    #[account(
        mut,
        constraint = citizenship_appl.fee_paid == true && citizenship_appl.voucher_fee_paid == true @GraphOfHumanityErrors::CanNotAssignJudgesBeforeFeePaid,
        constraint = citizenship_appl.judges.len() as u64 != NUM_OF_JUDGES || citizenship_appl.judges.len() as u64 != treasury.num_of_citizens @GraphOfHumanityErrors::CitizenApplJudgesAlreadyAssigned,
        constraint = citizenship_appl.randomness_account == None @GraphOfHumanityErrors::RandomnessJudgeAlreadyRequested
    )]
    pub citizenship_appl: Account<'info, CitizenshipApplication>,
    /// CHECK: The account's data is validated within the ORAO VRF program
    #[account(
        mut,
        seeds = [RANDOMNESS_ACCOUNT_SEED, &force],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    pub randomness_account: AccountInfo<'info>,
    #[account(
        seeds = [
            b"treasury"
        ],
        bump=treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    /// CHECK: This is the ORAO VRF program
    pub vrf_program: Program<'info, OraoVrf>,
    #[account(
        mut,
        seeds = [CONFIG_ACCOUNT_SEED],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    pub vrf_config: Account<'info, NetworkState>,
    /// CHECK: This is the treasury account for the ORAO VRF program
    #[account(mut)]
    pub vrf_treasury: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RequestRandomnessJudges>, force: [u8; 32]) -> Result<()> {
    // Zero seed is illegal in VRF
    require!(
        force != [0_u8; 32],
        GraphOfHumanityErrors::NullSeedInvalidForVrf
    );

    let citizenship_appl = &mut ctx.accounts.citizenship_appl;
    let randomness_account = &ctx.accounts.randomness_account;

    // Request randomness from ORAO VRF
    let cpi_program = ctx.accounts.vrf_program.to_account_info();
    let cpi_accounts = orao_solana_vrf::cpi::accounts::RequestV2 {
        payer: ctx.accounts.cranker.to_account_info(),
        network_state: ctx.accounts.vrf_config.to_account_info(),
        treasury: ctx.accounts.vrf_treasury.to_account_info(),
        request: randomness_account.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    orao_solana_vrf::cpi::request_v2(cpi_ctx, force)?;

    citizenship_appl.randomness_account = Some(randomness_account.key());
    emit!(JudgeRandomnessRequested {
        citizenship_appl: citizenship_appl.key()
    });
    Ok(())
}
