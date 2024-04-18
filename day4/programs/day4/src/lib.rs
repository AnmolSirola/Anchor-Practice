
use anchor_lang::prelude::*;

declare_id!("8ncntT5Y9SPGQYbwSX6pHbFrm7DREuRNvYJy39s117j5");

#[program]
pub mod day4 {
    use super::*;

    pub fn limit_range(ctx: Context<LimitRange>, a: u64) -> Result<()> {
        if a < 10 {
            return err!(MyError::AisTooSmall);
        }
        if a > 100 {
            return err!(MyError::AisTooBig);
        }
        msg!("The Result is {}", a);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LimitRange {}

#[error_code]
pub enum MyError{
    #[msg{"is a too big"}]
    AisTooBig,
    #[msg{"is a too small"}]
    AisTooSmall,
}
