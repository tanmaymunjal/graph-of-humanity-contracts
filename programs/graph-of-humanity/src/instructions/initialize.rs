use crate::event::ContractInitialized;
use crate::state::Treasury;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint::USDC,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct InitializeContract<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(
        init,
        payer=initializer,
        space=8+Treasury::INIT_SPACE,
        seeds = [
            b"treasury"
        ],
        bump
    )]
    pub treasury: Account<'info, Treasury>,
    #[account(
        init,
        payer = initializer,
        associated_token::mint = usdc_mint,
        associated_token::authority = treasury
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    // #[account(address=USDC)]
    pub usdc_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler(ctx: Context<InitializeContract>, initialize_message: String) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    treasury.bump = ctx.bumps.treasury;

    emit!(ContractInitialized {
        message: initialize_message
    });

    Ok(())
}
