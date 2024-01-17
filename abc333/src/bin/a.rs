use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut answer = String::from("");
    for _ in 1..=n {
        answer += &n.to_string();
    }

    println!("{}", answer);
}
