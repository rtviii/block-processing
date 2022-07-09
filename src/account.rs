
fn main() -> io::Result<()> {
    use std::path::PathBuf;

    // let datapath = Path::new("").join(std::env::current_exe().unwrap()).join("samples");
    let datapath = "/home/rxz/dev/block-processing/samples";
    println!("attemptint to ope m {}", datapath);

    let mut rd = read_dir(datapath)?
        .map(|readdir| readdir.map(|p| p.path()))
        .collect::<io::Result<Vec<PathBuf>>>()?;

    let mut hmglobal: HashMap<&str, AccountProfile> = HashMap::new();

    // -------------------
    for blockpath in rd.iter() {

        println!("------------------------------------------------------------------");
        println!("Processing blockpath: {}", blockpath.display());

        let mut reader = BufReader::new(File::open(blockpath)?);
        let mut block = String::new();
        reader.read_to_string(&mut block);
        let mut block_parsed: Value = serde_json::from_str(&block)?;


        for tx in block_parsed["transactions"].as_array_mut().unwrap().iter() {
            tx_extract_accdata(&tx["transaction"], &mut hmglobal);
        }
        println!("Global HM :{:#?}", &hmglobal);

    };
    // -----
    serde_json::to_writer(&File::create("global_map.json")?, &hmglobal);

    Ok(())
}


// TODO : Compute Budget calculations  ( https://github.com/solana-labs/solana/blob/090e11210aa7222d8295610a6ccac4acda711bb9/program-runtime/src/compute_budget.rs#L26-L87)
// pub fn main() -> io::Result<()> {
//     use std::path::{ PathBuf};

//     // let datapath = Path::new("").join(std::env::current_exe().unwrap()).join("samples");
//     let datapath = "/home/rxz/dev/block-processing/samples";
//     println!("attemptint to ope m {}", datapath);

//     let mut rd = read_dir(datapath)?
//         .map(|readdir| readdir.map(|p| p.path()))
//         .collect::<io::Result<Vec<PathBuf>>>()?;

//     for blockpath in rd.iter() {
//         let block_acc_profile = 0;
//         let block_accounts = HashMap::<String, AccountProfile>::new();

//         println!("------------------------------------------------------------------");
//         println!("Processing blockpath: {}", blockpath.display());
//         let mut reader                = BufReader::new(File::open(blockpath)?);
//         let mut block                 = String::new();
//         let _                         = reader.read_to_string(&mut block);
//         let block: Value              = serde_json::from_str(&block)?;

//         let mut hmglobal: HashMap<&str, AccountProfile> = HashMap::new();

//         // println!("{:#?}", block["transactions"].as_array().iter());
//         for tx in block["transactions"].as_array().unwrap() {
//             // TODO: Whether balances have changed: tally the difference in meta tag
//             match tx_extract_accdata(&tx["transaction"],hmglobal ) {
//                 Ok(hm) => {}
//                 Err(e) => {
//                     println!("{:?}", e);
//                 }
//             };

//         }
//     }

//     Ok(())
// }
