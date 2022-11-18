use serde::{Deserialize, Serialize};

use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::Write;
use std::io;

use core::fmt::Write as encode_write;

#[derive(Serialize, Deserialize, Debug)]
struct Proof {
    pi_a: Vec<u8>,
    pi_b: Vec<u8>,
    pi_c: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
struct VerifyKey {
    alpha_1: Vec<u8>,
    beta_1: Vec<u8>,
    beta_2: Vec<u8>,
    gamma_2: Vec<u8>,
    delta_1: Vec<u8>,
    delta_2: Vec<u8>,
    ic: Vec<Vec<u8>>,
}

fn main() -> Result<(), io::Error> {

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
    
    let res_a = format!("{}{}", "0x", encode_hex(&pi_a));
    let res_b = format!("{}{}", "0x", encode_hex(&pi_b));
    let res_c = format!("{}{}", "0x", encode_hex(&pi_c));

    let vkey_a = deserialized_vkey.alpha_1;
    let vkey_b1 = deserialized_vkey.beta_1;
    let vkey_b2 = deserialized_vkey.beta_2;
    let vkey_g = deserialized_vkey.gamma_2;
    let vkey_d1 = deserialized_vkey.delta_1;
    let vkey_d2 = deserialized_vkey.delta_2;
    let vkey_ic_1 = &deserialized_vkey.ic[0];
    let vkey_ic_2 = &deserialized_vkey.ic[1];

    let res_va = format!("{}{}", "0x", encode_hex(&vkey_a));
    let res_vb1 = format!("{}{}", "0x", encode_hex(&vkey_b1));
    let res_vb2 = format!("{}{}", "0x", encode_hex(&vkey_b2));
    let res_vg = format!("{}{}", "0x", encode_hex(&vkey_g));
    let res_vd1 = format!("{}{}", "0x", encode_hex(&vkey_d1));
    let res_vd2 = format!("{}{}", "0x", encode_hex(&vkey_d2));
    let res_vic1 = format!("{}{}", "0x", encode_hex(&vkey_ic_1));
    let res_vic2 = format!("{}{}", "0x", encode_hex(&vkey_ic_2));
    println!("pi_a: {}", res_a);
    println!("pi_b: {}", res_b);
    println!("pi_c: {}", res_c);
    println!("vkey_alpha: {}", res_va);
    println!("vkey_beta1: {}", res_vb1);
    println!("vkey_beta2: {}", res_vb2);
    println!("vkey_gamma: {}", res_vg);
    println!("vkey_delta1: {}", res_vd1);
    println!("vkey_delta2: {}", res_vd2);
    println!("vkey_ic_1: {}", res_vic1);
    println!("vkey_ic_2: {}", res_vic2);


    let mut file_proofa = File::create("proof_a.txt").unwrap();
    file_proofa.write(res_a.as_bytes())?;
    let mut file_proofb = File::create("proof_b.txt").unwrap();
    file_proofb.write(res_b.as_bytes())?;
    let mut file_proofc = File::create("proof_c.txt").unwrap();
    file_proofc.write(res_c.as_bytes())?;

    let mut file_vkey_a = File::create("vkey_a.txt").unwrap();
    file_vkey_a.write(res_va.as_bytes())?;
    let mut file_vkey_b1 = File::create("vkey_b1.txt").unwrap();
    file_vkey_b1.write(res_vb1.as_bytes())?;
    let mut file_vkey_b2 = File::create("vkey_b2.txt").unwrap();
    file_vkey_b2.write(res_vb2.as_bytes())?;
    let mut file_vkey_g = File::create("vkey_g.txt").unwrap();
    file_vkey_g.write(res_vg.as_bytes())?;
    let mut file_vkey_d1 = File::create("vkey_d1.txt").unwrap();
    file_vkey_d1.write(res_vd1.as_bytes())?;
    let mut file_vkey_d2 = File::create("vkey_d2.txt").unwrap();
    file_vkey_d2.write(res_vd2.as_bytes())?;
    let mut file_vkey_ic_1 = File::create("vkey_ic_1.txt").unwrap();
    file_vkey_ic_1.write(res_vic1.as_bytes())?;
    let mut file_vkey_ic_2 = File::create("vkey_ic_2.txt").unwrap();
    file_vkey_ic_2.write(res_vic2.as_bytes())?;

    Ok(())

}
pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}