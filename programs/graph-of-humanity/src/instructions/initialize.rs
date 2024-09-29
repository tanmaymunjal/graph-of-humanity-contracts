use crate::event::ContractInitialized;
use crate::state::{CitizenshipApplication, Member, Treasury};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint::USDC,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

#[derive(Accounts)]
#[instruction(initialize_message: String)]
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
        associated_token::authority = treasury,
        token::token_program = token_program,
    )]
    pub treasury_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    // #[account(address=USDC)]
    pub usdc_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        init,
        payer=initializer,
        space=8+Member::INIT_SPACE,
        seeds = [
            initializer.key().as_ref(),
            b"member"
        ],
        bump
    )]
    pub member: Box<Account<'info, Member>>,
    #[account(
        init,
        payer=initializer,
        space=8+CitizenshipApplication::INIT_SPACE,
        seeds = [
            member.key().as_ref(),
            initialize_message.as_bytes(),
            b"citizenship_appl"
        ],
        bump
    )]
    pub citizenship_appl: Box<Account<'info, CitizenshipApplication>>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler(ctx: Context<InitializeContract>, initialize_message: String) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    let initializer = &ctx.accounts.initializer;
    let citizenship_appl = &mut ctx.accounts.citizenship_appl;
    let member = &mut ctx.accounts.member;

    treasury.bump = ctx.bumps.treasury;
    treasury.num_of_citizens = 1;
    treasury.distributions = 0;
    treasury.distribution_active = false;

    member.bump = ctx.bumps.member;
    member.member_creator = initializer.key();
    member.citizen_name = "admin".to_string();
    member.bio_link = "admin".to_string();
    member.citizen = true;
    member.num_of_appeals = 0;
    member.appeal_pending = false;
    member.citizen_index = Some(0);

    citizenship_appl.bump = ctx.bumps.citizenship_appl;
    citizenship_appl.member = member.key();
    citizenship_appl.voucher_member = member.key();
    citizenship_appl.video_link = "admin".to_string();
    citizenship_appl.other_verifying_links = None;
    citizenship_appl.fee_paid = true;
    citizenship_appl.appeal_number = member.num_of_appeals;
    citizenship_appl.voucher_fee_paid = true;
    citizenship_appl.judges = vec![];
    citizenship_appl.accept_vote = 0;
    citizenship_appl.reject_votes = 0;
    citizenship_appl.randomness_account = None;
    citizenship_appl.voting_started = None;

    emit!(ContractInitialized {
        message: initialize_message
    });

    Ok(())
}
