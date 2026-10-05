use std::cmp;
use std::env;
use std::fs;
use std::i32;
use std::io;
use std::vec;

#[warn(dead_code)]
fn brute_force(cities: &[Vec<i32>], indices: &mut [usize], days: usize, sum: i32) -> i32 {
    if days == 0 {
        return sum;
    }

    let mut max_val = i32::MIN;

    for i in 0..cities.len() {
        let current_day = indices[i];
        if current_day < cities[i].len() {
            indices[i] += 1;
            let res = brute_force(cities, indices, days - 1, sum + cities[i][current_day]);
            indices[i] -= 1;
            max_val = cmp::max(max_val, res);
        }
    }

    max_val
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        std::process::exit(1);
    }

    let test_num = &args[1];
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let test_input = format!(
        "{}/hand-on_03/01_holiday_planning/input/input{}.txt",
        manifest_dir, test_num
    );
    let test_output = format!(
        "{}/hand-on_03/01_holiday_planning/output/output{}.txt",
        manifest_dir, test_num
    );

    let content_in = fs::read_to_string(test_input)?;
    let content_out = fs::read_to_string(test_output)?;

    let mut values_in = content_in.split_whitespace();
    let mut values_out = content_out.split_whitespace();

    let n: usize = values_in.next().unwrap().parse().unwrap();
    let d: usize = values_in.next().unwrap().parse().unwrap();

    let mut dp = vec![0i32; d + 1];
    let mut psum = vec![0i32; d + 1];

    for _ in 0..n {
        let mut sum = 0;
        for i in 1..=d {
            let num: i32 = values_in.next().unwrap().parse().unwrap();
            sum += num;
            psum[i] = sum;
        }

        for j in (1..=d).rev() {
            let mut best = dp[j];
            for k in 1..=j {
                best = cmp::max(best, dp[j - k] + psum[k]);
            }
            dp[j] = best;
        }
    }

    let expected: i32 = values_out.next().unwrap().parse().unwrap();
    assert_eq!(dp[d], expected, "Error in test {}", test_num);


    Ok(())
}
