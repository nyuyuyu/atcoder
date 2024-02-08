use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let mut answers = vec![];
    for i in 1..2_u32.pow(10) {
        let mut num = vec![];
        for (i, c) in format!("{:b}", i).chars().rev().enumerate() {
            if c == '1' {
                num.push(i);
            }
        }

        answers.push(num.iter().sorted().rev().join("").parse::<u64>().unwrap());
    }

    answers.sort();
    println!("{}", answers[k]);
}
