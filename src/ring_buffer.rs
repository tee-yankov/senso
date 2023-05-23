use std::collections::VecDeque;

#[derive(Debug)]
// Technically a FILO queue with a fixed capacity, not super exactly a ring buf
pub struct RingBuf<T> {
    head: usize,
    capacity: usize,
    pub buf: VecDeque<T>,
}

impl<T> From<Vec<T>> for RingBuf<T> {
    fn from(items: Vec<T>) -> Self {
        Self {
            head: items.len(),
            capacity: items.len(),
            buf: VecDeque::from(items),
        }
    }
}

impl<T> RingBuf<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            head: 0,
            capacity,
            buf: VecDeque::with_capacity(capacity),
        }
    }

    pub fn put(&mut self, item: T) {
        // If full, pop first element
        if self.head == self.capacity {
            self.buf.pop_front();
        } else {
            self.head += 1;
        }
        self.buf.push_back(item);
    }

    pub fn get(&self) -> &VecDeque<T> {
        &self.buf
    }

    pub fn len(&self) -> usize {
        self.head
    }
}

mod tests {
    #[allow(unused)]
    use super::RingBuf;

    #[test]
    fn test_ring_buf_from_vec() {
        let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut ring_buf = RingBuf::from(items);

        ring_buf.put(10);
        ring_buf.put(11);
        ring_buf.put(12);
        ring_buf.put(13);
        ring_buf.put(14);
        ring_buf.put(15);

        assert!(ring_buf.len() == 9);
        assert!(ring_buf.get().iter().next() == Some(&7));
        assert!(ring_buf.get().iter().last() == Some(&15));
    }

    #[test]
    fn test_ring_buf_new() {
        let items = vec![0, 1, 2, 3, 4, 5];
        let mut ring_buf = RingBuf::new(2);
        for item in items {
            ring_buf.put(item);
        }

        assert!(ring_buf.len() == 2);
        assert!(ring_buf.get().iter().last() == Some(&5));
    }
}
