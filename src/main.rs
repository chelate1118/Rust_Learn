#![allow(non_snake_case)]

mod sound_player;
mod data_structures;

use crate::data_structures::{DS, Stack, Queue};

fn main()
{
    sound_player::Play(String::from("src/resource/046.wav"), 2000);

    let mut q : Queue<(i32, i32)> = DS::new();
    q.push((1, 2));
    q.push((3, 4));
    println!("{}", q.pop().unwrap().1);
    println!("{}", q.pop().unwrap().1);

    let mut s : Stack<(i32, i32)> = DS::new();
    s.push((1, 2));
    s.push((3, 4));
    println!("{}", s.pop().unwrap().1);
    println!("{}", s.pop().unwrap().1);
}
