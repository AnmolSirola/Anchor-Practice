use anchor_lang::prelude::*;

declare_id!("2TQQ6sVSQ9cmxgDeLHzxmftPCZtmhp6cBpDcwRY4ivZv");

#[program]
pub mod day5 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        struct Person{
            my_name: String,
            my_age: u64,
        }

        let mut person1: Person = Person {
            my_age: name,
            my_age: age,
        };

        msg!("{} is {} years old", person1.my_name, person1.my_age);

        person1.my_name = "Bob".to_String();
        person1.my_age = 16;

        msg!("{} is {} years old", person1.my_name, person1.my_age);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
