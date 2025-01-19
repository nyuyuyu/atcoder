use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        td: String,
        s: [String; n],
    }

    let mut answer = vec![];
    for (i, tc) in s.iter().enumerate() {
        if td == *tc {
            answer.push(i + 1);
            continue;
        }

        if td.len() == tc.len() + 1 {
            for (j, (tdc, tcc)) in td.chars().zip(tc.chars()).enumerate() {
                if tdc != tcc {
                    if td[(j + 1)..] == tc[j..] {
                        answer.push(i + 1);
                    }
                    break;
                }
                if j == tc.len() - 1 {
                    answer.push(i + 1);
                }
            }
            continue;
        }

        if td.len() == tc.len() - 1 {
            for (j, (tdc, tcc)) in td.chars().zip(tc.chars()).enumerate() {
                if tdc != tcc {
                    if td[j..] == tc[(j + 1)..] {
                        answer.push(i + 1);
                    }
                    break;
                }
                if j == td.len() - 1 {
                    answer.push(i + 1);
                }
            }
            continue;
        }

        if td.len() == tc.len() {
            for (j, (tdc, tcc)) in td.chars().zip(tc.chars()).enumerate() {
                if tdc != tcc {
                    if td[(j + 1)..] == tc[(j + 1)..] {
                        answer.push(i + 1);
                    }
                    break;
                }
            }
        }
    }

    println!("{}", answer.len());
    println!("{}", answer.iter().join(" "));
}
