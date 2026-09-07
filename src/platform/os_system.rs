use std::cell::RefCell;
use std::process::Child;

#[derive(Default)]
pub struct OsSystem {
    children: RefCell<Vec<Child>>,
}

impl OsSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn keep_child(&self, child: Child) {
        self.children.borrow_mut().push(child);
    }

    pub fn reap_children(&self) {
        for mut child in self.children.borrow_mut().drain(..) {
            let _ = child.try_wait();
        }
    }
}
