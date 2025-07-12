use anchor_lang::prelude::*;

declare_id!("5J42cgZv3WJL3ZWNRRKcZDvyLEnSraegFbFT9WKgwPaj");

#[program]
pub mod pump_fun {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
