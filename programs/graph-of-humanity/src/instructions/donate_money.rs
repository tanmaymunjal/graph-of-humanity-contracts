use crate::event::MoneyDonated;
use crate::state::Treasury;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint::USDC,
    token_interface::{transfer, Mint, TokenAccount, TokenInterface, Transfer},
};

#[derive(Accounts)]
pub struct DonateMoney<'info> {
    pub doner: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = doner
    )]
    pub doner_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [
            b"treasury"
        ],
        bump=treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = treasury
    )]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,
    // #[account(address=USDC)]
    pub usdc_mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler(ctx: Context<DonateMoney>, amount: u64) -> Result<()> {
    let doner_token_account = &mut ctx.accounts.doner_token_account;
    let doner = &ctx.accounts.doner;
    let treasury_token_account = &mut ctx.accounts.treasury_token_account;

    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: doner_token_account.to_account_info(),
                to: treasury_token_account.to_account_info(),
                authority: doner.to_account_info(),
            },
        ),
        amount,
    )?;

    emit!(MoneyDonated {
        doner: doner.key(),
        amount: amount
    });

    Ok(())
}
