use anchor_lang::prelude::*;

declare_id!("8BGgPTYHbvd6U8AGURgpJ2gdh7Gsrtg5MaqpAv8KnR16");

#[program]
pub mod day3 {
    use super::*;

/* 
    //The default initialize function
    pub fn add(ctx: Context<NonEmptyAccountExample>, 
                      a: u64,
                      b: u64) -> Result<()> {
        let sum = a + b;                               
        msg!("Let sum be {}", sum );                
        Ok(())
    }

    pub fn sub(ctx: Context<NonEmptyAccountExample>, 
                      a: u64,
                      b: u64) -> Result<()> {
        let diff = a - b;                               
        msg!("Let diff be {}", diff );                
        Ok(())
    }   
*/
    pub fn function_a(ctx: Context<NonEmptyAccountExample>) -> Result<()> {
        Ok(())
    }

    pub fn function_b(ctx: Context<Empty>, firstArg: u64) -> Result<()> { //2
        Ok(())
    } 
}     

#[derive(Accounts)]
pub struct NonEmptyAccountExample<'info> { //1
    signer: Signer<'info>,
    another_signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Empty {} //2 