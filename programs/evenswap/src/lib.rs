use anchor_lang::prelude::*;

declare_id!("8KswSMtEQ9sjHkZ7ogYy1e2wcnntFnCJigNijTFiQEfe");

#[program]
pub mod evenswap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
