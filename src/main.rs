use anyhow::Result;
use fastcrypto::encoding::Base64;
use fastcrypto_zkp::bn254::zk_login_api::ZkLoginEnv;
use im::hashmap::HashMap as ImHashMap;
use std::env;
use sui_types::signature::VerifyParams;
use sui_types::transaction::{Transaction, TransactionData};
use sui_types::{crypto::ToFromBytes, signature::GenericSignature};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let tx_base64 = Base64::try_from(String::from(args[1].as_str()))?;
    // let base64_str = "AAAAAQBx9TX5AXir029unc+GnJKEbLXvPEwrnhWfp5SaofG+XQdjb3VudGVyBmNyZWF0ZQAAZClXrvQHUxG/j+g9pAoay1J7G6fof+VWR8LrxlADzvsBMdEp2sqzSN0bexRB7jIUas+xp9UwCLwkRjjZi6CuNO5HJw4AAAAAACCjIIQXqPcO8JQESWTYVdvNQwodBjBnN52Wj5lLy/7qJmQpV670B1MRv4/oPaQKGstSexun6H/lVkfC68ZQA8776AMAAAAAAADwD0YAAAAAAAA=";
    // let tx_base64 = Base64::try_from(String::from(base64_str)).unwrap();
    // let signature_str = "ALJCPPlDe2pc6+Er9Mh/BNQhaK5nScMX0B+Vu674cIQM23GL1WlALg41Vn5km6EwR9dWjQcvkEDof+bSpx2PiwOX/NYXhh+8Elx25+t+Ung/9uoKXk3xB+6n90u5REwfuw==";
    // let signature_base64 = Base64::try_from(String::from(signature_str)).unwrap();
    let data: TransactionData = bcs::from_bytes(&tx_base64.to_vec()?)?;
    let data_str = serde_json::to_string(&data.clone())?;
    if args.len() == 3 {
        let signature_base64 = Base64::try_from(String::from(args[2].as_str()))?;
        let sign = GenericSignature::from_bytes(&signature_base64.to_vec().unwrap()).unwrap();
        let mut sigs = Vec::new();
        sigs.push(sign);
        let txn = Transaction::from_generic_sig_data(data, sigs);
        let verify_params =
            VerifyParams::new(ImHashMap::new(), vec![], ZkLoginEnv::default(), true, true);
        txn.verify_signature(&verify_params).unwrap();
    }
    print!("{}", data_str);
    Ok(())
}
