#[derive(Default)]
pub struct Tracked<T> {
    value: T,
    dirty: bool,
}

impl<T> Tracked<T> {
    pub fn new(value: T) -> Self {
        Self { value, dirty: true }  // start dirty so initial state triggers
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
        self.dirty = true;
    }

    /// Only marks dirty if value actually changed
    pub fn set_if_changed(&mut self, value: T)
    where
        T: PartialEq,
    {
        if self.value != value {
            self.value = value;
            self.dirty = true;
        }
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn take_dirty(&mut self) -> Option<&T> {
        if std::mem::take(&mut self.dirty) {
            Some(&self.value)
        } else {
            None
        }
    }
}
