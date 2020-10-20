use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if(args.len() > 1) {
        let mut i = args[1].len() - 1;
        let char_vec: Vec<char> = args[1].chars().collect();
        while i > 2 {
            convertToBase64(&[char_vec[i], char_vec[i-1], char_vec[i-2]]);
            i -= 3;
        }
        if(i == 2) {
            convertToBase64(&[char_vec[i], char_vec[i-1], char_vec[i-2]]);
        }
        if(i == 1) {
            convertToBase64(&[char_vec[i], char_vec[i-1]]);
        }
        if(i == 0) {
            convertToBase64(&[char_vec[i]]);
        }
        println!("{:?}", args[1]);
    }
}

fn convertToBase64(args: &[char]) -> std::vec::Vec<u8> {
    let mut bytes:Vec<u8> = Vec::new();
    for arg in args {
        let byte = u8::from_str_radix(&arg.to_string()[0..1], 16);
        match byte {
            Ok(value) => bytes.push(value),
            Err(error) => {}
        }
    }
    println!("{:?}", bytes);
    return bytes;
}