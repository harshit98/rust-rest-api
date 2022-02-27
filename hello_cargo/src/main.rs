// defining constants
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


fn main() {
    println!("Hello, world!");
    println!("3 Hrs (in secs) = {}", THREE_HOURS_IN_SECONDS);

    // defining tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("Tuple values: ({},{},{})", x, y, z);
}
