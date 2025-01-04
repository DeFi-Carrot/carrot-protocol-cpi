use anchor_lang::prelude::*;
use carrot_protocol_cpi::{typedefs::InitVaultArgs, cpi::{accounts::InitVault, init_vault}, ID as CarrotProgramId};
use anchor_spl::token_interface::Mint;

declare_id!("9KoNuRAqLjM3141SaREdHC7GChbWp3KffcyVGHrspAPV");

#[program]
pub mod example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let init_vault_accounts = InitVault {
            vault: ctx.accounts.vault.to_account_info(),
            shares: ctx.accounts.shares.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };

        let args = InitVaultArgs {
            redemption_fee_bps: 0,
            management_fee_bps: 0,
            performance_fee_bps: 0,
        };

        init_vault(
            CpiContext::new(
                ctx.accounts.carrot_program.to_account_info(),
                init_vault_accounts,
            ),
            args
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: seeds
    #[account(mut, seeds = [b"vault", shares.key().as_ref()], bump, seeds::program = CarrotProgramId)]
    pub vault: UncheckedAccount<'info>,

    pub shares: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub carrot_program: Program<'info, carrot_protocol_cpi::program::Carrot>,
}
