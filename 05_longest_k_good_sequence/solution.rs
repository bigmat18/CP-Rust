// https://codeforces.com/contest/616/problem/D?locale=en

use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    
    let mut tokens = buffer.split_whitespace();

    let n: usize = match tokens.next() {
        Some(t) => t.parse().unwrap(),
        None => return,
    };
    let k: usize = tokens.next().unwrap().parse().unwrap();

    let mut vec: Vec<usize> = Vec::with_capacity(n);
    for _ in 0..n {
        vec.push(tokens.next().unwrap().parse().unwrap());
    }
    
    let mut map = HashMap::new();
    
    let mut left = 0;
    let mut lbest = 0;
    let mut rbest = 0;

    for (right, &num) in vec.iter().enumerate() {
        *map.entry(num).or_insert(0) += 1;
        
        while map.len() > k {
            let left_val = vec[left];
            if let Some(count) = map.get_mut(&left_val) {
                *count -= 1;
                if *count == 0 {
                    map.remove(&left_val);
                }
            }
            left += 1;
        }
        
        if (right - left) > (rbest - lbest) {
            rbest = right;
            lbest = left;
        }
    }

    println!("{} {}", lbest+1, rbest+1);
}
