use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [[String; 1]; n],
    }

    let mut wins = Vec::new();
    for i in s {
        let mut sum = 0;
        for j in i[0].chars() {
            if j == 'o' {
                sum += 1;
            }
        }

        wins.push(sum);
    }

    let mut answer = Vec::new();
    for (i, _) in wins.iter().enumerate().sorted_by(|(_, a), (_, b)| b.cmp(a)) {
        answer.push(i + 1);
    }

    println!("{}", answer.iter().join(" "));
}
