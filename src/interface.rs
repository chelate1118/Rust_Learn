
#[derive(Clone)]
pub(super) struct Runnable {
    method: fn() ->()
}

impl Runnable {
    pub(super) fn new(method: fn()->()) -> Self {
        Runnable{ method }
    }

    pub(super) fn run(&self) {
        (self.method)();
    }
}
