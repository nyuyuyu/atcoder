use proconio::input;

fn main() {
    input! {
        n: i32
    }

    let mut answer = 0;
    for c in format!("{:b}", n).chars().rev() {
        if c == '0' {
            answer += 1;
        } else {
            break;
        }
    }

    println!("{}", answer);
}
