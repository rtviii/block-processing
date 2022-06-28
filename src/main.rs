use serde_json::Value;
use serde::{Serialize,Deserialize};
use solana_sdk::pubkey::Pubkey;
use std::{
    collections::HashMap,
    fs::{read_dir, File, ReadDir},
    io::{self, BufReader, Read},
    path::PathBuf,
    vec, sync::Arc,
};

pub fn str_is_pda(acc: &&str) -> Result<bool, bs58::decode::Error> {
    let bytes = bs58::decode(acc).into_vec()?;
    Ok(Pubkey::new(&bytes).is_on_curve())
}




#[derive(Default,Debug)]
pub struct AccountProfile {
    pub num_entered_as_signed_rw  : u64,
    pub num_entered_as_signed_r   : u64,
    pub num_entered_as_unsigned_rw: u64,
    pub num_entered_as_unsigned_r : u64,

    pub tx_top_mentions       : u64,
    pub unique_in_instructions: u64,
    pub all_in_instructions   : u64,

    pub is_pda    : bool,
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

            // TODO: Whether balances have changed: tally the difference in meta tag
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
    

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SolanaMessageHeader {
    pub numReadonlySignedAccounts  : u8,
    pub numReadonlyUnsignedAccounts: u8,
    pub numRequiredSignatures      : u8,
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

    let header                = tx["message"]["header"].as_object().ok_or(DeserializationError{msg:"couldn't get header".to_string(), ..Default::default()})?;
    let num_readonly_signed   = header.get("numReadonlySignedAccounts").unwrap().as_f64().ok_or(DeserializationError::default())? as usize;
    let num_readonly_unsigned = header.get("numReadonlyUnsignedAccounts").unwrap().as_f64().ok_or(DeserializationError::default())? as usize;
    let num_signatures        = header.get("numRequiredSignatures").unwrap().as_f64().ok_or(DeserializationError::default())? as usize;

    // Split the account indexes into Signed/Unsigned and R/W slices.
    let acc_range       = (0..account_list.len()).collect::<Vec<usize>>();
    let ( S, U )    = acc_range.split_at(num_signatures as usize);
    let ( Srw, Sr ) = S.split_at(S.len() - num_readonly_signed as usize);
    let ( Urw, Ur ) = U.split_at(U.len() - num_readonly_unsigned as usize);
   
    for i in acc_range{
    
        let mut acc_profile = AccountProfile{..Default::default()};
        
        if (Srw.contains(&i)){
            acc_profile.num_entered_as_signed_rw+=1
        }else if (Sr.contains(&i)) {
            acc_profile.num_entered_as_signed_r +=1
        }
        else if (Urw.contains(&i)){
            acc_profile.num_entered_as_unsigned_rw+=1
        }
        else if (Ur.contains(&i)) {
            acc_profile.num_entered_as_unsigned_r +=1
        }
        


    }
    



    


                    // "accountKeys": [
                    //     "6aCna9ZopJJUuTijkuKLmd57tnMco8KQBH7J8ydCjT2r", // signed   r/w
                    //     "EfKB2E4kYinooGF4BFMWXgS2gZLFeBDQ2hffo9LSSN9V", // signed   r
                    //     "ChigE9pK6g4UW3skQnKFAwyGETLzEcS2RYDep77XzmJt", // unsigned r/w
                    //     "SysvarS1otHashes111111111111111111111111111",  // unsigned r
                    //     "SysvarC1ock11111111111111111111111111111111",  // unsigned r
                    //     "Vote111111111111111111111111111111111111111"   // unsigned r
                    // ],
                    // "header": {
                    //     "numReadonlySignedAccounts"  : 1,   // <-- of those requiring signatures
                    //     "numReadonlyUnsignedAccounts": 3,   // <-- last
                    //     "numRequiredSignatures"      : 2    // <-- first
                    // },




    for ( i,acc ) in account_list.iter().enumerate(){
        println!("Iterating over account index {} : {}", i, acc);
        let mut acc_profile = AccountProfile{..Default::default()};
        

        let signable = []

    



        if i < num_signatures  && i < num_readonly_signed {

        }

        
        // hm_per_tx.insert(acc.to_string());
    }

    let ixs = tx["message"]["instructions"].as_array().ok_or(DeserializationError{msg:"couldn't get instructions".to_string(), ..Default::default()})?;
    // let recentBlockhash = tx["message"]["recentBlockhash"].as_array().ok_or(DeserializationError{msg:"couldn't get recent blockhash".to_string(), ..Default::default()})?;




    for ix in ixs {
        // println!("Instruction : {}", ix);
        process_instruction(account_list.as_slice(), &mut HashMap::new(), &ix)?;
        panic!()
    }





    Ok(hm_per_tx)
}




// TODO : Compute Budget calculations  ( https://github.com/solana-labs/solana/blob/090e11210aa7222d8295610a6ccac4acda711bb9/program-runtime/src/compute_budget.rs#L26-L87)
