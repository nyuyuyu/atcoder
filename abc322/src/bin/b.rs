use proconio::input;

fn main() {
    input! {
        nm: (usize, usize),
        s: String,
        t: String,
    }

    let n = nm.0;
    let m = nm.1;

    let mut answer = 3;
    if &t[..n] == s && &t[(m - n)..] == s {
        answer = 0;
    } else if &t[..n] == s {
        answer = 1;
    } else if &t[(m - n)..] == s {
        answer = 2;
    }

    println!("{}", answer);
}
