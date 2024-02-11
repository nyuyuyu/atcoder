use proconio::input;

fn main() {
    input! {
        q: usize,
        query: [[usize; 2]; q],
    }

    let mut a = vec![];
    for i in query {
        if i[0] == 1 {
            a.push(i[1]);
            continue;
        }

        println!("{}", a.iter().nth_back(i[1] - 1).unwrap());
    }
}
