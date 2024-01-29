use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    const LIMIT: u64 = 10_u64.pow(18);

    let mut candidates = vec![];
    for x in 0..=59 {
        for y in 0..=59 {
            let ans = 2_u64.pow(x) * 3_u64.pow(y);
            if ans > LIMIT {
                break;
            }
            candidates.push(ans);
        }
    }
    for x in 0..=59 {
        for y in 0..=59 {
            let ans = 2_u64.pow(y) * 3_u64.pow(x);
            if ans > LIMIT {
                break;
            }
            candidates.push(ans);
        }
    }

    if candidates.contains(&n) {
        println!("Yes");
    } else {
        println!("No");
    }
}
