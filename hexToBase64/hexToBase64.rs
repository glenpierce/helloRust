use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);
    if args.len() > 1 {
        let mut i: usize = args[1].len() - 1;
        let char_vec: Vec<char> = args[1].chars().collect();
        let mut byteVec: Vec<u8> = Vec::new();
        while i > 2 {
            let mut convertedValue: Vec<u8> = convertToBase64(&[char_vec[i], char_vec[i-1], char_vec[i-2]]);
            byteVec.append(&mut convertedValue);
            i -= 3;
        }
        if i == 2 {
            let mut convertedValue: Vec<u8> = convertToBase64(&[char_vec[i], char_vec[i-1], char_vec[i-2]]);
            byteVec.append(&mut convertedValue);
        }
        if i == 1 {
            let mut convertedValue: Vec<u8> = convertToBase64(&[char_vec[i], char_vec[i-1]]);
            byteVec.append(&mut convertedValue);
        }
        if i == 0 {
            let mut convertedValue: Vec<u8> = convertToBase64(&[char_vec[i]]);
            byteVec.append(&mut convertedValue);
        }
        println!("args[1]: {:?}", args[1]);
        println!("byteVec: {:?}", byteVec);
    }
}

fn convertToBase64(args: &[char]) -> std::vec::Vec<u8> {
    let mut bytes:Vec<u8> = Vec::new();
    for arg in args {
        let byte = u8::from_str_radix(&arg.to_string()[0..1], 16);
        match byte {
            Ok(value) => bytes.push(value),
            Err(error) => {
                println!("{:?}", error);
            }
        }
    }
    println!("bytes: {:?}", bytes);
    return bytes;
}