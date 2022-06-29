use serde::{Deserialize, Serialize};
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use std::{
    collections::HashMap,
    fs::{read_dir, File},
    io::{self, BufReader, Read}, vec, fmt::Display,
};

pub fn str_is_pda(acc: &&str) -> Result<bool, bs58::decode::Error> {
    let bytes = bs58::decode(acc).into_vec()?;
    Ok(Pubkey::new(&bytes).is_on_curve())
}

pub struct DataFreq (u64,u64);
struct average_length {
   pub total_length: u64,
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

    pub is_pda: bool,
    pub is_program: bool,
    // TODO: program methods 

    // If it's a program
    pub method_freq      : Vec<u64>,
    pub num_call_to      : u64,
    pub num_input_accs_ix: Vec<u8>,
    // // pub arg_data         : Vec<&'a [u8]>,
    // pub arg_data         : DataFreq
}


#[derive(Default, Debug)]
pub struct DeserializationError {
    pub msg: String,
    pub tx_sig: Option<String>,
    pub block_height: Option<String>,
}

pub fn process_instruction<'a>(
    tx_accs: &[&'a str],
    tx_hm: &mut HashMap<&'a str, AccountProfile<'a>>,
    ix: &'a Value,
) -> Result<(), DeserializationError> {
    // ! Remember to -1 the pid indexes

    let acc_inds = ix["accounts"]
        .as_array()
        .ok_or(DeserializationError {
            msg: "couldn't get accoint_ids".to_string(),
            ..Default::default()
        })?
        .iter()
        .map(|id| id.as_u64().unwrap())
        .collect::<Vec<u64>>();
    let data = ix["data"]
        .as_str()
        .ok_or(DeserializationError {
            msg: "couldn't get ix data ".to_string(),
            ..Default::default()
        })?
        .as_bytes();
    let pid_ind = ix["programIdIndex"].as_u64().ok_or(DeserializationError {
        msg: "couldn't get prog index ".to_string(),
        ..Default::default()
    })?;

    let program = tx_accs[pid_ind as usize];
    if let Some(prog_profile) = tx_hm.get_mut(program) {
        prog_profile.ix_mentions += 1;
        prog_profile.is_program = true;
        prog_profile.num_call_to += 1;
        prog_profile.num_call_to += 1;
        prog_profile.arg_data.push(data);
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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SolanaMessageHeader {
    pub numReadonlySignedAccounts: u8,
    pub numReadonlyUnsignedAccounts: u8,
    pub numRequiredSignatures: u8,
}

/// Extracting data from a single transaction
/// # Arguments
/// * `tx` - the transaction to extract data from
/// # Returns
/// * `Result<HashMap<String, AccountProfile>, DeserializationError>` - the hashmap for this particular tx to be merged with a per-block one
pub fn tx_extract_accdata(
    tx: &Value,
) -> Result<HashMap<&str, AccountProfile>, DeserializationError> {

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

        hm_per_tx.insert(account_list[*i], acc_profile);
    }

    println!("{:?}", hm_per_tx);
    // panic!("\n\n\nDeliberately stopped: <>:<><><><><");
    // each instruction updates the transaction hashmap separately
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
    Ok(hm_per_tx)
}

// TODO : Compute Budget calculations  ( https://github.com/solana-labs/solana/blob/090e11210aa7222d8295610a6ccac4acda711bb9/program-runtime/src/compute_budget.rs#L26-L87)
pub fn main() -> io::Result<()> {
    use std::path::{ PathBuf};

    // let datapath = Path::new("").join(std::env::current_exe().unwrap()).join("samples");
    let datapath = "/home/rxz/dev/block-processing/samples";
    println!("attemptint to ope m {}", datapath);

    let mut rd = read_dir(datapath)?
        .map(|readdir| readdir.map(|p| p.path()))
        .collect::<io::Result<Vec<PathBuf>>>()?;

    for blockpath in rd.iter() {
        let block_acc_profile = 0;
        let block_accounts = HashMap::<String, AccountProfile>::new();

        println!("------------------------------------------------------------------");
        println!("Processing blockpath: {}", blockpath.display());
        let mut reader = BufReader::new(File::open(blockpath)?);
        let mut block = String::new();
        let _ = reader.read_to_string(&mut block);
        let block: Value = serde_json::from_str(&block)?;



        let mut txhms:Vec<HashMap<&str, AccountProfile>> = vec![];
        // println!("{:#?}", block["transactions"].as_array().iter());
        for tx in block["transactions"].as_array().unwrap() {
            // TODO: Whether balances have changed: tally the difference in meta tag
            match tx_extract_accdata(&tx["transaction"]) {
                Ok(hm) => {
                    // println!("tx hm :{:?}", hm)
                   txhms.push(hm);
                   if txhms.len() > 6{
                        merge_hmaps(txhms[0], txhms[1]);
                        panic!("Accumulated 6 hmaps");
                   }
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            };

        }
    }

    Ok(())
}
