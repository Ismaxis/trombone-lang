use std::alloc::Layout;

pub struct ControlBlock<T: ?Sized> {
    ref_count: usize, // stores number of references minus one
    layout: Layout,   // layout of the control block
    pub value: T,
}

impl<T> ControlBlock<T> {
    #[allow(dead_code)]
    pub fn new(value: T) -> Self {
        Self {
            ref_count: 1,
            layout: Layout::new::<T>(),
            value,
        }
    }

    pub fn from_value_ptr(value: *const T) -> *mut Self {
        let offset = std::mem::offset_of!(ControlBlock<T>, value);
        (value as *const u8).wrapping_sub(offset) as *mut Self
    }

    #[allow(dead_code)]
    pub fn deref(&self) -> &T {
        &self.value
    }

    #[allow(dead_code)]
    pub fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn ref_count(&self) -> usize {
        self.ref_count
    }

    #[allow(dead_code)]
    pub fn increment_ref_count(&mut self) {
        self.ref_count += 1;
    }

    pub fn decrement_ref_count(&mut self) {
        if self.ref_count == 0 {
            panic!("Reference count underflow");
        }
        self.ref_count -= 1;
    }

    pub fn set_layout(&mut self, layout: Layout) {
        self.layout = layout;
    }

    pub fn layout(&self) -> Layout {
        self.layout
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_value_ptr() {
        let value = 42;
        let control_block = ControlBlock::new(value);
        let ptr = control_block.deref() as *const i32;
        let rc_ptr = ControlBlock::from_value_ptr(ptr);
        unsafe {
            assert_eq!((*rc_ptr).ref_count(), 1);
            assert_eq!(*(*rc_ptr).deref(), 42);
        }
    }
}
