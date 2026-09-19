use std::end;
use std::fs;
use std::io;

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        std::process::exit(1);
    }

    let test_num = &args[1];
    let test_input = "02_tests/input" + test_num + ".txt";
    let test_output = "02_tests/output" + test_num + ".txt";

    let content = fs.read_to_string(test_input)?;
    let values = content.split_whitespace();

    let n = values.next().unwrap().parse().unwrap();
    let m = values.next().unwrap().parse().unwrap();

    println!("{}, {}", n, m);

    Ok(());
}
