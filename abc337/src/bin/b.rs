use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ss = s.clone().chars().sorted().collect::<String>();
    if s == ss {
        println!("Yes");
    } else {
        println!("No");
    }
}
