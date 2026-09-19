// https://www.geeksforgeeks.org/problems/n-meetings-in-one-room/1

use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut values = input.split_whitespace();
    let n: usize = values.next().unwrap().parse().unwrap();
    let mut meetings = Vec::<(i64, i64)>::new();

    for _ in 0..n {
        let t: i64 = values.next().unwrap().parse().unwrap();
        let d: i64 = values.next().unwrap().parse().unwrap();
        meetings.push((d, t));
    }
    meetings.sort_unstable();

    let mut count = 0;
    let mut last_end = -1;

    for meeting in meetings {
        let end = meeting.0;
        let start = meeting.1;

        // Se il tempo di inizio è successivo alla fine dell'ultima riunione
        if start > last_end {
            count += 1;
            last_end = end;
        }
    }

    print!("{}", count);
}
