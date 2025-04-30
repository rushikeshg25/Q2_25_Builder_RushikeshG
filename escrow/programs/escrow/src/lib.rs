#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
pub mod instructions;
pub mod states;
use crate::instructions::*;

declare_id!("DqgAWdWBXMNQfrN8AmzNCDpSEtv9rRNoqegmo7ACY7c7");

#[program]
pub mod escrow {

    use super::*;

    pub fn make(ctx: Context<Make>, seeds: u64, receive_amt: u64, deposit_amt: u64) -> Result<()> {
        ctx.accounts
            .init_escrow_state(seeds, receive_amt, ctx.bumps)?;
        ctx.accounts.deposit(deposit_amt)?;
        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.withdraw()?;
        ctx.accounts.close()?;
        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.withdraw()?;
        ctx.accounts.close()?;
        Ok(())
    }
}
