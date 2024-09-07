use anchor_lang::prelude::*;

declare_id!("GigY2BgaW1iJn5JQS2JbBCDkVbPeLErNpJqpwoQY63YD");

#[program]
pub mod graph_of_humanity {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
