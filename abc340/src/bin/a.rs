use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        d: usize,
    }

    let mut answer = vec![];
    for i in (a..=b).step_by(d) {
        answer.push(i);
    }

    println!("{}", answer.iter().join(" "));
}
