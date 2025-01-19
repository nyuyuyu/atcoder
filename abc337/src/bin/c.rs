use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut answer = Vec::new();
    let mut tmp = vec![0; n + 1];
    for (i, v) in a.iter().enumerate() {
        if *v == -1 {
            answer.push(i + 1);
            continue;
        }

        tmp[*v as usize] = i + 1;
    }

    let mut p = answer[0];
    for _ in 0..n - 1 {
        p = tmp[p];
        answer.push(p);
    }

    println!("{}", answer.iter().join(" "));
}
