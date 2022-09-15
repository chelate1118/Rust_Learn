#![allow(non_snake_case)]

mod real_number;
mod interface;

use real_number::Real;
use real_number::Real::{Rat, Irr};
use crate::interface::Runnable;

fn main() {
    let mut x: Real = Rat(real_number::Rational{num:10, den:7});
    let y: Real = Rat(real_number::Rational{num:20, den:9});
    let z: Real = Irr(10.23487);

    let mut a = 10;
    {
        dbg!(a);
        let b = &mut a;
        *b += 10;
        dbg!(b);
    }
    dbg!(a);
    let c;
    {
        let d = 20;
        c = &d;
        dbg!(c);
        dbg!(d);
    }

    let mut y = y;
    let mut w = &mut y;
    *w = x + z;
    dbg!(w);
    w = &mut x;
    dbg!(w);

    println!("{} {}", x + y, x - y);
    println!("{} {}", x + z, y + z);

    // Java 스타일. 함수형 지원이 잘 돼서 딱히 필요 없는 듯...
    Runnable::new(|| println!("run!!")).run();
    let function = || { println!("run!!!"); };
    Runnable::new(function).run();
}