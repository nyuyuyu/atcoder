use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
        s: [String; n],
    }

    let mut sum_now = vec![0; n];
    for (pi, ps) in s.iter().enumerate() {
        sum_now[pi] += pi + 1;
        for (i, c) in ps.chars().enumerate() {
            if c == 'o' {
                sum_now[pi] += a[i];
            }
        }
    }

    let mut answer = vec![0; n];
    let max_score = sum_now.iter().max().unwrap();
    for (pi, score_now) in sum_now.iter().enumerate() {
        if score_now == max_score {
            continue;
        }

        let mut sum = *score_now;
        let rs: Vec<usize> = s[pi]
            .chars()
            .enumerate()
            .map(|(i, c)| if c == 'x' { a[i] } else { 0 })
            .sorted()
            .rev()
            .collect();
        for score in rs {
            answer[pi] += 1;
            sum += score;
            if max_score < &sum {
                break;
            }
        }
    }

    for i in answer {
        println!("{}", i);
    }
}
