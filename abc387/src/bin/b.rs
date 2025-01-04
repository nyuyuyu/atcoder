use proconio::input;

fn main() {
    input! {
        x: i32,
    }

    let mut sum = 0;
    for i in 1..=9 {
        for j in 1..=9 {
            let answer = i * j;
            if answer != x {
                sum += answer;
            }
        }
    }

    println!("{}", sum);
}
