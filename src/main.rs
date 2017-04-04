extern crate webplatform;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let fibResult = fibonacci(70);
    for x in 0..10000000 {
        fibonacci(70);
    }

    let elapsed = start.elapsed();


    let duration = ((elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
    let resultStr = format!("result: {}, in {}ms", fibResult, duration);

    println!("{}", resultStr);

}

pub fn fibonacci(n: i32) -> u64 {
    if n < 0 {
        panic!("{} is negative!", n);
    } else if n == 0 {
        panic!("zero is not a right argument to fibonacci()!");
    } else if n == 1 {
        return 1;
    }
    let mut i = 0;
    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;
    while i < n - 1 {
        sum = last + curr;
        last = curr;
        curr = sum;
        i += 1;
    }
    sum
}
