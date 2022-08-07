#![allow(unused)]

pub(crate) trait DS {
    fn new() -> Self;
}

pub(crate) struct Queue<T: Copy + Sized>
{
    val: Vec<T>,
    s: usize,
    e: usize,
}

impl<T: Copy + Sized> DS for Queue<T> {
    fn new() -> Queue<T> {
        Queue { val: Vec::new(), s: 0, e: 0 }
    }
}

impl<T: Copy + Sized> Queue<T> {
    pub fn push(&mut self, x: T) {
        self.val.push(x);
        self.e += 1;
    }

    pub fn is_empty(&self) -> bool { self.s == self.e }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.s += 1;
            Some(self.val[self.s - 1])
        }
    }

    pub fn front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.val[self.s])
        }
    }

    pub fn size(&self) -> usize {
        self.e - self.s
    }
}

pub(crate) struct Stack<T: Copy + Sized>
{
    val: Vec<T>,
    siz: usize,
}

impl<T: Copy + Sized> DS for Stack<T> {
    fn new() -> Stack<T> {
        Stack { val: Vec::new(), siz: 0 }
    }
}

impl<T: Copy + Sized> Stack<T> {
    pub fn push(&mut self, x: T) {
        self.val.push(x);
        self.siz += 1;
    }

    pub fn is_empty(&self) -> bool { self.siz == 0 }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.siz -= 1;
            Some(self.val[self.siz])
        }
    }

    pub fn top(&self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.val[self.siz - 1])
        }
    }
}
