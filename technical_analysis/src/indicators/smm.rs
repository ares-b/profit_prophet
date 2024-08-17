use std::collections::BinaryHeap;
use std::cmp::Reverse;
use crate::indicators::Indicator;
use crate::{CircularBuffer, IndicatorValue};

#[derive(Debug, Clone)]
pub struct SimpleMovingMedian {
    buffer: CircularBuffer,
    max_heap: BinaryHeap<IndicatorValue>,
    min_heap: BinaryHeap<Reverse<IndicatorValue>>,
}

impl SimpleMovingMedian {
    #[inline]
    pub fn new(period: usize) -> Self {
        SimpleMovingMedian {
            buffer: CircularBuffer::new(period),
            max_heap: BinaryHeap::with_capacity(period / 2 + 1),
            min_heap: BinaryHeap::with_capacity(period / 2 + 1),
        }
    }

    #[inline]
    fn add_value(&mut self, value: IndicatorValue) {
        // Use a single comparison to decide where to insert
        if let Some(&max_top) = self.max_heap.peek() {
            if value <= max_top {
                self.max_heap.push(value);
            } else {
                self.min_heap.push(Reverse(value));
            }
        } else {
            self.max_heap.push(value);
        }

        self.balance_heaps();
    }

    #[inline]
    fn remove_value(&mut self, value: IndicatorValue) {
        // Attempt to remove from max_heap, then from min_heap
        let max_top = self.max_heap.peek();
        let min_top = self.min_heap.peek().map(|x| x.0);

        if max_top == Some(&value) {
            self.max_heap.pop();
        } else if min_top == Some(value) {
            self.min_heap.pop();
        } else {
            // If not at the top of either heap, reconstruct
            self.reconstruct_heaps(value);
        }

        self.balance_heaps();
    }

    #[inline]
    fn reconstruct_heaps(&mut self, value: IndicatorValue) {
        self.max_heap = self.max_heap
            .drain()
            .filter(|&x| x != value)
            .collect();

        self.min_heap = self.min_heap
            .drain()
            .filter(|&Reverse(x)| x != value)
            .collect();
    }

    #[inline]
    fn balance_heaps(&mut self) {
        if self.max_heap.len() > self.min_heap.len() + 1 {
            self.min_heap.push(Reverse(self.max_heap.pop().unwrap()));
        } else if self.min_heap.len() > self.max_heap.len() {
            self.max_heap.push(self.min_heap.pop().unwrap().0);
        }
    }

    #[inline]
    fn calculate_median(&self) -> IndicatorValue {
        match self.max_heap.len().cmp(&self.min_heap.len()) {
            std::cmp::Ordering::Greater => *self.max_heap.peek().unwrap(),
            std::cmp::Ordering::Less => self.min_heap.peek().unwrap().0,
            std::cmp::Ordering::Equal => {
                let max_root = *self.max_heap.peek().unwrap();
                let min_root = self.min_heap.peek().unwrap().0;
                (max_root + min_root) / IndicatorValue::from(2.0)
            }
        }
    }
}

impl Indicator for SimpleMovingMedian {
    type Output = IndicatorValue;
    type Input = IndicatorValue;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        if self.buffer.is_full() {
            if let Some(old_value) = self.buffer.push(input) {
                self.remove_value(old_value);
            }
        } else {
            self.buffer.push(input);
        }

        self.add_value(input);

        self.calculate_median()
    }

    #[inline]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        let mut last_output = IndicatorValue::from(0.0);
        for &value in input {
            last_output = self.next(value);
        }
        last_output
    }

    #[inline]
    fn reset(&mut self) {
        self.buffer.clear();
        self.max_heap.clear();
        self.min_heap.clear();
    }
}
