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

// defines structure of the tweet account
#[account]
pub struct Tweet {
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic: String,
    pub content: String,
}

// add constants for sizing properties of Tweet struct
const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_TOPIC_LENGTH: usize = 50 * 4;
const MAX_CONTENT_LENGTH: usize = 280 * 4;

// add constant on tweet account that provides its total size 
impl Tweet {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // author
        + TIMESTAMP_LENGTH // timestamp
        + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH // topic
        + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH // content
}