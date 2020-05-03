fn main() {
    for i in 0..5 {
        let fizz = if i % 3 == 0 {"fizz"} else {"buzz"};
        println!("{}{}", fizz, i);
    }
}
