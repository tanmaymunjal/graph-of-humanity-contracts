use crate::constants::CITIZENSHIP_FEE;
use crate::error::GraphOfHumanityErrors;
use crate::event::CitizenshipVoucherFeeApplied;
use crate::state::{CitizenshipApplication, Member, Treasury};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint::USDC,
    token_interface::{transfer, Mint, TokenAccount, TokenInterface, Transfer},
};

#[derive(Accounts)]
pub struct FundVoucher<'info> {
    #[account(mut)]
    pub member_voucher: Signer<'info>,
    #[account(
        seeds = [
            member_voucher.key().as_ref(),
            b"member"
        ],
        constraint = member_voucher_account.citizen == true @GraphOfHumanityErrors::NonMemberCantVouch,
        bump=member_voucher_account.bump
    )]
    pub member_voucher_account: Account<'info, Member>,
    #[account(
        mut,
        token::mint = usdc_mint,
        token::authority = member_voucher,
        token::token_program = token_program
    )]
    pub member_voucher_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        constraint = citizenship_appl.voucher_member == member_voucher.key() @GraphOfHumanityErrors::CanNotFullfillSomebodyElseVoucher,
        constraint = citizenship_appl.voucher_fee_paid == false @GraphOfHumanityErrors::CitizenshipVoucherAlreadyFunded
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
        token::mint = usdc_mint,
        token::authority = treasury,
        token::token_program = token_program
    )]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,
    // #[account(address=USDC)]
    pub usdc_mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler(ctx: Context<FundVoucher>) -> Result<()> {
    let member_voucher_token_account = &mut ctx.accounts.member_voucher_token_account;
    let member_voucher = &mut ctx.accounts.member_voucher;
    let treasury_token_account = &mut ctx.accounts.treasury_token_account;
    let usdc_mint = &ctx.accounts.usdc_mint;
    let citizenship_appl = &mut ctx.accounts.citizenship_appl;

    let fee = 2u64.pow(citizenship_appl.appeal_number as u32)
        * CITIZENSHIP_FEE
        * 10u64.pow(usdc_mint.decimals as u32);
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: member_voucher_token_account.to_account_info(),
                to: treasury_token_account.to_account_info(),
                authority: member_voucher.to_account_info(),
            },
        ),
        fee,
    )?;

    // Update the citizenship application status
    citizenship_appl.voucher_fee_paid = true;

    emit!(CitizenshipVoucherFeeApplied {
        citizenship_appl: citizenship_appl.key()
    });

    Ok(())
}
