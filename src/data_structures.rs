#![allow(unused)]

struct Queue<T : Copy + Sized>
{
    val : Vec<T>, s : usize, e : usize
}

impl<T : Copy + Sized> Queue<T> {
    fn new() -> Queue<T> {
        Queue { val : Vec::new(), s : 0, e : 0 }
    }

    fn push(&mut self, x : T) {
        self.val.push(x);
        self.e += 1;
    }

    fn is_empty(&self) -> bool { self.s == self.e }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.s += 1;
            Some(self.val[self.s - 1])
        }
    }

    fn size(&self) -> usize {
        self.e - self.s
    }
}
