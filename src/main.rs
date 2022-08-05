#![allow(non_snake_case)]

fn inputInteger() -> i32 {
    let mut tmp = String::new();
    std::io::stdin().read_line(&mut tmp).unwrap();
    tmp.trim().parse::<i32>().expect("not number")
}

fn inputIntegers() -> Vec<i32> {
    let mut tmp = String::new();
    std::io::stdin().read_line(&mut tmp).unwrap();
    let split = tmp.split_whitespace();
    let mut ret: Vec<i32> = Vec::new();
    for i in split { ret.push(i.parse::<i32>().expect("not number")); }
    ret
}

fn main() {
    let arr = inputIntegers();
    let K = arr[0];
    let N = arr[1];
    let mut arr: Vec<i32> = Vec::new();

    for _ in 0..K { arr.push(inputInteger()); }

    arr.sort_by(|a, b| {
        let mut x = a.to_string();
        let mut y = b.to_string();
        let z = y.clone();

        y.push_str(&x);
        x.push_str(&z);
        y.cmp(&x)
    });

    let mut cop = arr.clone();
    cop.sort_by(|a, b| {
        let mut x = a.to_string();
        let mut y = b.to_string();
        let z = y.clone();

        if x.len() != y.len() {
            return y.len().cmp(&x.len());
        }

        y.push_str(&x);
        x.push_str(&z);
        y.cmp(&x)
    });

    for i in arr {
        print!("{}", i);
        if i == cop[0] {
            for _ in 0..N - K {
                print!("{}", i);
                cop[0] = -1;
            }
        }
    }
}