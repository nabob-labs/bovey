pub use bovey_transaction_status_client_types::ParsedInstruction;
use {
    crate::{
        extract_memos::{ocp_memo_id_v1, ocp_memo_id_v3},
        parse_address_lookup_table::parse_address_lookup_table,
        parse_associated_token::{parse_associated_token, ocp_associated_token_id},
        parse_bpf_loader::{parse_bpf_loader, parse_bpf_upgradeable_loader},
        parse_stake::parse_stake,
        parse_system::parse_system,
        parse_token::parse_token,
        parse_vote::parse_vote,
    },
    inflector::Inflector,
    serde_json::Value,
    bovey_account_decoder::parse_token::ocp_token_ids,
    bovey_message::{compiled_instruction::CompiledInstruction, AccountKeys},
    bovey_pubkey::Pubkey,
    bovey_sdk_ids::{address_lookup_table, stake, system_program, vote},
    std::{
        collections::HashMap,
        str::{from_utf8, Utf8Error},
    },
    thiserror::Error,
};

lazy_static! {
    static ref ADDRESS_LOOKUP_PROGRAM_ID: Pubkey = address_lookup_table::id();
    static ref ASSOCIATED_TOKEN_PROGRAM_ID: Pubkey = ocp_associated_token_id();
    static ref BPF_LOADER_PROGRAM_ID: Pubkey = bovey_sdk_ids::bpf_loader::id();
    static ref BPF_UPGRADEABLE_LOADER_PROGRAM_ID: Pubkey =
        bovey_sdk_ids::bpf_loader_upgradeable::id();
    static ref MEMO_V1_PROGRAM_ID: Pubkey = ocp_memo_id_v1();
    static ref MEMO_V3_PROGRAM_ID: Pubkey = ocp_memo_id_v3();
    static ref STAKE_PROGRAM_ID: Pubkey = stake::id();
    static ref SYSTEM_PROGRAM_ID: Pubkey = system_program::id();
    static ref VOTE_PROGRAM_ID: Pubkey = vote::id();
    static ref PARSABLE_PROGRAM_IDS: HashMap<Pubkey, ParsableProgram> = {
        let mut m = HashMap::new();
        m.insert(
            *ADDRESS_LOOKUP_PROGRAM_ID,
            ParsableProgram::AddressLookupTable,
        );
        m.insert(
            *ASSOCIATED_TOKEN_PROGRAM_ID,
            ParsableProgram::OcpAssociatedTokenAccount,
        );
        m.insert(*MEMO_V1_PROGRAM_ID, ParsableProgram::OcpMemo);
        m.insert(*MEMO_V3_PROGRAM_ID, ParsableProgram::OcpMemo);
        for ocp_token_id in ocp_token_ids() {
            m.insert(ocp_token_id, ParsableProgram::OcpToken);
        }
        m.insert(*BPF_LOADER_PROGRAM_ID, ParsableProgram::BpfLoader);
        m.insert(
            *BPF_UPGRADEABLE_LOADER_PROGRAM_ID,
            ParsableProgram::BpfUpgradeableLoader,
        );
        m.insert(*STAKE_PROGRAM_ID, ParsableProgram::Stake);
        m.insert(*SYSTEM_PROGRAM_ID, ParsableProgram::System);
        m.insert(*VOTE_PROGRAM_ID, ParsableProgram::Vote);
        m
    };
}

#[derive(Error, Debug)]
pub enum ParseInstructionError {
    #[error("{0:?} instruction not parsable")]
    InstructionNotParsable(ParsableProgram),

    #[error("{0:?} instruction key mismatch")]
    InstructionKeyMismatch(ParsableProgram),

    #[error("Program not parsable")]
    ProgramNotParsable,

    #[error("Internal error, please report")]
    SerdeJsonError(#[from] serde_json::error::Error),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ParsedInstructionEnum {
    #[serde(rename = "type")]
    pub instruction_type: String,
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub info: Value,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ParsableProgram {
    AddressLookupTable,
    OcpAssociatedTokenAccount,
    OcpMemo,
    OcpToken,
    BpfLoader,
    BpfUpgradeableLoader,
    Stake,
    System,
    Vote,
}

pub fn parse(
    program_id: &Pubkey,
    instruction: &CompiledInstruction,
    account_keys: &AccountKeys,
    stack_height: Option<u32>,
) -> Result<ParsedInstruction, ParseInstructionError> {
    let program_name = PARSABLE_PROGRAM_IDS
        .get(program_id)
        .ok_or(ParseInstructionError::ProgramNotParsable)?;
    let parsed_json = match program_name {
        ParsableProgram::AddressLookupTable => {
            serde_json::to_value(parse_address_lookup_table(instruction, account_keys)?)?
        }
        ParsableProgram::OcpAssociatedTokenAccount => {
            serde_json::to_value(parse_associated_token(instruction, account_keys)?)?
        }
        ParsableProgram::OcpMemo => parse_memo(instruction)?,
        ParsableProgram::OcpToken => serde_json::to_value(parse_token(instruction, account_keys)?)?,
        ParsableProgram::BpfLoader => {
            serde_json::to_value(parse_bpf_loader(instruction, account_keys)?)?
        }
        ParsableProgram::BpfUpgradeableLoader => {
            serde_json::to_value(parse_bpf_upgradeable_loader(instruction, account_keys)?)?
        }
        ParsableProgram::Stake => serde_json::to_value(parse_stake(instruction, account_keys)?)?,
        ParsableProgram::System => serde_json::to_value(parse_system(instruction, account_keys)?)?,
        ParsableProgram::Vote => serde_json::to_value(parse_vote(instruction, account_keys)?)?,
    };
    Ok(ParsedInstruction {
        program: format!("{program_name:?}").to_kebab_case(),
        program_id: program_id.to_string(),
        parsed: parsed_json,
        stack_height,
    })
}

fn parse_memo(instruction: &CompiledInstruction) -> Result<Value, ParseInstructionError> {
    parse_memo_data(&instruction.data)
        .map(Value::String)
        .map_err(|_| ParseInstructionError::InstructionNotParsable(ParsableProgram::OcpMemo))
}

pub fn parse_memo_data(data: &[u8]) -> Result<String, Utf8Error> {
    from_utf8(data).map(|s| s.to_string())
}

pub(crate) fn check_num_accounts(
    accounts: &[u8],
    num: usize,
    parsable_program: ParsableProgram,
) -> Result<(), ParseInstructionError> {
    if accounts.len() < num {
        Err(ParseInstructionError::InstructionKeyMismatch(
            parsable_program,
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use {super::*, serde_json::json};

    #[test]
    fn test_parse() {
        let no_keys = AccountKeys::new(&[], None);
        let memo_instruction = CompiledInstruction {
            program_id_index: 0,
            accounts: vec![],
            data: vec![240, 159, 166, 150],
        };
        assert_eq!(
            parse(&MEMO_V1_PROGRAM_ID, &memo_instruction, &no_keys, None).unwrap(),
            ParsedInstruction {
                program: "ocp-memo".to_string(),
                program_id: MEMO_V1_PROGRAM_ID.to_string(),
                parsed: json!("🦖"),
                stack_height: None,
            }
        );
        assert_eq!(
            parse(&MEMO_V3_PROGRAM_ID, &memo_instruction, &no_keys, Some(1)).unwrap(),
            ParsedInstruction {
                program: "ocp-memo".to_string(),
                program_id: MEMO_V3_PROGRAM_ID.to_string(),
                parsed: json!("🦖"),
                stack_height: Some(1),
            }
        );

        let non_parsable_program_id = Pubkey::from([1; 32]);
        assert!(parse(&non_parsable_program_id, &memo_instruction, &no_keys, None).is_err());
    }

    #[test]
    fn test_parse_memo() {
        let good_memo = "good memo".to_string();
        assert_eq!(
            parse_memo(&CompiledInstruction {
                program_id_index: 0,
                accounts: vec![],
                data: good_memo.as_bytes().to_vec(),
            })
            .unwrap(),
            Value::String(good_memo),
        );

        let bad_memo = vec![128u8];
        assert!(std::str::from_utf8(&bad_memo).is_err());
        assert!(parse_memo(&CompiledInstruction {
            program_id_index: 0,
            data: bad_memo,
            accounts: vec![],
        })
        .is_err(),);
    }
}
