// https://codeforces.com/contest/86/problem/D

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut values = input.split_whitespace();
    let n: usize = values.next().unwrap().parse().unwrap();
    let t: usize = values.next().unwrap().parse().unwrap();

    let mut arr : Vec<i64> = Vec::with_capacity(n);
    let mut max_val : i64 = 0;
    for _ in 0..n {
        let val : i64 = values.next().unwrap().parse().unwrap();
        arr.push(val);
        max_val = max_val.max(val);
    }

    let mut queries : Vec<(usize, usize, usize)> = Vec::with_capacity(t);
    for i in 0..t {
        let l: usize = values.next().unwrap().parse().unwrap();
        let r: usize = values.next().unwrap().parse().unwrap();
        queries.push((l-1, r-1, i));
    }

    let sqrt_n = (n as f64).sqrt() as usize + 1;
    queries.sort_by_key(|&(l, r, _)| (l / sqrt_n, r));

    let mut results: Vec<i64> = vec![0; t];
    let mut freq = vec![0i64; (max_val + 1) as usize];

    let mut left = 0;
    let mut right = 0;
    let mut current_ans: i64 = 0;

    let add = |val: i64, ans: &mut i64, freq: &mut [i64]| {
        let count = freq[val as usize];
        *ans += val * (2 * count + 1);
        freq[val as usize] += 1;
    };

    let remove = |val: i64, ans: &mut i64, freq: &mut [i64]| {
        let count = freq[val as usize];
        *ans -= val * (2 * count - 1);
        freq[val as usize] -= 1;
    };

    for (l, r, original_idx) in queries {
        while left > l {
            left -= 1;
            add(arr[left], &mut current_ans, &mut freq);
        }
        while right <= r {
            add(arr[right], &mut current_ans, &mut freq);
            right += 1;
        }
        while left < l {
            remove(arr[left], &mut current_ans, &mut freq);
            left += 1;
        }
        while right > r + 1 {
            right -= 1;
            remove(arr[right], &mut current_ans, &mut freq);
        }

        results[original_idx] = current_ans;
    }

    for result in results {
        println!("{}", result);
    }
}
