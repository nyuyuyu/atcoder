use proconio::input;

fn main() {
    input! {
        mut n: u64,
    }

    if n == 1 {
        println!("{}", 0);
        return;
    }

    n = n - 1;

    let mut answer = String::from("");
    while n != 0 {
        answer += &((n % 5) * 2).to_string();
        n = n / 5;
    }

    answer = answer.chars().rev().collect::<String>();
    println!("{}", answer);
}
