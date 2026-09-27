// https://codeforces.com/problemset/problem/596/B?locale=en

use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut values = input.split_whitespace();
    let n : usize = values.next().unwrap().parse().unwrap();
    let mut b : Vec<i64> = vec![0; n+1];

    for i in 1..=n {
        b[i] = values.next().unwrap().parse().unwrap();
    }

    let mut result : i64 = 0;

    for i in 1..=n {
        result += (b[i] - b[i-1]).abs();
    }

    print!("{}", result);

}
