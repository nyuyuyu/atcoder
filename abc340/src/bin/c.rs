use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    let mut memo: HashMap<u64, u64> = HashMap::new();
    println!("{}", f(n, &mut memo));
}

fn f(n: u64, m: &mut HashMap<u64, u64>) -> u64 {
    if n < 2 {
        return 0;
    }
    match m.get(&n) {
        None => {
            let sum = f(n / 2, m) + f((n + 1) / 2, m) + n;
            m.entry(n).or_insert(sum);
            return sum;
        }
        _ => {
            return m[&n];
        }
    }
}
