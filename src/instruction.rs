use serde::{Deserialize, Serialize};
use serde_json::Value;
use solana_sdk::{pubkey::Pubkey, blake3::Hash};
use std::{
    collections::HashMap,
    fs::{read_dir, File},
    io::{self, BufReader, Read}, vec, fmt::Display,
};
use hex::{encode, decode};

use crate::transaction::{DeserializationError, AccountProfile, str_is_pda};




pub fn process_instruction<'a>(
    tx_accs: &[&'a str],
    tx_hm: &mut HashMap<&'a str, AccountProfile>,
    ix: &'a Value,
) -> Result<(), DeserializationError> {
    // ! Remember to -1 the pid indexes

    let pid_ind = ix["programIdIndex"].as_u64().ok_or(DeserializationError {
        msg: "couldn't get prog index ".to_string(),
        ..Default::default()
    })?;
    let program = tx_accs[pid_ind as usize];
    let data = ix["data"]
        .as_str()
        .ok_or(DeserializationError {
            msg: "couldn't get ix data ".to_string(),
            ..Default::default()
        })?;
        // .as_bytes();



    

    
    println!("program {}", program);
    println!("Data string: {:?}", data);
    println!("Data hex: {:?}",hex::encode( bs58::decode(data).into_vec().unwrap() ));
    println!("1st byte: 0x{:x}", bs58::decode(data).into_vec().unwrap()[0]);
    println!("\n\n");
    // panic!("stopped ");




    let acc_inds = ix["accounts"]
        .as_array()
        .ok_or(DeserializationError {
            msg: "couldn't get accoint_ids".to_string(),
            ..Default::default()
        })?
        .iter()
        .map(|id| id.as_u64().unwrap())
        .collect::<Vec<u64>>();


    if let Some(prog_profile) = tx_hm.get_mut(program) {
        prog_profile.ix_mentions += 1;
        prog_profile.is_program = true;
        prog_profile.num_call_to += 1;
        prog_profile.num_call_to += 1;
        // prog_profile.arg_data.push(data);
        prog_profile.num_input_accs_ix.push(acc_inds.len() as u8);
    } else {
        panic!("Program account not found in tx_hm. Logic error.");
    }

    for acc_index in acc_inds {
        if let Some(acc) = tx_hm.get_mut(tx_accs[acc_index as usize]) {
            if str_is_pda(&tx_accs[acc_index as usize]).expect("couldn't check if acc is pda") {
                acc.is_pda = true;
            };
            acc.ix_mentions += 1;
        } else {
            panic!("Account not found in tx_hm. Logic error.");
        }
    }

    Ok(())
}