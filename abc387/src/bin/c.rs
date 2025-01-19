use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    }

    println!("{}", f(r) - f(l - 1));
}

fn f(x: usize) -> usize {
    if x < 10 {
        return 0;
    }

    let mut count = 0;
    let digits: Vec<usize> = x
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    // 桁数が x より小さい
    // e.g. x = 2100 なら 0 - 999 まで走査する
    for i in 2..digits.len() {
        for j in 1..=9 {
            count += (j as usize).pow((i as u32) - 1);
        }
    }

    // 桁数が x と同じ、かつ x の先頭の数字未満
    // e.g. x = 2100 なら 1000 - 1999 まで走査する
    for i in 1..digits[0] {
        count += i.pow((digits.len() as u32) - 1);
    }

    // 桁数が x と同じ、かつ x の先頭の数字と同じ
    // e.g. x = 2100 なら 2000 - 2100 まで走査する
    let mut flag = false;
    for i in 1..digits.len() {
        flag = true;
        for j in 0..digits[0] {
            if j == digits[i] {
                flag = false;
                break;
            }
            count += digits[0].pow((digits.len() as u32) - (i as u32) - 1);
        }
        if flag {
            break;
        }
    }
    if !flag {
        count += 1;
    }

    count
}
