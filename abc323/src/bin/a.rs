use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut answer = "Yes";
    for i in (2..=16).step_by(2) {
        if &s.chars().nth(i - 1).unwrap().to_string() == "1" {
            answer = "No";
            break;
        }
    }

    println!("{}", answer);
}
