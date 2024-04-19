/* Single tx.origin
use anchor_lang::prelude::*;

declare_id!("Hf96fZsgq9R6Y1AHfyGbhi9EAmaQw2oks8NqakS6XVt1");

#[program]
pub mod day14 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let the_signer1: &mut Signer = &mut ctx.accounts.signer1;

				// Function logic....

        msg!("The signer1: {:?}", *the_signer1.key);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer1: Signer<'info>,
}
 */


 use anchor_lang::prelude::*;

declare_id!("Hf96fZsgq9R6Y1AHfyGbhi9EAmaQw2oks8NqakS6XVt1");

#[program]
pub mod day14 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result{
        let the_signer1: &mut Signer = &mut ctx.accounts.signer1;
        let the_signer2: &mut Signer = &mut ctx.accounts.signer2;
     
        msg!("The signer1: {:?}", *the_signer1.key);
        msg!("The signer2: {:?}", *the_signer2.key);

        Ok(())
    }
}

#[derive[Accounts]]
pub struct initialize<'info>{
    pub signer1: Signer<'info>,
    pub signer2: Signer<'info>,
}

