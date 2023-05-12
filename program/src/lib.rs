//! Arbitrage bot between two swap programs!
use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint,
        entrypoint::ProgramResult,
        msg,
        program::invoke,
        pubkey::Pubkey,
    },
};

/// The two swap programs we want to consider arbitrage opportunities between
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ArbBotArgs {
    pub swap_1: Pubkey,
    pub swap_2: Pubkey,
}

entrypoint!(process);

/// Program's entrypoint (has only one instruction type)
fn process(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let args = ArbBotArgs::try_from_slice(instruction_data)?;
    let _swap_program_1 = args.swap_1;
    let _swap_program_2 = args.swap_2;

    // TODO: Replace with bytemuck fns
    let is_token_account_1 = true; // This should be a flag for when the `Signer` pops up
    let is_token_account_2 = true; // This should be a flag for when the first `Mint` pops up

    let accounts_iter = &mut accounts.iter();

    // Load all token accounts for pool #1
    let _token_accounts_swap_1: Vec<&AccountInfo> = {
        let mut accounts: Vec<&AccountInfo> = vec![];
        loop {
            let account = next_account_info(accounts_iter)?;
            if is_token_account_1 {
                accounts.push(account);
            } else {
                break;
            }
        }
        accounts
    };

    // Load all token accounts for pool #2
    let _token_accounts_swap_2: Vec<&AccountInfo> = {
        let mut accounts: Vec<&AccountInfo> = vec![];
        loop {
            let account = next_account_info(accounts_iter)?;
            if is_token_account_2 {
                accounts.push(account);
            } else {
                break;
            }
        }
        accounts
    };

    // Load all mints
    let _mints: Vec<&AccountInfo> = {
        let mut accounts: Vec<&AccountInfo> = vec![];
        while accounts_iter.len() != 0 {
            accounts.push(next_account_info(accounts_iter)?);
        }
        accounts
    };

    // Search for arbitrage opportunity

    // If one is found, run the `invoke_arbitrage()..)` fn

    Ok(())
}

fn invoke_arbitrage(
    swap_1: (Pubkey, &[AccountInfo], u64),
    swap_2: (Pubkey, &[AccountInfo], u64),
) -> ProgramResult {
    // invoke(instruction, account_infos)?;
    // invoke(instruction, account_infos)?;
    msg!("Arbitrage executed");
    Ok(())
}
