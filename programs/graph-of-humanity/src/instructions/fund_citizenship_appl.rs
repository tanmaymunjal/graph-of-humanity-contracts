use crate::constants::CITIZENSHIP_FEE;
use crate::event::CitizenshipFeePaid;
use crate::state::{CitizenshipApplication, Member, Treasury};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint::USDC,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct FundCitizenshipAppl<'info> {
    #[account(mut)]
    pub member_creator: Signer<'info>,
    #[account(
        seeds = [
            member_creator.key().as_ref(),
            b"member"
        ],
        bump=member.bump
    )]
    pub member: Account<'info, Member>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = member_creator
    )]
    pub member_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [
            member.key().as_ref(),
            &citizenship_appl.appeal_number.to_le_bytes(),
            b"citizenship_appl"
        ],
        bump=citizenship_appl.bump
    )]
    pub citizenship_appl: Account<'info, CitizenshipApplication>,
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
    pub treasury_token_account: Account<'info, TokenAccount>,
    #[account(address=USDC)]
    pub usdc_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler(ctx: Context<FundCitizenshipAppl>) -> Result<()> {
    let member_token_account = &mut ctx.accounts.member_token_account;
    let member_creator = &mut ctx.accounts.member_creator;
    let treasury = &ctx.accounts.treasury;
    let citizenship_appl = &mut ctx.accounts.citizenship_appl;

    let fee = 2u64.pow(citizenship_appl.appeal_number as u32) * CITIZENSHIP_FEE;
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: member_token_account.to_account_info(),
                to: treasury.to_account_info(),
                authority: member_creator.to_account_info(),
            },
        ),
        fee,
    )?;

    // Update the citizenship application status
    citizenship_appl.fee_paid = true;

    emit!(CitizenshipFeePaid {
        citizenship_appl: citizenship_appl.key()
    });

    Ok(())
}
