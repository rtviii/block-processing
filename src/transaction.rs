use serde::{Deserialize, Serialize};
use serde_json::Value;
use solana_sdk::{pubkey::Pubkey, blake3::Hash};
use std::{
    collections::HashMap,
    fs::{read_dir, File},
    io::{self, BufReader, Read}, vec, fmt::Display,
};
use hex::{encode, decode};

use crate::instruction::process_instruction;

pub fn str_is_pda(acc: &&str) -> Result<bool, bs58::decode::Error> {
    let bytes = bs58::decode(acc).into_vec()?;
    Ok(Pubkey::new(&bytes).is_on_curve())
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SolanaMessageHeader {
    pub numReadonlySignedAccounts: u8,
    pub numReadonlyUnsignedAccounts: u8,
    pub numRequiredSignatures: u8,
}


#[derive(Default, Debug)]
pub struct DataFreq {
   pub total_length  : u64,
   pub num_occurences: u64
}


#[derive(Default, Debug)]
pub struct AccountProfile {
    
    pub num_entered_as_signed_rw  : u64,
    pub num_entered_as_signed_r   : u64,
    pub num_entered_as_unsigned_rw: u64,
    pub num_entered_as_unsigned_r : u64,

    /// to account for number of times this account (or program) is referred to
    /// in 'accountKeys' at the top of transactions
    pub tx_top_mentions: u64,

    /// to account for number of times this account (or program) is referred to
    /// by indexes interior to instructions(whether as pidindex or input)
    pub ix_mentions: u64,

    pub is_pda    : bool,
    pub is_program: bool,

    // If it's a program
    pub data_first_byte  : HashMap<u8, u64>,
    pub num_call_to      : u64,
    pub num_input_accs_ix: Vec<u8>,
    pub arg_data         : DataFreq
}
#[derive(Default, Debug)]
pub struct DeserializationError {
    pub msg         : String,
    pub tx_sig      : Option<String>,
    pub block_height: Option<String>,
}



/// Extracting data from a single transaction
/// # Arguments
/// * `tx` - the transaction to extract data from
/// # Returns
/// * `Result<HashMap<String, AccountProfile>, DeserializationError>` - the hashmap for this particular tx to be merged with a per-block one
pub fn tx_extract_accdata<'a>(
    tx       : &Value,
    global_hm: &mut HashMap<&'a str, AccountProfile>
) -> Result<(), DeserializationError> {

    let account_list: Vec<&str> = tx["message"]["accountKeys"]
        .as_array()
        .ok_or(DeserializationError {
            msg: "couldn't get account_list".to_string(),
            ..Default::default()
        })?
        .iter()
        .map(|x| x.as_str().unwrap())
        .collect::<Vec<&str>>();

    let mut hm_per_tx = HashMap::new();



    let header = tx["message"]["header"]
        .as_object()
        .ok_or(DeserializationError {
            msg: "couldn't get header".to_string(),
            ..Default::default()
        })?;
    let num_readonly_signed = header
        .get("numReadonlySignedAccounts")
        .unwrap()
        .as_f64()
        .ok_or(DeserializationError::default())? as usize;
    let num_readonly_unsigned = header
        .get("numReadonlyUnsignedAccounts")
        .unwrap()
        .as_f64()
        .ok_or(DeserializationError::default())? as usize;

    let num_signatures = header
        .get("numRequiredSignatures")
        .unwrap()
        .as_f64()
        .ok_or(DeserializationError::default())? as usize;

    // Split the account indexes into Signed/Unsigned and R/W slices.
    let acc_range = (0..account_list.len()).collect::<Vec<usize>>();
    let (S, U) = acc_range.split_at(num_signatures as usize);
    let (srw, sr) = S.split_at(S.len() - num_readonly_signed as usize);
    let (urw, ur) = U.split_at(U.len() - num_readonly_unsigned as usize);

    for i in acc_range.iter() {
        let mut acc_profile = AccountProfile {
            ..Default::default()
        };

        acc_profile.tx_top_mentions += 1;
        if srw.contains(i) {
            acc_profile.num_entered_as_signed_rw += 1
        } else if sr.contains(i) {
            acc_profile.num_entered_as_signed_r += 1
        } else if urw.contains(i) {
            acc_profile.num_entered_as_unsigned_rw += 1
        } else if ur.contains(i) {
            acc_profile.num_entered_as_unsigned_r += 1
        }

        if str_is_pda(&account_list[*i]).expect("couldn't check if acc is pda") {
            acc_profile.is_pda = true;
        };

        hm_per_tx.insert(account_list[*i], acc_profile);
    }


    let first_sig = tx["signatures"][0].as_str()
        .ok_or(DeserializationError {
            msg: "couldn't get signatures".to_string(),
            ..Default::default()
        })?;

    println!("\t\t\tTx first sig :{}", first_sig);

    let ixs = tx["message"]["instructions"]
        .as_array()
        .ok_or(DeserializationError {
            msg: "couldn't get instructions".to_string(),
            ..Default::default()
        })?;

    for ix in ixs {
        process_instruction(account_list.as_slice(), &mut hm_per_tx, &ix)?;
    }


    // let recentBlockhash = tx["message"]["recentBlockhash"].as_array().ok_or(DeserializationError{msg:"couldn't get recent blockhash".to_string(), ..Default::default()})?;
    Ok(())
}