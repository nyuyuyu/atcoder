use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let l = s.chars().nth(0).unwrap().to_digit(10).unwrap();
    let r = s.chars().nth(2).unwrap().to_digit(10).unwrap();

    println!("{}", l * r);
}
