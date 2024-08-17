use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    let mut answer = "No";
    if b < c {
        if a < b || a >= c {
            answer = "Yes";
        }
    } else {
        if a >= c && a < b {
            answer = "Yes";
        }
    }

    println!("{}", answer);
}
