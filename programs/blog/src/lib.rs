use anchor_lang::prelude::*;

declare_id!("Ex4CxjdqdvXsfcGFwgkgnp2fUsxYLFwPRJMoRmQ52B3W");

#[program]
pub mod blog {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
