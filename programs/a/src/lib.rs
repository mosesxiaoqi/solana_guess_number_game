use anchor_lang::prelude::*;

declare_id!("3zwhjgGpr8syrkSF8PWWFnfR6heXzv1npTC9wCxH9rbe");

#[program]
pub mod a {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
