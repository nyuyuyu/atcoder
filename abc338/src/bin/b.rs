use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut scores = HashMap::new();
    for i in 'a'..='z' {
        scores.insert(i, 0);
    }

    for i in s.chars() {
        let score = *scores.get(&i).unwrap();
        scores.insert(i, score + 1);
    }

    let mut max = 0;
    let mut answer = 'a';
    for i in 'a'..='z' {
        let score = *scores.get(&i).unwrap();
        if score > max {
            max = score;
            answer = i;
        }
    }

    println!("{}", answer);
}
