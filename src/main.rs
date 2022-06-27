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

#[derive(Default,Debug)]
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

            match tx_extract_accdata(&tx["transaction"]){
                Ok(k) =>{},
                Err(e) =>{
                    println!("{:?}", e);
                }

        };

            // let accounts = tx["transaction"]["message"]["accountKeys"]
            //     .as_array()
            //     .unwrap();
            // let ixs = tx["transaction"]["message"]["instructions"]
            //     .as_array()
            //     .unwrap();

            // let num_signers = tx["transaction"]["message"]["header"]["numRequiredSignatures"]
            //     .as_i64()
            //     .map_or(-1, |x| x as i64);
            // let num_signatures = tx["transaction"]["signatures"]
            //     .as_array()
            //     .map_or(-1, |x| x.len() as i64);

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

#[derive(Default, Debug)]
pub struct DeserializationError{
    pub msg: String,
    pub tx_sig      : Option<String>,
    pub block_height: Option<String>,
}


pub fn process_instruction(tx_accs:&Vec<&str>, tx_hm: &mut HashMap<&str,AccountProfile > ,ix:&Value)->Result<u8,DeserializationError>{

    let acc_ids  = ix["accounts"].as_array().ok_or(DeserializationError{msg:"couldn't get accoint_ids".to_string(), ..Default::default()})?
    .iter().map(|id| id.as_u64().unwrap()).collect::<Vec<u64>>();

    let ixdata   = ix["data"].as_str().ok_or(DeserializationError{msg:"couldn't get ix data ".to_string(), ..Default::default()})?;
    let pidindex = ix["programIdIndex"].as_u64().ok_or(DeserializationError{msg:"couldn't get prog index ".to_string(), ..Default::default()})?;


    // println!("Got accountlist : {:?}", tx_accs);
    // println!("indexed with pid {} : {:?}", pidindex, tx_accs[pidindex as usize-1]);
    // acc_ids[pidindex as usize].to_string();

    println!("tx accs : {:?} | pidid :{:?}", tx_accs.len(), pidindex);
    println!("tx accs : {:?} | pidid :{:?}", tx_accs.len(), pidindex);
    if (tx_accs.len() == pidindex as usize) {
        panic!("pidindex out of bounds");
    }

    Ok(0)
}


pub fn tx_extract_accdata (tx:&Value) -> Result<HashMap<String, AccountProfile>, DeserializationError>{
    // let account_list    = tx["transaction"]["message"]["accountKeys"].as_array().ok_or(DeserializationError{msg:"couldn't get transaction", ..Default::default()})?;

    let mut account_list    = tx["message"]["accountKeys"]
    .as_array().ok_or(DeserializationError{msg:"couldn't get account_list".to_string(), ..Default::default()})?
    .iter().map(|x| x.as_str().unwrap()).collect::<Vec<&str>>();

    // println!(" Acc list is {:#?}", account_list);

    let ixs             = tx["message"]["instructions"].as_array().ok_or(DeserializationError{msg:"couldn't get instructions".to_string(), ..Default::default()})?;
    // let recentBlockhash = tx["message"]["recentBlockhash"].as_array().ok_or(DeserializationError{msg:"couldn't get recent blockhash".to_string(), ..Default::default()})?;
    // let header          = tx["message"]["header"].as_array().ok_or(DeserializationError{msg:"couldn't get the header".to_string(), ..Default::default()})?;


    let mut hm_per_tx  = HashMap::new();
    // for acc in account_list{
    //     hm_per_tx.insert(acc.to_string(), AccountProfile::default());
    // }


    for ix in ixs {
        // println!("Instruction : {}", ix);
        process_instruction(&account_list, &mut HashMap::new(), &ix)?;
    }





    Ok(hm_per_tx)
}




// TODO : Compute Budget calculations  ( https://github.com/solana-labs/solana/blob/090e11210aa7222d8295610a6ccac4acda711bb9/program-runtime/src/compute_budget.rs#L26-L87)
