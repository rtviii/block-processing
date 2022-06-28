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
    pub num_entered_as_signed   : u64,
    pub num_entered_as_readwrite: u64,
    pub num_entered_as_readonly : u64,

    pub tx_top_mentions       : u64,
    pub unique_in_instructions: u64,
    pub all_in_instructions   : u64,

    pub is_program: bool,

    // If it's a program 
    pub num_call_to      : u64,
    pub num_input_accs_ix: Vec<u8>,
    pub arg_data         : Vec<Vec<u8>>,

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
        let block_accounts    = HashMap::<String, AccountProfile>::new();

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



pub fn process_instruction(tx_accs:&[&str], tx_hm: &mut HashMap<&str,AccountProfile > ,ix:&Value)->Result<(),DeserializationError>{
// ! Remember to -1 the pid indexes

    let ix_acc_inds  = ix["accounts"].as_array().ok_or(DeserializationError{msg:"couldn't get accoint_ids".to_string(), ..Default::default()})?
    .iter().map(|id| id.as_u64().unwrap()).collect::<Vec<u64>>();

    let ix_data    = ix["data"].as_str().ok_or(DeserializationError{msg:"couldn't get ix data ".to_string(), ..Default::default()})?;
    let ix_pid_ind = ix["programIdIndex"].as_u64().ok_or(DeserializationError{msg:"couldn't get prog index ".to_string(), ..Default::default()})?;

    for acc_index in ix_acc_inds{
        println!("{}th account in tx_accs({:?}) is {}", acc_index, tx_accs, tx_accs[acc_index as usize - 1]);
    }
    
    // tx_hm.entry().or_insert(Default::default());


    Ok(())
}


/// Extracting data from a single transaction
/// # Arguments
/// * `tx` - the transaction to extract data from
/// # Returns
/// * `Result<HashMap<String, AccountProfile>, DeserializationError>` - the hashmap for this particular tx to be merged with a per-block one
pub fn tx_extract_accdata (tx:&Value) -> Result<HashMap<String, AccountProfile>, DeserializationError>{

    let mut account_list: Vec<&str>    = tx["message"]["accountKeys"]
    .as_array().ok_or(DeserializationError{msg:"couldn't get account_list".to_string(), ..Default::default()})?
    .iter().map(|x| x.as_str().unwrap()).collect::<Vec<&str>>();

    let mut hm_per_tx  = HashMap::new();
    for acc in account_list.iter(){
        hm_per_tx.insert(acc.to_string(), AccountProfile::default());
        
    }


    let ixs             = tx["message"]["instructions"].as_array().ok_or(DeserializationError{msg:"couldn't get instructions".to_string(), ..Default::default()})?;
    // let recentBlockhash = tx["message"]["recentBlockhash"].as_array().ok_or(DeserializationError{msg:"couldn't get recent blockhash".to_string(), ..Default::default()})?;
    // let header          = tx["message"]["header"].as_array().ok_or(DeserializationError{msg:"couldn't get the header".to_string(), ..Default::default()})?;





    for ix in ixs {
        // println!("Instruction : {}", ix);
        process_instruction(account_list.as_slice(), &mut HashMap::new(), &ix)?;
        panic!()
    }





    Ok(hm_per_tx)
}




// TODO : Compute Budget calculations  ( https://github.com/solana-labs/solana/blob/090e11210aa7222d8295610a6ccac4acda711bb9/program-runtime/src/compute_budget.rs#L26-L87)
