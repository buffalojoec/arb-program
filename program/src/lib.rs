//! Arbitrage bot between two swap programs!
mod arb;
mod error;
mod swap;
mod util;

use arb::try_arbitrage;
use borsh::{BorshDeserialize, BorshSerialize};
use error::ArbitrageProgramError;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};
use util::{check_pool_address, load_arbitrage_accounts};

/// The two swap programs we want to consider arbitrage opportunities between
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ArbBotArgs {
    pub swap_1_program_id: Pubkey,
    pub swap_1_pool_address: Pubkey,
    pub swap_2_program_id: Pubkey,
    pub swap_2_pool_address: Pubkey,
}

entrypoint!(process);

/// Program's entrypoint (has only one instruction type)
fn process(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Make sure the pool addresses match the provided program IDs
    let args = ArbBotArgs::try_from_slice(instruction_data)?;
    check_pool_address(args.swap_1_program_id, args.swap_1_pool_address)?;
    check_pool_address(args.swap_2_program_id, args.swap_2_pool_address)?;

    // Read in all of the accounts and aggregate them by type
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let (token_accounts_user, token_accounts_swap_1, token_accounts_swap_2, mints) =
        load_arbitrage_accounts(
            accounts_iter,
            payer.key,
            &args.swap_1_pool_address,
            &args.swap_2_pool_address,
        )?;

    // Ensure the vectors are the same length
    let mints_len = mints.len();
    if !(mints_len == token_accounts_user.len()
        && mints_len == token_accounts_swap_1.len()
        && mints_len == token_accounts_swap_2.len())
    {
        return Err(ArbitrageProgramError::InvalidAccountsList.into());
    }

    // Check if there is an arbitrage opportunity between the two pools, and
    // execute the trade if there is one
    try_arbitrage(
        token_accounts_user,
        token_accounts_swap_1,
        token_accounts_swap_2,
        mints,
        payer,
        token_program,
    )
}
