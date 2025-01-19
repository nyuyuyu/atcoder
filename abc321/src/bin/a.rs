use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut answer = "Yes";
    let mut number = 0;
    for (i, c) in n.to_string().chars().rev().enumerate() {
        if i != 0 && c.to_digit(10).unwrap() <= number {
            answer = "No";
            break;
        }

        number = c.to_digit(10).unwrap();
    }

    println!("{}", answer);
}
