use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [usize; n],
    }

    let steps: HashSet<&usize> = d.iter().collect();

    println!("{}", steps.len());
}
