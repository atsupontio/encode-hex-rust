
fn main() {
use hex;

    const NUM_BLINDING_BITS: usize = 128;
    
    let id = "1d7361b1-5018-4043-804b-7c589bdabc31";
    let secret = "123456";
    let nonce = "afe387d2";

    // if secret.len() % 2 != 1 {

    // }
    
    let new_id: String = id.split('-').collect();
    println!("new_id: {:?}", new_id);
    
    let id_bytes = hex::decode(&new_id).unwrap();
    // let a: Vec<bool> = id_bytes.iter().map(|e| Some(e)).collect();
    println!("bytes: {:?}", id_bytes);
    let a = hexdump(&id_bytes);
    let secret_bytes = hex::decode(&secret).unwrap();
    println!("secret: {:?}", secret_bytes);
    hexdump(&secret_bytes);
    let nonce_bytes = hex::decode(&nonce).unwrap();
    println!("nonce:{:?}", nonce_bytes);
    hexdump(&nonce_bytes);

    // let old_blinding_bits: Vec<bool> = (0..NUM_BLINDING_BITS).map(|_| preimage).collect();
}

fn hexdump(bytes: &[u8]) -> Vec<String> {

    let mut v: Vec<String> = Vec::new();
    

    println!("{} bytes:", bytes.len());
    for (i, b) in bytes.iter().enumerate() {
        // b: &u8 の値を2桁の16進数で表示する
        print!("{:?}", b);
        let a = format!("{:b}", b);
        v.push(a);

        // 値を16個表示するごとに改行する
        if (i + 1) % 16 == 0 {
            println!();
        }
    }
    println!();
    v
}