use libm::sqrt;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [[i64; 2]; n],
    }

    let mut answer = vec![0; n];
    for i in 0..n {
        let mut far_index = 0;
        let mut far_distance = 0.0;
        for j in 0..n {
            if i == j {
                continue;
            }

            let d = distance(&xy[i], &xy[j]);
            if far_distance < d {
                far_index = j;
                far_distance = d;
            }
        }
        answer[i] = far_index;
    }
    for i in answer {
        println!("{}", i + 1);
    }
}

fn distance(xy1: &Vec<i64>, xy2: &Vec<i64>) -> f64 {
    return sqrt(((xy1[0] - xy2[0]).pow(2) + (xy1[1] - xy2[1]).pow(2)) as f64);
}
