use anchor_lang::prelude::*;

declare_id!("AGGtK3Vzo8k5JFqmrmrDnGPwzGzUzenyUw6tbtDUbi84");

#[program]
pub mod extra_assets {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
