// https://codeforces.com/contest/545/problem/C?locale=en

use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut values = input.split_whitespace();
    let n : usize = values.next().unwrap().parse().unwrap();

    let mut a = Vec::new();

    for i in 1..=n {
         let x : i64 = values.next().unwrap().parse().unwrap();
         let h : i64 = values.next().unwrap().parse().unwrap();
         a.push((x, h));
    }

    if n == 1 {
        print!("1");
        return;
    }

    let mut result : i64 = 2;
    let mut prev : i32 = -1;

    for i in 1..n-1 {
        if prev == -1 && a[i].0 - a[i].1 > a[i-1].0 {
            result += 1;
            prev = -1;
        } else if prev == 0 && a[i].0 - a[i].1 > a[i-1].0 {
            result += 1;
            prev = -1;
        } else if prev == 1 && a[i].0 - a[i].1 > a[i-1].0 + a[i-1].1 {
            result += 1;
            prev = -1;
        } else if a[i].0 + a[i].1 < a[i+1].0 {
            result += 1;
            prev = 1;
        } else {
            prev = 0;
        }

    }

    print!("{}", result);

}
