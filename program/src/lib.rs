//! Arbitrage bot between two swap programs!

use bytemuck::{Pod, Zeroable};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
};
use std::ops::{Add, Div, Mul};
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
    pub swap_1: (Pubkey, Pubkey),
    pub swap_2: (Pubkey, Pubkey),
}

entrypoint!(process);

/// Program's entrypoint (has only one instruction type)
fn process(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let args = ArbBotArgs::try_from_slice(instruction_data)?;
    let (swap_1_program_id, swap_1_pool_address) = args.swap_1;
    let (swap_2_program_id, swap_2_pool_address) = args.swap_2;

    let accounts_iter = &mut accounts.iter();

    let payer = next_account_info(accounts_iter)?;

    let mut token_accounts_swap_1: Vec<(&AccountInfo, Pubkey, Pubkey, u64)> = vec![];
    let mut mint: Vec<(&AccountInfo, u8)> = vec![];
    let mut token_accounts_swap_2: Vec<(&AccountInfo, Pubkey, Pubkey, u64)> = vec![];
    let mut user_token_accounts: Vec<(&AccountInfo, Pubkey, Pubkey, u64)> = vec![];

    // TODO: Everything should be in same order
    loop {
        let account = next_account_info(accounts_iter)?;

        let data = &account.data.borrow()[..64];

        match bytemuck::try_from_bytes::<PartialTokenAccount>(data) {
            Ok(token_account_data) => {
                if token_account_data.owner.eq(&swap_1_pool_address) {
                    token_accounts_swap_1.push((
                        account,
                        token_account_data.mint,
                        token_account_data.owner,
                        token_account_data.amount,
                    ))
                } else if token_account_data.owner.eq(&swap_2_pool_address) {
                    token_accounts_swap_2.push((
                        account,
                        token_account_data.mint,
                        token_account_data.owner,
                        token_account_data.amount,
                    ))
                } else if token_account_data.owner.eq(&payer.key) {
                    user_token_accounts.push((
                        account,
                        token_account_data.mint,
                        token_account_data.owner,
                        token_account_data.amount,
                    ))
                } else {
                    return Err(solana_program::program_error::ProgramError::IncorrectProgramId);
                }
            }
            Err(_) => {
                let mint_data = &data[44..45];
                match bytemuck::try_from_bytes::<PartialMintAccount>(mint_data) {
                    Ok(mint_data) => mint.push((account, mint_data.decimals)),
                    Err(_) => {
                        return Err(solana_program::program_error::ProgramError::InvalidAccountData)
                    }
                }
            }
        }
        if accounts_iter.len() == 0 {
            break;
        }
    }

    // Check if the vectors are the same length

    if !(token_accounts_swap_1.len() == mint.len() && token_accounts_swap_2.len() == mint.len()) {
        return Err(solana_program::program_error::ProgramError::NotEnoughAccountKeys);
    }

    // Check if there is an arbitrage opportunity between the two pools

    

    Ok(())
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Zeroable, Pod)]
pub struct PartialTokenAccount {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Zeroable, Pod)]
pub struct PartialMintAccount {
    pub decimals: u8,
}

fn find_token_account(data: &[u8]) -> Option<&PartialTokenAccount> {
    todo!()
}

fn find_mint_decimals(data: &[u8]) -> Option<&PartialMintAccount> {
    todo!()
}

fn invoke_arbitrage(
    swap_1: (Pubkey, &[AccountInfo], u64),
    swap_2: (Pubkey, &[AccountInfo], u64),
) -> ProgramResult {
    let account_metas_1 = swap_1
        .1
        .iter()
        .map(|account| AccountMeta {
            pubkey: *account.key,
            is_signer: account.is_signer,
            is_writable: account.is_writable,
        })
        .collect::<Vec<AccountMeta>>();

    let ix1 = Instruction::new_with_borsh(swap_1.0, &swap_1.2, account_metas_1);

    let account_metas_2 = swap_2
        .1
        .iter()
        .map(|account| AccountMeta {
            pubkey: *account.key,
            is_signer: account.is_signer,
            is_writable: account.is_writable,
        })
        .collect::<Vec<AccountMeta>>();

    let ix2 = Instruction::new_with_borsh(swap_2.0, &swap_2.2, account_metas_2);

    invoke(&ix1, swap_1.1)?;
    invoke(&ix2, swap_2.1)?;
    msg!("Arbitrage executed");
    Ok(())
}

fn determine_swap_receive(
    pool_recieve_balance: u64,
    receive_decimals: u8,
    pool_pay_balance: u64,
    pay_decimals: u8,
    pay_amount: u64,
) -> Result<u64, ProgramError> {
    // Convert all values to nominal floats using their respective mint decimal
    // places
    let big_r = convert_to_float(pool_recieve_balance, receive_decimals);
    let big_p = convert_to_float(pool_pay_balance, pay_decimals);
    let p = convert_to_float(pay_amount, pay_decimals);
    // Calculate `f(p)` to get `r`
    let bigr_times_p = big_r.mul(p);
    let bigp_plus_p = big_p.add(p);
    let r = bigr_times_p.div(bigp_plus_p);
    // Make sure `r` does not exceed liquidity
    if r > big_r {
        return Err(ProgramError::InsufficientFunds);
    }
    // Return the real value of `r`
    Ok(convert_from_float(r, receive_decimals))
}

/// Converts a `u64` value - in this case the balance of a token account - into
/// an `f32` by using the `decimals` value of its associated mint to get the
/// nominal quantity of a mint stored in that token account
///
/// For example, a token account with a balance of 10,500 for a mint with 3
/// decimals would have a nominal balance of 10.5
fn convert_to_float(value: u64, decimals: u8) -> f32 {
    (value as f32).div(f32::powf(10.0, decimals as f32))
}

/// Converts a nominal value - in this case the calculated value `r` - into a
/// `u64` by using the `decimals` value of its associated mint to get the real
/// quantity of the mint that the user will receive
///
/// For example, if `r` is calculated to be 10.5, the real amount of the asset
/// to be received by the user is 10,500
fn convert_from_float(value: f32, decimals: u8) -> u64 {
    value.mul(f32::powf(10.0, decimals as f32)) as u64
}
