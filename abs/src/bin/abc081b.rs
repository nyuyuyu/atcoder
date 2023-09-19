use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut count: usize = 0;
    loop {
        if a.iter().any(|&i| i % 2 != 0) {
            break;
        }

        for i in a.iter_mut() {
            *i = *i / 2;
        }
        count += 1;
    }

    println!("{}", count);
}
