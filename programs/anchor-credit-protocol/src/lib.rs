use anchor_lang::prelude::*;

declare_id!("3hCQwkaJM5ePHyT2YDnCxE5K8DT1WpucB16yuJnMcpde");

#[program]
pub mod anchor_credit_protocol {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
