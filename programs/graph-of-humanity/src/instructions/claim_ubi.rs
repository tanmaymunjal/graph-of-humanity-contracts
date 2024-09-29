use crate::error::GraphOfHumanityErrors;
use crate::event::UBIDistributed;
use crate::state::{ClaimHashMap, DistributionEpoch, Member, Treasury, UBIRandomnessAccount};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint::USDC,
    token_interface::{transfer, Mint, TokenAccount, TokenInterface, Transfer},
};

#[derive(Accounts)]
pub struct ClaimUBI<'info> {
    #[account(mut)]
    pub claimer: Signer<'info>,
    #[account(
        seeds = [
            claimer.key().as_ref(),
            b"member"
        ],
        bump=claimer_member_acc.bump,
        constraint = claimer_member_acc.citizen == true @GraphOfHumanityErrors::CanNotClaimUBI
    )]
    pub claimer_member_acc: Account<'info, Member>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = claimer
    )]
    pub claimer_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [
            b"treasury"
        ],
        bump=treasury.bump,
        constraint = treasury.distribution_active == true @GraphOfHumanityErrors::NoEpochStarted
    )]
    pub treasury: Account<'info, Treasury>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = treasury
    )]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [
            &treasury.distributions.to_le_bytes(),
            b"di_epoch",
        ],
        bump=epoch.bump
    )]
    pub epoch: Account<'info, DistributionEpoch>,
    #[account(
        mut,
        constraint = ubi_randomness_acc.epoch == epoch.key() @GraphOfHumanityErrors::WrongEpochSelected,
        constraint = ubi_randomness_acc.accounts.contains(&claimer_member_acc.citizen_index.unwrap()) @GraphOfHumanityErrors::CanNotClaimUBIIfNotChoosen
    )]
    pub ubi_randomness_acc: Account<'info, UBIRandomnessAccount>,
    #[account(
        init,
        payer=claimer,
        space=8+ClaimHashMap::INIT_SPACE,
        seeds = [
            claimer.key().as_ref(),
            epoch.key().as_ref(),
            b"claim_hashmap"
        ],
        bump
    )]
    pub claim_hashmap: Account<'info, ClaimHashMap>,
    // #[account(address=USDC)]
    pub usdc_mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler(ctx: Context<ClaimUBI>) -> Result<()> {
    let treasury_token_account = &mut ctx.accounts.treasury_token_account;
    let treasury = &mut ctx.accounts.treasury;
    let epoch = &mut ctx.accounts.epoch;
    let claimer = &ctx.accounts.claimer;
    let claimer_token_account = &mut ctx.accounts.claimer_token_account;

    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: treasury_token_account.to_account_info(),
                to: claimer_token_account.to_account_info(),
                authority: treasury.to_account_info(),
            },
        ),
        1000,
    )?;

    epoch.num_of_users_distributed += 1;
    if epoch.num_of_users_distributed == epoch.num_of_users_to_distribute {
        treasury.distributions += 1;
        treasury.distribution_active = false;
    };

    emit!(UBIDistributed {
        account: claimer.key()
    });
    Ok(())
}
