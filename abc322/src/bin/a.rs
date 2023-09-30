use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String
    }

    const KEYWORD: &str = "ABC";

    let mut answer = -1;
    if let Some(i) = s.find(KEYWORD) {
        answer = i as i32 + 1;
    }

    println!("{}", answer);
}
