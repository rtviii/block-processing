use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use std::{
    collections::HashMap,
    fs::{read_dir, File, ReadDir},
    io::{self, BufReader, Read},
    path::PathBuf,
    vec,
};

pub fn str_is_pda(acc: &&str) -> Result<bool, bs58::decode::Error> {
    let bytes = bs58::decode(acc).into_vec()?;
    Ok(Pubkey::new(&bytes).is_on_curve())
}

pub struct AccountProfile {
    pub is_program: bool,

    pub mentions: u64,
    pub in_transactions: u64,
    pub in_instructions: u64,
    pub call_to: u64,

    pub arg_data_size: u64,
    pub num_accounts_ix: u64,

    pub num_entered_as_signed: u64,
    pub num_entered_as_readwrite: u64,
    pub num_entered_as_readonly: u64,
}



pub fn main() -> io::Result<()> {

    use std::path::{ PathBuf, Path };

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

        // println!("{:#?}", block["transactions"].as_array().iter());
        for tx in block["transactions"].as_array().unwrap() {
            let accounts = tx["transaction"]["message"]["accountKeys"]
                .as_array()
                .unwrap();
            let ixs = tx["transaction"]["message"]["instructions"]
                .as_array()
                .unwrap();

            let num_signers = tx["transaction"]["message"]["header"]["numRequiredSignatures"]
                .as_i64()
                .map_or(-1, |x| x as i64);
            let num_signatures = tx["transaction"]["signatures"]
                .as_array()
                .map_or(-1, |x| x.len() as i64);

            // for ix in ixs {
            //     let programIdIndex = ix["programIdIndex"].as_u64().unwrap();
            //     println!("Program index is {} and the account there is {}", programIdIndex, accounts[programIdIndex as usize]);
            // }

            // for account in accounts {
            //     let acc = account.as_str().unwrap();
            //     println!("Acc: {:?} is on curve: {:?}", &acc, str_is_pda(&acc).unwrap());
            // }
        }
    }

    Ok(())
}

#[derive(Default)]
pub struct DeserializationError<'a>{
    pub msg: &'a str,
    pub tx_sig      : Option<String>,
    pub block_height: Option<String>,
}

pub fn tx_extract_accdata (tx:&Value) -> Result<HashMap<String, AccountProfile>, DeserializationError>{
    let account_list =  tx["transaction"]["message"]["accountKeys"].as_array().ok_or(DeserializationError{msg:"couldn't deserialize transaction", ..Default::default()})?;
    println!("{:#?}", account_list);
    return Ok(HashMap::new());
}




// TODO : Compute Budget calculations  ( https://github.com/solana-labs/solana/blob/090e11210aa7222d8295610a6ccac4acda711bb9/program-runtime/src/compute_budget.rs#L26-L87)
