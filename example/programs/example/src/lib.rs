use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use carrot_protocol_cpi::{
    cpi::{
        accounts::{InitVault, Issue},
        init_vault, issue,
    },
    program::Carrot,
    typedefs::{InitVaultArgs, IssueArgs},
    Vault, ID as CarrotProgramId,
};
use std::str::FromStr;

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
            args,
        )?;

        Ok(())
    }

    pub fn deposit<'a, 'b, 'c, 'info>(ctx: Context<'a, 'b, 'c, 'info, Deposit<'info>>, amount: u64) -> Result<()> {
        let issue_accounts = Issue {
            vault: ctx.accounts.vault.to_account_info(),
            shares: ctx.accounts.shares.to_account_info(),
            user_shares_ata: ctx.accounts.user_shares_ata.to_account_info(),
            asset: ctx.accounts.asset.to_account_info(),
            vault_asset_ata: ctx.accounts.vault_asset_ata.to_account_info(),
            user_asset_ata: ctx.accounts.user_asset_ata.to_account_info(),
            user: ctx.accounts.user.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            asset_token_program: ctx.accounts.asset_token_program.to_account_info(),
            shares_token_program: ctx.accounts.shares_token_program.to_account_info(),
            log_program: ctx.accounts.log_program.to_account_info(),
        };


        // to get the remaining accounts
        // you must get the vault data off chain
        // and pass in the vault token account and oracle defined for each asset
        // currently this would be 3 assets, USDC, USDT, and pyUSD

        //
        // off chain example code from @carrot-protocol/rpc-client
        //

        //const vaultData: Vault = await this.getVault(vault);
        //const assetsData: web3.PublicKey[] = vaultData.assets.flatMap(
        //  (asset: Asset) => [asset.ata, asset.oracle],
        //);

        issue(
            CpiContext::new(
                ctx.accounts.carrot_program.to_account_info(),
                issue_accounts,
            )
            .with_remaining_accounts(ctx.remaining_accounts.to_vec()),
            IssueArgs { amount },
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

#[derive(Accounts)]
pub struct Deposit<'info> {
    /// CHECK: seeds
    #[account(mut,
        has_one = shares,
        seeds = [b"vault", shares.key().as_ref()], bump, seeds::program = CarrotProgramId)]
    pub vault: Account<'info, Vault>,

    pub shares: InterfaceAccount<'info, Mint>,

    pub asset: InterfaceAccount<'info, Mint>,

    #[account(mut, token::mint = asset, token::authority = vault)]
    pub vault_asset_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, token::mint = shares, token::authority = user)]
    pub user_shares_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, token::mint = asset, token::authority = user)]
    pub user_asset_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub asset_token_program: Interface<'info, TokenInterface>,

    pub shares_token_program: Interface<'info, TokenInterface>,

    pub carrot_program: Program<'info, Carrot>,

    /// CHECK: manual
    #[account(executable, address = Pubkey::from_str("7Mc3vSdRWoThArpni6t5W4XjvQf4BuMny1uC8b6VBn48").unwrap())]
    pub log_program: UncheckedAccount<'info>,
}
