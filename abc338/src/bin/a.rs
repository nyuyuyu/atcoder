use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let a = s.chars().nth(0).unwrap().to_uppercase();
    let b = &s[1..s.len()].to_lowercase();

    let expected = format!("{}{}", a, b);
    if s == expected {
        println!("Yes");
    } else {
        println!("No");
    }
}
