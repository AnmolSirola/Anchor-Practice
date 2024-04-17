use anchor_lang::prelude::*;

declare_id!("8BGgPTYHbvd6U8AGURgpJ2gdh7Gsrtg5MaqpAv8KnR16");

#[program]
pub mod day2 {
    use super::*;

    //The default initialize function
    pub fn initialize(ctx: Context<Initialize>,
                      a: u64,
                      b: u64,
                      message: String) -> Result<()> {
        msg!("We now that your a {:?}", message);                
        msg!("We will know pass {} and {}", a, b);                
        Ok(())
    }

    pub fn array(ctx: Context<Initialize>,
                      arr: Vec<u64>) -> Result<()>{
        msg!("Here is your fuckin {:?}", arr);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
