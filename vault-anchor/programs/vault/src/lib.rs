#![allow(unexpected_cfgs)]
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
pub mod states;
use states::VaultState;

declare_id!("3g4ygkXUtPqecoFjaqQnN7NVw7XpcG5ztjpCswbs7Q7k");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }
}

//Deposit
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
           mut,
           seeds=[b"state",payer.key().as_ref()],
           bump=vault_state.state_bump
       )]
    pub vault_state: Account<'info, VaultState>,
    #[account(mut,seeds=[b"vault",vault_state.key().as_ref()],bump=vault_state.vault_bump)]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

//Withdraw
#[derive(Accounts)]
pub struct WithDraw<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
           mut,
           seeds=[b"state",payer.key().as_ref()],
           bump=vault_state.state_bump
       )]
    pub vault_state: Account<'info, VaultState>,
    #[account(mut,seeds=[b"vault",vault_state.key().as_ref()],bump=vault_state.vault_bump)]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

//Close
#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds=[b"state",signer.key().as_ref()],
        bump=vault_state.state_bump,
        close = signer
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(mut,seeds=[b"vault",vault_state.key().as_ref()],bump=vault_state.vault_bump)]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

// Init Context and Instruction
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
         init,
         payer=signer,
         seeds=[b"vault",signer.key().as_ref()],
         space=VaultState::INIT_SPACE,
         bump,
     )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        seeds=[b"vault",vault_state.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

//Instructions
impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.vault_state.state_bump = bumps.vault_state;
        self.vault_state.vault_bump = bumps.vault;
        Ok(())
    }
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_account = Transfer {
            from: self.payer.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_account);
        transfer(cpi_ctx, amount)
    }
}

impl<'info> WithDraw<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_account = Transfer {
            to: self.payer.to_account_info(),
            from: self.vault.to_account_info(),
        };

        let seeds = &[
            &b"vault"[..],
            &self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_account, signer_seeds);
        transfer(cpi_ctx, amount)
    }
}

impl<'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_account = Transfer {
            from: self.vault.to_account_info(),
            to: self.signer.to_account_info(),
        };

        let seeds = &[
            &b"vault"[..],
            &self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_account, signer_seeds);
        transfer(cpi_ctx, self.vault.get_lamports())
    }
}
