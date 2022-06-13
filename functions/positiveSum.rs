fn positiveSum(arr: &[i32]) -> i32 {
    let mut returnValue = 0;
    for &value in arr.iter() {
        if value > 0 {
            returnValue += value;
        }
    }
    return returnValue;
}

fn main() {
    let values= &[1,2,3,-4];
    println!("{}", positiveSum(values));
}