fn main() {
    let upper_limit: i32 = 1000;
    let mut sum: i32 = 0;
    for x in 1..upper_limit  {
        if x % 3 == 0 && x % 5 == 0 {
            sum += x;
        } else if x % 3 == 0 {
            sum += x;
        } else if x % 5 == 0 {
            sum += x;
        }
    }
    println!("{}", sum);
}
