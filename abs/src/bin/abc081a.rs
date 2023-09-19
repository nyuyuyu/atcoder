use proconio::input;

fn main() {
    input! {
        s: String
    }

    let mut count: usize = 0;
    for i in s.chars().into_iter() {
        if i == '1' {
            count += 1;
        }
    }

    println!("{}", count);
}
