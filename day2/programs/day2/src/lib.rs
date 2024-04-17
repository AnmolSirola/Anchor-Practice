use anchor_lang::prelude::*;

declare_id!("8BGgPTYHbvd6U8AGURgpJ2gdh7Gsrtg5MaqpAv8KnR16");

#[program]
pub mod day2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
