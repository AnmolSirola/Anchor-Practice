use anchor_lang::prelude::*;

declare_id!("CvLS5ou1aaxBjJwmb9jtL5d659NUrKhz41ZQhZmidRr3");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
