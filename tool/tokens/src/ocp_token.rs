use {
    crate::{
        args::{DistributeTokensArgs, OcpTokenArgs},
        commands::{get_fee_estimate_for_messages, Error, FundingSource, TypedAllocation},
    },
    console::style,
    bovey_account_decoder::parse_token::{real_number_string, real_number_string_trimmed},
    bovey_rpc_client::rpc_client::RpcClient,
    bovey_sdk::{instruction::Instruction, message::Message, native_token::lamports_to_bov},
    ocp_associated_token_account::{
        get_associated_token_address, instruction::create_associated_token_account,
    },
    ocp_token::{
        bovey_program::program_pack::Pack,
        state::{Account as OcpTokenAccount, Mint},
    },
};

pub fn update_token_args(client: &RpcClient, args: &mut Option<OcpTokenArgs>) -> Result<(), Error> {
    if let Some(ocp_token_args) = args {
        let sender_account = client
            .get_account(&ocp_token_args.token_account_address)
            .unwrap_or_default();
        ocp_token_args.mint = OcpTokenAccount::unpack(&sender_account.data)?.mint;
        update_decimals(client, args)?;
    }
    Ok(())
}

pub fn update_decimals(client: &RpcClient, args: &mut Option<OcpTokenArgs>) -> Result<(), Error> {
    if let Some(ocp_token_args) = args {
        let mint_account = client.get_account(&ocp_token_args.mint).unwrap_or_default();
        let mint = Mint::unpack(&mint_account.data)?;
        ocp_token_args.decimals = mint.decimals;
    }
    Ok(())
}

pub(crate) fn build_ocp_token_instructions(
    allocation: &TypedAllocation,
    args: &DistributeTokensArgs,
    do_create_associated_token_account: bool,
) -> Vec<Instruction> {
    let ocp_token_args = args
        .ocp_token_args
        .as_ref()
        .expect("ocp_token_args must be some");
    let wallet_address = allocation.recipient;
    let associated_token_address =
        get_associated_token_address(&wallet_address, &ocp_token_args.mint);
    let mut instructions = vec![];
    if do_create_associated_token_account {
        instructions.push(create_associated_token_account(
            &args.fee_payer.pubkey(),
            &wallet_address,
            &ocp_token_args.mint,
            &ocp_token::id(),
        ));
    }
    instructions.push(
        ocp_token::instruction::transfer_checked(
            &ocp_token::id(),
            &ocp_token_args.token_account_address,
            &ocp_token_args.mint,
            &associated_token_address,
            &args.sender_keypair.pubkey(),
            &[],
            allocation.amount,
            ocp_token_args.decimals,
        )
        .unwrap(),
    );
    instructions
}

pub(crate) fn check_ocp_token_balances(
    messages: &[Message],
    allocations: &[TypedAllocation],
    client: &RpcClient,
    args: &DistributeTokensArgs,
    created_accounts: u64,
) -> Result<(), Error> {
    let ocp_token_args = args
        .ocp_token_args
        .as_ref()
        .expect("ocp_token_args must be some");
    let allocation_amount: u64 = allocations.iter().map(|x| x.amount).sum();
    let fees = get_fee_estimate_for_messages(messages, client)?;

    let token_account_rent_exempt_balance =
        client.get_minimum_balance_for_rent_exemption(OcpTokenAccount::LEN)?;
    let account_creation_amount = created_accounts * token_account_rent_exempt_balance;
    let fee_payer_balance = client.get_balance(&args.fee_payer.pubkey())?;
    if fee_payer_balance < fees + account_creation_amount {
        return Err(Error::InsufficientFunds(
            vec![FundingSource::FeePayer].into(),
            lamports_to_bov(fees + account_creation_amount).to_string(),
        ));
    }
    let source_token_account = client
        .get_account(&ocp_token_args.token_account_address)
        .unwrap_or_default();
    let source_token = OcpTokenAccount::unpack(&source_token_account.data)?;
    if source_token.amount < allocation_amount {
        return Err(Error::InsufficientFunds(
            vec![FundingSource::OcpTokenAccount].into(),
            real_number_string_trimmed(allocation_amount, ocp_token_args.decimals),
        ));
    }
    Ok(())
}

pub(crate) fn print_token_balances(
    client: &RpcClient,
    allocation: &TypedAllocation,
    ocp_token_args: &OcpTokenArgs,
) -> Result<(), Error> {
    let address = allocation.recipient;
    let expected = allocation.amount;
    let associated_token_address = get_associated_token_address(&address, &ocp_token_args.mint);
    let recipient_account = client
        .get_account(&associated_token_address)
        .unwrap_or_default();
    let (actual, difference) = if let Ok(recipient_token) =
        OcpTokenAccount::unpack(&recipient_account.data)
    {
        let actual_ui_amount = real_number_string(recipient_token.amount, ocp_token_args.decimals);
        let delta_string =
            real_number_string(recipient_token.amount - expected, ocp_token_args.decimals);
        (
            style(format!("{actual_ui_amount:>24}")),
            format!("{delta_string:>24}"),
        )
    } else {
        (
            style("Associated token account not yet created".to_string()).yellow(),
            "".to_string(),
        )
    };
    println!(
        "{:<44}  {:>24}  {:>24}  {:>24}",
        allocation.recipient,
        real_number_string(expected, ocp_token_args.decimals),
        actual,
        difference,
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    // The following unit tests were written for v1.4 using the ProgramTest framework, passing its
    // BanksClient into the `bovey-tokens` methods. With the revert to RpcClient in this module, that approach was no longer viable.
    // These tests were removed rather than rewritten to avoid accruing technical debt. Once a new
    // rpc/client framework is implemented, they should be restored.
    //
    // async fn test_process_ocp_token_allocations()
    // async fn test_process_ocp_token_transfer_amount_allocations()
    // async fn test_check_ocp_token_balances()
    //
}
