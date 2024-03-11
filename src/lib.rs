use fastcrypto::encoding::Base64;
use fastcrypto_zkp::bn254::zk_login_api::ZkLoginEnv;
use im::hashmap::HashMap as ImHashMap;
use sui_types::signature::VerifyParams;
use sui_types::transaction::{ Transaction, TransactionData
};
use sui_types::{crypto::ToFromBytes, signature::GenericSignature};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(tx_data: &str, signature: &str) {
    let tx_base64 = Base64::try_from(String::from(tx_data)).unwrap();
    let data: TransactionData = bcs::from_bytes(&tx_base64.to_vec()?)?;
    let signature_base64 = Base64::try_from(String::from(signature)).unwrap();
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
}
