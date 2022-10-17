use std::str;

fn abbrev_name(name: &str) -> String {
    let bytes = name.as_bytes();
    let mut returnValue = String::new();

    returnValue.push(bytes[0] as char);
    returnValue.push('.');

    for(i, &char) in bytes.iter().enumerate() {
        if char == b' ' {
            returnValue.push(bytes[i + 1] as char);
            return returnValue.to_uppercase();
        }
    }
    return returnValue.to_uppercase();
}

fn main() {
    let name = String::from("Glen Pierce");
    println!("hello {:?}", name);
    println!("{}", abbrev_name(&name));
}