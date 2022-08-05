#![allow(non_snake_case)]

use std::collections::VecDeque;

fn inputInteger() -> usize {
    let mut tmp = String::new();
    std::io::stdin().read_line(&mut tmp).unwrap();
    tmp.trim().parse::<usize>().expect("not number")
}

fn bfs(map : Vec<Vec<char>>) -> i32 {
    const DX: [i32; 4] = [1, 0, -1, 0];
    const DY: [i32; 4] = [0, 1, 0, -1];

    let n = map.len();
    let mut group = [[0; 100]; 100];
    let mut visit = [[false; 100]; 100];
    let mut cnt = 0;

    for i in 0..n {
        for j in 0..n {
            if group[i][j] > 0 { continue; }

            let mut queue : VecDeque<(usize, usize)> = VecDeque::new();

            queue.push_back((i, j));
            cnt += 1;

            while !queue.is_empty() {
                let cur = queue.pop_front().expect("Empty");

                for d in 0..4usize {
                    let nx = cur.0 as i32 + DX[d];
                    let ny = cur.1 as i32 + DY[d];

                    if nx < 0 || nx >= n as i32 || ny < 0 || ny >= n as i32 || map[cur.0][cur.1] != map[nx as usize][ny as usize] || visit[nx as usize][ny as usize] {
                        continue;
                    }

                    group[nx as usize][ny as usize] = cnt;
                    visit[nx as usize][ny as usize] = true;
                    queue.push_back((nx as usize, ny as usize));
                }
            }
        }
    }

    cnt
}

fn main() {
    let n = inputInteger();
    let mut map : Vec<Vec<char>> = Vec::new();

    for _i in 0..n {
        let mut tmp = String::new();
        std::io::stdin().read_line(&mut tmp).unwrap();
        map.push(tmp.chars().collect());
    }

    println!("{}", bfs(map.clone()));

    for i in 0..n {
        for j in 0..n {
            if map[i][j] == 'R' {
                map[i][j] = 'G';
            }
        }
    }

    println!("{}", bfs(map));
}