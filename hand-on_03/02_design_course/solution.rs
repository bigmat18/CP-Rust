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
        "{}/hand-on_03/02_design_course/input/input{}.txt",
        manifest_dir, test_num
    );
    let test_output = format!(
        "{}/hand-on_03/02_design_course/output/output{}.txt",
        manifest_dir, test_num
    );

    let content_in = fs::read_to_string(test_input)?;
    let content_out = fs::read_to_string(test_output)?;

    let mut values_in = content_in.split_whitespace();
    let mut values_out = content_out.split_whitespace();

    let n: usize = values_in.next().unwrap().parse().unwrap();

    let mut courses: Vec<(i32, i32)> = Vec::with_capacity(n);
    for _ in 0..n {
        let b = values_in.next().unwrap().parse().unwrap();
        let d = values_in.next().unwrap().parse().unwrap();
        courses.push((b, d));
    }

    courses.sort_unstable_by(|a, b| {
        a.0.cmp(&b.0).then(b.1.cmp(&a.1))
    });
    let mut tails: Vec<i32> = Vec::new();

    for (_, d) in courses {
        let idx = tails.partition_point(|&val| val < d);
        if idx == tails.len() {
            tails.push(d);
        } else {
            tails[idx] = d;
        }
    }
    let expected: i32 = values_out.next().unwrap().parse().unwrap();
    assert_eq!(tails.len() as i32, expected, "Error in test {}", test_num);

    Ok(())
}
