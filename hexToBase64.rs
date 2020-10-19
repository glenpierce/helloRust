use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if(args.len() > 1) {
        let mut i = args[1].len() - 1;
        let char_vec: Vec<char> = args[1].chars().collect();
        while i > 0 {
            println!("{:?}", char_vec[i]);
            i -= 1;
        }
        println!("{:?}", args[1]);
    }
    // println!("{:?}", args);
}