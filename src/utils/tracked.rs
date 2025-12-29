use std::cell::{Cell, Ref, RefCell};

#[derive(Default)]
pub struct Tracked<T> {
    value: RefCell<T>,
    dirty: Cell<bool>,
}

impl<T> Tracked<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: RefCell::new(value),
            dirty: Cell::new(true),
        } // start dirty so initial state triggers
    }

    pub fn set(&self, value: T) {
        self.value.replace(value);
        self.dirty.set(true);
    }

    /// Only marks dirty if value actually changed
    pub fn set_if_changed(&self, value: T)
    where
        T: PartialEq,
    {
        if *self.value.borrow() != value {
            self.value.replace(value);
            self.dirty.set(true);
        }
    }

    pub fn get(&'_ self) -> Ref<'_, T> {
        self.value.borrow()
    }

    pub fn take_dirty(&'_ self) -> Option<Ref<'_, T>> {
        if self.dirty.replace(false) {
            Some(self.value.borrow())
        } else {
            None
        }
    }
}
