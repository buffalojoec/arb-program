//! Arbitrage opportunity spotting and trade placement
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};
// import determine_swap_price from swap.rs
use crate::swap::determine_swap_receive;

/// Checks to see if there is an arbitrage opportunity between the two pools,
/// and executes the trade if there is one
pub fn try_arbitrage(
    token_accounts_user: Vec<(&AccountInfo<'_>, Pubkey, Pubkey, u64)>,
    token_accounts_swap_1: Vec<(&AccountInfo<'_>, Pubkey, Pubkey, u64)>,
    token_accounts_swap_2: Vec<(&AccountInfo<'_>, Pubkey, Pubkey, u64)>,
    mints: Vec<(&AccountInfo<'_>, u8)>,
    _payer: &AccountInfo<'_>,
    _token_program: &AccountInfo<'_>,
) -> ProgramResult {
    // Make sure the vector lengths are 2 and the mints are the same for all.
    assert_eq!(token_accounts_user.len(), 2);
    assert_eq!(token_accounts_swap_1.len(), 2);
    assert_eq!(token_accounts_swap_2.len(), 2);
    assert_eq!(mints.len(), 2);
    assert_eq!(token_accounts_user[0].1, token_accounts_swap_1[0].1);
    assert_eq!(token_accounts_user[0].1, token_accounts_swap_2[0].1);

    // Here we assume user has the token at index 0.
    let user_token_balance = token_accounts_user[0].3;

    // Calculate the amount received from the first swap.
    let swap_1_received = determine_swap_receive(
        token_accounts_swap_1[1].3, // pool_receive_balance
        mints[1].1,                 // receive_decimals
        token_accounts_swap_1[0].3, // pool_pay_balance
        mints[0].1,                 // pay_decimals
        user_token_balance,         // pay_amount
    )?;

    // Calculate the amount received from the second swap.
    let swap_2_received = determine_swap_receive(
        token_accounts_swap_2[1].3, // pool_receive_balance
        mints[0].1,                 // receive_decimals
        token_accounts_swap_2[0].3, // pool_pay_balance
        mints[1].1,                 // pay_decimals
        swap_1_received,            // pay_amount
    )?;

    // Check for arbitrage opportunity.
    if swap_2_received > user_token_balance {
        println!(
            "Arbitrage opportunity found! Initial: {}, After Swaps: {}",
            user_token_balance, swap_2_received
        );

        // Invoke the arbitrage cpi.
        invoke_arbitrage(
            (
                token_accounts_swap_1[0].2,
                [
                    token_accounts_swap_1[0].0.clone(),
                    token_accounts_swap_1[1].0.clone(),
                ]
                .as_ref(),
                token_accounts_swap_1[0].3,
            ),
            (
                token_accounts_swap_2[0].2,
                [
                    token_accounts_swap_2[0].0.clone(),
                    token_accounts_swap_2[1].0.clone(),
                ]
                .as_ref(),
                token_accounts_swap_2[1].3,
            ),
        );
    } else {
        // random error for now
        return Err(solana_program::program_error::ProgramError::InvalidInstructionData);
    }

    Ok(())
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
