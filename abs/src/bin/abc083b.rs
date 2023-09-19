use proconio::input;

fn main() {
    input! {
        nab: [u32; 3],
    }
    let n = nab[0];
    let a = nab[1];
    let b = nab[2];

    let mut sum = 0;
    for i in 1..n + 1 {
        let digit_sum = i
            .to_string()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .sum::<u32>();
        if digit_sum >= a && digit_sum <= b {
            sum += i;
        }
    }

    println!("{}", sum);
}
