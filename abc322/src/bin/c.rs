use proconio::input;

fn main() {
    input! {
        nm: (usize, usize),
        a: [usize; nm.1],
    }

    let n = nm.0;

    let mut c = 0;
    for i in 1..=n {
        if i > a[c] {
            c += 1;
        }

        println!("{}", a[c] - i);
    }
}
