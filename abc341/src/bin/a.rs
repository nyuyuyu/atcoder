use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut answer: Vec<char> = vec!['1'];
    for _ in 0..n {
        answer.push('0');
        answer.push('1');
    }

    println!("{}", answer.iter().join(""));
}
