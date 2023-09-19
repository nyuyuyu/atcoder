use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut alice = 0;
    let mut bob = 0;

    a.sort();
    for (i, v) in a.iter().rev().enumerate() {
        if i % 2 == 0 {
            alice += v;
        } else {
            bob += v;
        }
    }

    println!("{}", alice - bob);
}
