use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let mut i: usize = args[1].len() - 1;
        let char_vec: Vec<char> = args[1].chars().collect();
        let mut byteVec: Vec<usize> = Vec::new();
        while i > 2 {
            let mut convertedValue: Vec<usize> = convertToBase10(&[char_vec[i], char_vec[i-1], char_vec[i-2]]);
            byteVec.append(&mut convertedValue);
            i -= 3;
        }
        if i == 2 {
            let mut convertedValue: Vec<usize> = convertToBase10(&[char_vec[i], char_vec[i-1], char_vec[i-2]]);
            byteVec.append(&mut convertedValue);
        }
        if i == 1 {
            let mut convertedValue: Vec<usize> = convertToBase10(&[char_vec[i], char_vec[i-1]]);
            byteVec.append(&mut convertedValue);
        }
        if i == 0 {
            let mut convertedValue: Vec<usize> = convertToBase10(&[char_vec[i]]);
            byteVec.append(&mut convertedValue);
        }

        let mut base10Value: usize = 0;
        let mut place: u32 = 0;
        for position in byteVec {
            let multiplier: usize = usize::pow(16, place);
            base10Value += position * multiplier;
            place += 1;
        }

        println!("{}", base10Value);
    }
}

fn convertToBase10(args: &[char]) -> std::vec::Vec<usize> {
    let mut bytes:Vec<usize> = Vec::new();
    for arg in args {
        let byte = usize::from_str_radix(&arg.to_string()[0..1], 16);
        match byte {
            Ok(value) => bytes.push(value),
            Err(error) => {
                println!("{:?}", error);
            }
        }
    }
    return bytes;
}