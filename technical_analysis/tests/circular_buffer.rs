#[cfg(test)]

mod tests {
    // tests/circular_buffer.rs

    use technical_analysis::CircularBuffer;

    #[test]
    fn test_new_buffer() {
        let capacity = 4;
        let buffer = CircularBuffer::new(capacity);

        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.capacity(), capacity);
        assert!(!buffer.is_full());
    }

    #[test]
    fn test_push_and_get() {
        let mut buffer = CircularBuffer::new(4);

        buffer.push(1.0.into());
        buffer.push(2.0.into());
        buffer.push(3.0.into());

        assert_eq!(buffer.get(0), Some(&3.0.into()));
        assert_eq!(buffer.get(1), Some(&2.0.into()));
        assert_eq!(buffer.get(2), Some(&1.0.into()));
    }

    #[test]
    fn test_push_overflow() {
        let mut buffer = CircularBuffer::new(3);

        buffer.push(1.0.into());
        buffer.push(2.0.into());
        buffer.push(3.0.into());
        buffer.push(4.0.into());

        assert_eq!(buffer.get(0), Some(&4.0.into()));
        assert_eq!(buffer.get(1), Some(&3.0.into()));
        assert_eq!(buffer.get(2), Some(&2.0.into()));
    }

    #[test]
    fn test_clear() {
        let mut buffer = CircularBuffer::new(4);

        buffer.push(1.0.into());
        buffer.push(2.0.into());

        assert_eq!(buffer.len(), 2);

        buffer.clear();
        assert_eq!(buffer.len(), 0);
        assert!(!buffer.is_full());
    }

    #[test]
    fn test_iterator() {
        let mut buffer = CircularBuffer::new(4);

        buffer.push(1.0.into());
        buffer.push(2.0.into());
        buffer.push(3.0.into());
        
        let mut iter = buffer.iter();
        assert_eq!(iter.next(), Some(&3.0.into()));
        assert_eq!(iter.next(), Some(&2.0.into()));
        assert_eq!(iter.next(), Some(&1.0.into()));
    }

    #[test]
    fn test_reversed_iterator() {
        let mut buffer = CircularBuffer::new(3);

        buffer.push(1.0.into());
        buffer.push(2.0.into());
        buffer.push(3.0.into());

        let mut iter = buffer.iter_reversed();
        assert_eq!(iter.next(), Some(&1.0.into()));
        assert_eq!(iter.next(), Some(&2.0.into()));
        assert_eq!(iter.next(), Some(&3.0.into()));
    }

    #[test]
    fn test_full_buffer_push_and_iter() {
        let mut buffer = CircularBuffer::new(3);

        buffer.push(1.0.into());
        buffer.push(2.0.into());
        buffer.push(3.0.into());
        buffer.push(4.0.into());

        let mut iter = buffer.iter();
        assert_eq!(iter.next(), Some(&4.0.into()));
        assert_eq!(iter.next(), Some(&3.0.into()));
        assert_eq!(iter.next(), Some(&2.0.into()));
    }

    #[test]
    fn test_full_buffer_push_and_reversed_iter() {
        let mut buffer = CircularBuffer::new(3);

        buffer.push(1.0.into());
        buffer.push(2.0.into());
        buffer.push(3.0.into());
        buffer.push(4.0.into());

        let mut iter = buffer.iter_reversed();
        assert_eq!(iter.next(), Some(&2.0.into()));
        assert_eq!(iter.next(), Some(&3.0.into()));
        assert_eq!(iter.next(), Some(&4.0.into()));
    }

    #[test]
    fn test_is_full() {
        let mut buffer = CircularBuffer::new(2);

        assert!(!buffer.is_full());
        buffer.push(1.0.into());
        buffer.push(2.0.into());
        assert!(buffer.is_full());
        buffer.push(3.0.into());
        assert!(buffer.is_full());
    }

}