use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut answer = String::from("");
    for i in 1..=n {
        if i % 3 == 0 {
            answer.push_str("x");
        } else {
            answer.push_str("o");
        }
    }

    println!("{}", answer);
}
