use proconio::input;

use std::cmp::Ordering;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }

    const FIVE_HUNDRED_YEN: usize = 500;
    const ONE_HUNDRED_YEN: usize = 100;
    const FIFTY_YEN: usize = 50;

    let mut count: usize = 0;
    for ai in 0..=(a) {
        if x < FIVE_HUNDRED_YEN * ai {
            break;
        }

        for bi in 0..=(b) {
            if x < FIVE_HUNDRED_YEN * ai + ONE_HUNDRED_YEN * bi {
                break;
            }

            for ci in 0..=(c) {
                match (FIVE_HUNDRED_YEN * ai + ONE_HUNDRED_YEN * bi + FIFTY_YEN * ci).cmp(&x) {
                    Ordering::Equal => {
                        count += 1;
                        break;
                    }
                    Ordering::Greater => break,
                    Ordering::Less => continue,
                }
            }
        }
    }

    println!("{}", count);
}
