use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        tl: [[usize; 2]; n],
    }

    let mut answer = Vec::new();
    for i in 0..d {
        let mut max = 0;
        for j in 0..n {
            let len = tl[j][0] * (tl[j][1] + i + 1);
            if len > max {
                max = len;
            }
        }
        answer.push(max);
    }

    for i in 0..answer.len() {
        println!("{}", answer[i]);
    }
}
