use std::iter::FusedIterator;
use crate::IndicatorValue;

#[derive(Debug, Clone)]
#[repr(align(64))]
pub struct CircularBuffer {
    buffer: Vec<Option<IndicatorValue>>,
    index: usize,
    full: bool,
    capacity: usize,
    capacity_1: usize,
}

impl CircularBuffer {
    #[inline]
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            buffer: vec![None; capacity],
            index: 0,
            full: false,
            capacity,
            capacity_1: capacity.saturating_sub(1),
        }
    }

    #[inline]
    fn get_circular_index(&self, index: usize) -> usize {
        debug_assert!(index < self.len(), "Index out of bounds");

        let circular_index: usize = self.index + self.capacity_1 - index;

        circular_index.wrapping_sub((circular_index >= self.capacity) as usize * self.capacity)
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&IndicatorValue> {
        let buf_index = self.get_circular_index(index);
        self.buffer.get(buf_index).and_then(|x| x.as_ref())
    }

    #[inline]
    pub fn push(&mut self, value: IndicatorValue) -> Option<IndicatorValue> {
        let old_value = std::mem::replace(&mut self.buffer[self.index], Some(value));

        self.index += 1;
        if self.index == self.capacity {
            self.index = 0;
            self.full = true;
        }

        old_value
    }

    #[inline]
    pub fn len(&self) -> usize {
        if self.full {
            self.capacity
        } else {
            self.index
        }
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.full
    }

    #[inline]
    pub fn clear(&mut self) {
        for elem in &mut self.buffer {
            *elem = None;
        }
        self.index = 0;
        self.full = false;
    }

    #[inline]
    pub fn iter(&self) -> CircularBufferIterator {
        CircularBufferIterator::new(self)
    }

    #[inline]
    pub fn iter_reversed(&self) -> ReversedCircularBufferIterator {
        ReversedCircularBufferIterator::new(self)
    }
}

#[derive(Copy, Clone)]
pub struct CircularBufferIterator<'a> {
    buffer: &'a CircularBuffer,
    index: usize,
    size: usize,
}

impl<'a> CircularBufferIterator<'a> {
    #[inline]
    pub fn new(buffer: &'a CircularBuffer) -> Self {
        CircularBufferIterator {
            buffer,
            index: 0,
            size: buffer.capacity,
        }
    }
}

impl<'a> Iterator for CircularBufferIterator<'a> {
    type Item = &'a IndicatorValue;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.size {
            return None;
        }

        let value = self.buffer.get(self.index);
        self.index = self.index + 1;
        value
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}

impl<'a> ExactSizeIterator for CircularBufferIterator<'a> {}

impl<'a> FusedIterator for CircularBufferIterator<'a> {}

#[derive(Copy, Clone)]
pub struct ReversedCircularBufferIterator<'a> {
    buffer: &'a CircularBuffer,
    index: isize,
    size: usize,
}

impl<'a> ReversedCircularBufferIterator<'a> {
    #[inline]
    pub fn new(buffer: &'a CircularBuffer) -> Self {
        ReversedCircularBufferIterator {
            buffer,
            index: buffer.capacity_1 as isize,
            size: buffer.capacity,
        }
    }
}

impl<'a> Iterator for ReversedCircularBufferIterator<'a> {
    type Item = &'a IndicatorValue;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < 0 {
            return None;
        }

        let value = self.buffer.get(self.index as usize);
        self.index -= 1;
        value
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}

impl<'a> ExactSizeIterator for ReversedCircularBufferIterator<'a> {}

impl<'a> FusedIterator for ReversedCircularBufferIterator<'a> {}
