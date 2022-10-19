use serde::{Deserialize, Serialize};

use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;

use core::fmt::Write;

#[derive(Serialize, Deserialize, Debug)]
struct Proof {
    pi_a: Vec<u8>,
    pi_b: Vec<u8>,
    pi_c: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
struct VerifyKey {
    alpha_1: Vec<u8>,
    beta_2: Vec<u8>,
    gamma_2: Vec<u8>,
    delta_2: Vec<u8>,
    ic: Vec<Vec<u8>>,
}

fn main() {

    let proof_file = File::open("./proof_uncompressed.json").unwrap();
    let vkey_file = File::open("vkey_uncompressed.json").unwrap();

    // let file = OpenOptions::new().read(true);

    let proof_reader = BufReader::new(proof_file);
    let vkey_reader = BufReader::new(vkey_file);

    let deserialized_proof: Proof = serde_json::from_reader(proof_reader).unwrap();
    let deserialized_vkey: VerifyKey = serde_json::from_reader(vkey_reader).unwrap();

    let pi_a = deserialized_proof.pi_a;
    let pi_b = deserialized_proof.pi_b;
    let pi_c = deserialized_proof.pi_c;
    
    let res_a = encode_hex(&pi_a);
    let res_b = encode_hex(&pi_b);
    let res_c = encode_hex(&pi_c);

    let vkey_a = deserialized_vkey.alpha_1;
    let vkey_b = deserialized_vkey.beta_2;
    let vkey_g = deserialized_vkey.gamma_2;
    let vkey_d = deserialized_vkey.delta_2;
    let vkey_ic_1 = &deserialized_vkey.ic[0];
    let vkey_ic_2 = &deserialized_vkey.ic[1];

    let res_va = encode_hex(&vkey_a);
    let res_vb = encode_hex(&vkey_b);
    let res_vg = encode_hex(&vkey_g);
    let res_vd = encode_hex(&vkey_d);
    let res_vic1 = encode_hex(&vkey_ic_1);
    let res_vic2 = encode_hex(&vkey_ic_2);
    println!("pi_a: {}", res_a);
    println!("pi_b: {}", res_b);
    println!("pi_c: {}", res_c);
    println!("vkey_alpha: {}", res_va);
    println!("vkey_beta: {}", res_vb);
    println!("vkey_gamma: {}", res_vg);
    println!("vkey_delta: {}", res_vd);
    println!("vkey_ic_1: {}", res_vic1);
    println!("vkey_ic_2: {}", res_vic2);
}
pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}