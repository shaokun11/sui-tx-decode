use anyhow::Result;
use fastcrypto::encoding::{Base64, Encoding};
use fastcrypto::signature_service::SignatureService;
use fastcrypto_zkp::bn254::zk_login_api::ZkLoginEnv;
use hex;
use im::hashmap::HashMap as ImHashMap;
use std::env;
use sui_types::message_envelope::Message;
use sui_types::signature::{AuthenticatorTrait, VerifyParams};
use sui_types::transaction::{
    InputObjectKind, Transaction, TransactionData, TransactionDataAPI, TransactionKind,
};
use sui_types::{crypto::ToFromBytes, signature::GenericSignature};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let args: Vec<String> = env::args().collect();
    // let tx_base64 = Base64::try_from(String::from(args[1].as_str()))?;
    let base64_str = "AAAAAQBx9TX5AXir029unc+GnJKEbLXvPEwrnhWfp5SaofG+XQdjb3VudGVyBmNyZWF0ZQAAZClXrvQHUxG/j+g9pAoay1J7G6fof+VWR8LrxlADzvsBMdEp2sqzSN0bexRB7jIUas+xp9UwCLwkRjjZi6CuNO5HJw4AAAAAACCjIIQXqPcO8JQESWTYVdvNQwodBjBnN52Wj5lLy/7qJmQpV670B1MRv4/oPaQKGstSexun6H/lVkfC68ZQA8776AMAAAAAAADwD0YAAAAAAAA=";
    let tx_base64 = Base64::try_from(String::from(base64_str)).unwrap();
    let data: TransactionData = bcs::from_bytes(&tx_base64.to_vec()?)?;
    // println!("{:?}", serde_json::to_string(&data)?);
    // https://github.com/MystenLabs/sui/blob/041c5f2bae2fe52079e44b70514333532d69f4e6/crates/sui-json-rpc/src/transaction_execution_api.rs#L73
    // let signature_str = "ALJCPPlDe2pc6+Er9Mh/BNQhaK5nScMX0B+Vu674cIQM23GL1WlALg41Vn5km6EwR9dWjQcvkEDof+bSpx2PiwOX/NYXhh+8Elx25+t+Ung/9uoKXk3xB+6n90u5REwfuw==";
    let signature_str = "AKD4XdltkCyBi1Heb4EJJ3lzuV3F4u7+CYeaE+Fd7qXpaT17yd4tHWjMf4CWq3TuXBLxTpkc2MV39P6p7eMV8QnqvbuA0Q1Bqu4RHV3JPpqmH+C527hWJGUBOZN1j9sg8w==";
    let signature_base64 = Base64::try_from(String::from(signature_str)).unwrap();
    let sign = GenericSignature::from_bytes(&signature_base64.to_vec().unwrap()).unwrap();
    let mut sigs = Vec::new();
    sigs.push(sign);
    let txn = Transaction::from_generic_sig_data(data, sigs);
    let verify_params =
        VerifyParams::new(ImHashMap::new(), vec![], ZkLoginEnv::default(), true, true);
    match txn.verify_signature(&verify_params) {
        Ok(_) => {
            println!("Signature verification passed");
        }
        Err(e) => {
            println!("Signature verification failed {}", e);
        }
    };
    Ok(())
}
