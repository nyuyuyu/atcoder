use proconio::input;

fn main() {
    input! {
        a: usize,
        bc: [usize; 2],
        s: String
    }

    println!("{} {}", a + bc[0] + bc[1], s);
}
