// Account initialization code

use anchor_lang::prelude::*;
use std::meme:size_of;

declare_id!("5TypCRpxDdHuvfvV9aSgM1aAdH3tfzzLZWeQdueD5VLA");

#[program]
pub mod day7 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {

    #[account(init,
              payer = signer,
              space=size_of::<MyStorage>() + 8,
              seeds = [],
              bump)]

    pub my_storage:Account<'info, MyStorage>

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct MyStorage {
    x: u64,
}