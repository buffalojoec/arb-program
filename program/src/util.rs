//! Util functions for arbitrage bot
use bytemuck::{Pod, Zeroable};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token_2022::pod::OptionalNonZeroPubkey;
use std::slice::Iter;

use crate::error::ArbitrageProgramError;

// Asserts the pool address provided is in fact derived from the program ID
// provided
pub fn check_pool_address(program_id: Pubkey, pool: Pubkey) -> ProgramResult {
    if !Pubkey::find_program_address(&[b"liquidity_pool"], &program_id)
        .0
        .eq(&pool)
    {
        return Err(solana_program::program_error::ProgramError::InvalidInstructionData);
    }
    Ok(())
}

/// The first three fields of the `spl_token::state::Account`
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct PartialTokenAccountState {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
}

/// The first two fields of the `spl_token::state::Mint`
///
/// The third field `decimals` - which is the one we are interested in - cannot
/// be included in this struct since Bytemuck will not allow two integer types
/// of varying size - such as `u64` and `u8`
///
/// However, since `decimals` is a single byte (`u8`), we can simply take the
/// next byte if the data deserializes into this struct properly
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct PartialMintState {
    pub mint_authority: OptionalNonZeroPubkey,
    pub supply: u64,
}

type ArbitrageAccounts<'a, 'b> = (
    Vec<(&'a AccountInfo<'b>, Pubkey, Pubkey, u64)>,
    Vec<(&'a AccountInfo<'b>, Pubkey, Pubkey, u64)>,
    Vec<(&'a AccountInfo<'b>, Pubkey, Pubkey, u64)>,
    Vec<(&'a AccountInfo<'b>, u8)>,
);

/// Reads each account from the accounts iterator and partially deserializes
/// it using zero-copy to check if it is a Token Account or a Mint, and
/// then adds the necessary data to the proper vector to be returned
pub fn load_arbitrage_accounts<'a, 'b>(
    accounts_iter: &'a mut Iter<AccountInfo<'b>>,
    payer_pubkey: &'a Pubkey,
    swap_1_pool_pubkey: &'a Pubkey,
    swap_2_pool_pubkey: &'a Pubkey,
) -> Result<ArbitrageAccounts<'a, 'b>, ProgramError> {
    let mut token_accounts_user = vec![];
    let mut token_accounts_swap_1 = vec![];
    let mut token_accounts_swap_2 = vec![];
    let mut mints = vec![];
    loop {
        let account = next_account_info(accounts_iter)?;
        // The `PartialTokenAccountState` will only be comprised of the first 64 bytes
        // of the account data
        match bytemuck::try_from_bytes::<PartialTokenAccountState>(&account.data.borrow()[..64]) {
            Ok(token_account_data) => {
                if token_account_data.owner.eq(payer_pubkey) {
                    token_accounts_user.push((
                        account,
                        token_account_data.mint,
                        token_account_data.owner,
                        token_account_data.amount,
                    ));
                } else if token_account_data.owner.eq(swap_1_pool_pubkey) {
                    token_accounts_swap_1.push((
                        account,
                        token_account_data.mint,
                        token_account_data.owner,
                        token_account_data.amount,
                    ));
                } else if token_account_data.owner.eq(swap_2_pool_pubkey) {
                    token_accounts_swap_2.push((
                        account,
                        token_account_data.mint,
                        token_account_data.owner,
                        token_account_data.amount,
                    ));
                } else {
                    // If the owner doesn't match any of the three provided addresses (user, swap1
                    // pool, swap2 pool), throw an error
                    return Err(ArbitrageProgramError::TokenAccountOwnerNotFound.into());
                }
            }
            // If the data can't be deserialized as a `PartialTokenAccountState`, try a
            // `PartialMintState`
            //
            // If the data can't be deserialized as a `PartialTokenAccountState` or a
            // `PartialMintState`, break out of the loop and return
            Err(_) => {
                // The `PartialMintState` will only be comprised of the first 44 bytes
                // of the account data
                //
                // We first check to validate the account is a mint by attempting to deserialize
                // `PartialMintState` from the first 44 bytes
                match bytemuck::try_from_bytes::<PartialMintState>(&account.data.borrow()[..44]) {
                    Ok(_) => {
                        // If that deserialization is successful, we can take the 45th byte as the
                        // `u8` value for `decimals`
                        match account.data.borrow().get(45) {
                            Some(decimals) => mints.push((account, *decimals)),
                            None => {
                                // If there's no byte at index 45 (the `u8` for `decimals`), break
                                // out of the loop and return
                                break;
                            }
                        }
                    }
                    Err(_) => {
                        // If the data can't be deserialized as a `PartialMintState`, break out
                        // of the loop and return
                        break;
                    }
                }
            }
        }
    }
    Ok((
        token_accounts_user,
        token_accounts_swap_1,
        token_accounts_swap_2,
        mints,
    ))
}
