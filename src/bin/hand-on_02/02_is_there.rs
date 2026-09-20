use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        std::process::exit(1);
    }

    let test_num = &args[1];
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let test_input = format!(
        "{}/src/bin/hand-on_02/02_tests/input{}.txt",
        manifest_dir, test_num
    );
    let test_output = format!(
        "{}/src/bin/hand-on_02/02_tests/output{}.txt",
        manifest_dir, test_num
    );

    let content_in = fs::read_to_string(test_input)?;
    let content_out = fs::read_to_string(test_output)?;

    let mut values_in = content_in.split_whitespace();
    let mut values_out = content_out.split_whitespace();

    let n: usize = values_in.next().unwrap().parse().unwrap();
    let m: usize = values_in.next().unwrap().parse().unwrap();

    let mut segments: Vec<i32> = vec![0; n + 1];
    for _ in 0..n {
        let l: usize = values_in.next().unwrap().parse().unwrap();
        let r: usize = values_in.next().unwrap().parse().unwrap();
        segments[l] += 1;
        segments[r + 1] -= 1;
    }

    let mut current_sum = 0;
    let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
    for (idx, &val) in segments.iter().enumerate().take(n) {
        current_sum += val;
        map.entry(current_sum).or_default().push(idx);
    }

    for i in 0..m {
        let mut result = 0;
        let l: usize = values_in.next().unwrap().parse().unwrap();
        let r: usize = values_in.next().unwrap().parse().unwrap();
        let k: i32 = values_in.next().unwrap().parse().unwrap();

        if let Some(arr) = map.get(&k) {
            let idx = arr.partition_point(|&pos| pos < l);
            if idx < arr.len() && arr[idx] <= r {
                result = 1;
            }
        }

        let expected: i32 = values_out.next().unwrap().parse().unwrap();
        assert_eq!(result, expected, "Error in index {}", i);
    }

    Ok(())
}
