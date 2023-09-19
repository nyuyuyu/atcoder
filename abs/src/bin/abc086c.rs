use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [[i32; 3]; n],
    }

    let mut answer = "Yes";

    let mut time = 0;
    let mut x = 0;
    let mut y = 0;
    for i in t {
        let distance = (x - i[1]).abs() + (y - i[2]).abs();
        let time_width = i[0] - time;
        if (distance % 2) != (time_width % 2) || distance > time_width {
            answer = "No";
            break;
        }

        time = i[0];
        x = i[1];
        y = i[2];
    }

    println!("{}", answer);
}
