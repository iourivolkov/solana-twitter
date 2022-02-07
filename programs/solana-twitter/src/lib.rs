use anchor_lang::prelude::*;

declare_id!("D7mRyDMJE1SqnA8kq7wy6KJUZFPodQCKaEGHmknKgfgc");

#[program]
pub mod solana_twitter {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
