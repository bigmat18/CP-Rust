use std::collections::btree_map::Values;
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
    let mut values = content_in.split_whitespace();

    let n: usize = values.next().unwrap().parse().unwrap();
    let m: usize = values.next().unwrap().parse().unwrap();

    let mut segments = vec![0; n + 1];

    for _ in 0..n {
        let l: usize = values.next().unwrap().parse().unwrap();
        let r: usize = values.next().unwrap().parse().unwrap();
        segments[l] += 1;
        segments[r + 1] -= 1;
    }

    for _ in 0..m {
        let l: i32 = values.next().unwrap().parse().unwrap();
        let r: i32 = values.next().unwrap().parse().unwrap();
        let k: i32 = values.next().unwrap().parse().unwrap();
    }

    println!("{}, {}", n, m);

    Ok(())
}
