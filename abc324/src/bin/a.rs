use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut answer = "Yes";
    if a.iter().unique().count() != 1 {
        answer = "No";
    }

    println!("{}", answer);
}
