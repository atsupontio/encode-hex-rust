use crypto::digest::Digest;


fn main() {
use hex;
use crypto::sha2::Sha256;

    const NUM_BLINDING_BITS: usize = 128;
    
    let id = "1d7361b1-5018-4043-804b-7c589bdabc31";
    let secret = "12345678";
    let nonce = "afe387d2";
    let name = "koyamaatsuki";
    let birth = "20000510";
    let test = "5c5cf735970b8516a5541bb567f4874042f385af1ed6eb76a79d95b8d6cc047a";

    println!("{:?}", test.as_bytes());
    println!("{:?}", hex::decode(&test));

    let a = hex::decode(&test).unwrap();
    println!("{:?}", "0x".to_owned() + &hex::encode(a));

    let binding = name.clone().to_string();
    let name_bytes = binding.as_bytes();
    println!("name_bytes: {:?}", name_bytes.clone());

    print!("name_bytes: ");
    for i in name_bytes.iter() {
        print!("{:?}", i);
    }
    println!();


    let mut birth_bytes = hex::decode(&birth).unwrap();
    let birth2 = hexdump(&birth_bytes);
    println!("birth_bytes: {:?}", birth_bytes.clone());
    

    let new_id: String = id.split('-').collect();
    println!("new_id: {:?}", new_id);
    
    let mut id_bytes = hex::decode(&new_id).unwrap();
    let id = hexdump(&id_bytes);
    let mut id_bool = Vec::new();
    for (i, c) in id.chars().enumerate() {
        // do something with character `c` and index `i`
        if c == '1' {
            id_bool.push(true);
        } else {
            id_bool.push(false);
        }
    }

    let secret_bytes = hex::decode(&secret).unwrap();
    println!("secret: {:?}", secret_bytes);
    let secret = hexdump(&secret_bytes);
    let mut secret_bool = Vec::new();
    for (i, c) in secret.chars().enumerate() {
        // do something with character `c` and index `i`
        if c == '1' {
            secret_bool.push(true);
        } else {
            secret_bool.push(false);
        }
    }
    let nonce_bytes = hex::decode(&nonce).unwrap();
    println!("nonce:{:?}", nonce_bytes);
    let nonce = hexdump(&nonce_bytes);
    let mut nonce_bool = Vec::new();
    for (i, c) in nonce.chars().enumerate() {
        // do something with character `c` and index `i`
        if c == '1' {
            nonce_bool.push(true);
        } else {
            nonce_bool.push(false);
        }
    }

    id_bytes.extend(secret_bytes.clone());
    id_bytes.extend(nonce_bytes);

    println!("total strings: {:?}", id_bytes);

    let (xl, xr) = split(id_bytes);

    println!("(xl, xr) = ({:?}, {:?})", xl, xr);


    let mut hash_result1 = [0u8; 32];
    let mut hash_result2 = [0u8; 32];

    let mut a = Sha256::new();
    a.input_str("123456");
    a.result(&mut hash_result1[..]);
    println!("a: {:?}", hash_result1);

    let mut b = Sha256::new();
    b.input(&secret_bytes);
    b.result(&mut hash_result2[..]);
    println!("b: {:?}", hash_result2);

    // let old_blinding_bits: Vec<bool> = (0..NUM_BLINDING_BITS).map(|_| preimage).collect();
}

fn hexdump(bytes: &[u8]) -> String {

    let mut sum = String::new();

    println!("{} bytes:", bytes.len());
    for (i, b) in bytes.iter().enumerate() {
        // b: &u8 の値を2桁の16進数で表示する
        print!("{:?}", b);
        let a = format!("{:b}", b);
        sum += &a;

        // 値を16個表示するごとに改行する
        if (i + 1) % 16 == 0 {
            println!();
        }
    }
    println!();
    println!("{:?}", sum);
    sum
}

fn split(bytes: Vec<u8>) -> (String, String) {
    let mut xl: String = String::default();
    let mut xr: String = String::default(); 

    println!("{} bytes:", bytes.len());
    for (i, b) in bytes.iter().enumerate() {
        // b: &u8 の値を2桁の16進数で表示する
        let a = format!("{:?}", b);
        xl += &a;

        // 値を16個表示するごとに改行する
        if i > bytes.len() / 2 {
            let a = format!("{:?}", b);
            xr += &a;
        }
    }
    (xl, xr)
}