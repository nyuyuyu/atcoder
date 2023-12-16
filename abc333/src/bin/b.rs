use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }
    let s1 = &s.chars().nth(0).unwrap();
    let s2 = &s.chars().nth(1).unwrap();
    let t1 = &t.chars().nth(0).unwrap();
    let t2 = &t.chars().nth(1).unwrap();

    let mut s_diff = ((*s1 as i32) - (*s2 as i32)).abs();
    if s_diff == 4 {
        s_diff = 1;
    }
    if s_diff == 3 {
        s_diff = 2;
    }
    let mut t_diff = ((*t1 as i32) - (*t2 as i32)).abs();
    if t_diff == 4 {
        t_diff = 1;
    }
    if t_diff == 3 {
        t_diff = 2;
    }

    let mut answer = "No";
    if s_diff == t_diff {
        answer = "Yes";
    }
    println!("{}", answer);
}
