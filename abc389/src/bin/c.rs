use std::{
    io::{self, BufRead},
    usize,
};

fn main() {
    let mut query: Vec<Vec<usize>> = vec![];

    let stdin = io::stdin();
    for (i, line) in stdin.lock().lines().enumerate() {
        if i == 0 {
            continue;
        }
        let s: Vec<usize> = line
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        query.push(s);
    }

    let mut queue: Vec<usize> = vec![];
    let mut len: usize = 0;
    let mut remove_count: usize = 0;
    for i in 0..query.len() {
        match query[i][0] {
            1 => {
                // 長さ l のヘビが行列の末尾に追加される
                queue.push(len);
                let l = query[i][1];
                len += l;
            }
            2 => {
                // 先頭のヘビが抜ける
                remove_count += 1;
            }
            3 => {
                // 行列の先頭から k 番目のヘビの頭の座標を出力する
                let k = query[i][1];
                println!("{}", queue[k - 1 + remove_count] - queue[remove_count]);
            }
            _ => unreachable!(),
        }
    }
}
