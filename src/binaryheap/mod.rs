pub struct BinaryHeap {
    values: Vec<i64>,
}

impl BinaryHeap {
    fn new() -> BinaryHeap {
        BinaryHeap {
            values: Vec::new()
        }
    }

    fn len(&self) -> usize {
        self.values.len()
    }

    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction() {
        let x = BinaryHeap::new();
        assert!(x.is_empty());
        assert_eq!(x.len(), 0);
    }

    #[test]
    fn mut_construction() {
        let mut x = BinaryHeap::new();
        assert!(x.is_empty());
        assert_eq!(x.len(), 0);
    }
}
