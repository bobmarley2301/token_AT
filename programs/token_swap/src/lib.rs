use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{Sysvar, rent::Rent},
};

use spl_associated_token_account::get_associated_token_address;
use spl_token::{instruction as token_instruction, state::Account as TokenAccount};

const TOKEN_MINT: &str = "CHsr7wmU7yGNaQdjFXDKCJDp1CcwJqPfuRAKWdyFsDYY";
const USDT_MINT: &str = "Es9vMFrzaHg5Xv1J7UiycDW2j2MK5Yt7yB4k8UzZfmzy";

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let user_account = next_account_info(accounts_iter)?;
    let token_account = next_account_info(accounts_iter)?;
    let sol_account = next_account_info(accounts_iter)?;
    let usdt_account = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;

    if instruction_data.len() != 1 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let action = instruction_data[0];

    match action {
        0 => buy_tokens(user_account, token_account, sol_account, usdt_account, token_program),
        1 => sell_tokens(user_account, token_account, sol_account, usdt_account, token_program),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

fn buy_tokens(
    user_account: &AccountInfo,
    token_account: &AccountInfo,
    sol_account: &AccountInfo,
    usdt_account: &AccountInfo,
    token_program: &AccountInfo,
) -> ProgramResult {
    msg!("User is buying tokens");

    let amount = 10_000; 
    let sol_balance = sol_account.lamports();
    let usdt_balance = TokenAccount::unpack(&usdt_account.data.borrow())?.amount; 

    if sol_balance < amount && usdt_balance < amount {
        return Err(ProgramError::InsufficientFunds);
    }

    // Вибір обміну на SOL або USDT
    if sol_balance >= amount {
        let token_transfer_instruction = token_instruction::transfer(
            token_program.key,
            sol_account.key,
            token_account.key,
            user_account.key,
            &[],
            amount,
        )?;

        invoke(
            &token_transfer_instruction,
            &[
                sol_account.clone(),
                token_account.clone(),
                user_account.clone(),
            ],
        )?;
    } else if usdt_balance >= amount {
        let token_transfer_instruction = token_instruction::transfer(
            token_program.key,
            usdt_account.key,
            token_account.key,
            user_account.key,
            &[],
            amount,
        )?;

        invoke(
            &token_transfer_instruction,
            &[
                usdt_account.clone(),
                token_account.clone(),
                user_account.clone(),
            ],
        )?;
    }

    msg!("BUY finished");
    Ok(())
}

fn sell_tokens(
    user_account: &AccountInfo,
    token_account: &AccountInfo,
    sol_account: &AccountInfo,
    usdt_account: &AccountInfo,
    token_program: &AccountInfo,
) -> ProgramResult {
    msg!("User is selling tokens");

    let token_balance = TokenAccount::unpack(&token_account.data.borrow())?.amount;

    if token_balance < 10_000 {
        return Err(ProgramError::InsufficientFunds);
    }

    let amount = 10_000; 

    
    let token_transfer_instruction = token_instruction::transfer(
        token_program.key,
        token_account.key,
        sol_account.key,
        user_account.key,
        &[],
        amount,
    )?;

    invoke(
        &token_transfer_instruction,
        &[
            token_account.clone(),
            sol_account.clone(),
            user_account.clone(),
        ],
    )?;

    msg!("Sell finished");
    Ok(())
}
