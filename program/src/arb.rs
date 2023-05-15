//! Arbitrage opportunity spotting and trade placement
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};

/// Checks to see if there is an arbitrage opportunity between the two pools,
/// and executes the trade if there is one
pub fn try_arbitrage(
    _token_accounts_user: Vec<(&AccountInfo<'_>, Pubkey, Pubkey, u64)>,
    _token_accounts_swap_1: Vec<(&AccountInfo<'_>, Pubkey, Pubkey, u64)>,
    _token_accounts_swap_2: Vec<(&AccountInfo<'_>, Pubkey, Pubkey, u64)>,
    _mints: Vec<(&AccountInfo<'_>, u8)>,
    _payer: &AccountInfo<'_>,
    _token_program: &AccountInfo<'_>,
) -> ProgramResult {
    todo!()
}

/// Trait used to convert from an `AccountInfo` to an `AccountMeta`
trait ToAccountMeta {
    fn to_account_meta(&self) -> AccountMeta;
}

impl ToAccountMeta for AccountInfo<'_> {
    fn to_account_meta(&self) -> AccountMeta {
        AccountMeta {
            pubkey: *self.key,
            is_signer: self.is_signer,
            is_writable: self.is_writable,
        }
    }
}

/// Invokes the arbitrage trade by sending a cross-program invocation (CPI) to
/// the first swap program and then the second immediately after
fn invoke_arbitrage(
    swap_1: (Pubkey, &[AccountInfo], u64),
    swap_2: (Pubkey, &[AccountInfo], u64),
) -> ProgramResult {
    let ix_swap_1 = Instruction::new_with_borsh(
        swap_1.0,
        &swap_1.2,
        swap_1
            .1
            .iter()
            .map(ToAccountMeta::to_account_meta)
            .collect(),
    );
    let ix_swap_2 = Instruction::new_with_borsh(
        swap_2.0,
        &swap_2.2,
        swap_2
            .1
            .iter()
            .map(ToAccountMeta::to_account_meta)
            .collect(),
    );
    invoke(&ix_swap_1, swap_1.1)?;
    invoke(&ix_swap_2, swap_2.1)?;
    Ok(())
}
