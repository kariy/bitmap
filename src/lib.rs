#[derive(Debug, Clone)]
pub struct BitMap(Vec<bool>);

impl BitMap {
    pub fn new() -> Self {
        BitMap(Vec::new())
    }
}

impl BitMap {
    pub fn insert(&mut self, num: usize) {
        let index: usize = num;
        if let Some(bit) = self.0.get_mut(index) {
            *bit = true;
        } else {
            // grow the vec if the bit array is smaller than the index
            if self.0.len() < index {
                self.0.resize(index, false);
            }
            self.0.insert(index, true);
        }
    }

    pub fn remove(&mut self, num: usize) -> Option<()> {
        if let Some(bit) = self.0.get_mut(num as usize) {
            *bit = false;
            Some(())
        } else {
            None
        }
    }

    pub fn contains(&self, num: usize) -> bool {
        self.0.get(num as usize).copied().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use crate::BitMap;

    #[test]
    fn test_insert() {
        let mut bitmap = BitMap::new();

        bitmap.insert(8);
        assert!(bitmap.contains(8));
        assert!(!bitmap.contains(1));
        assert!(!bitmap.contains(10));
        assert_eq!(bitmap.0.len(), 9, "inner vec should be resized");

        bitmap.insert(20);
        assert!(bitmap.contains(8));
        assert!(bitmap.contains(20));
        assert!(!bitmap.contains(1));
        assert!(!bitmap.contains(10));
        assert_eq!(bitmap.0.len(), 21, "inner vec should be resized");
    }

    #[test]
    fn test_insert_and_remove() {
        let mut bitmap = BitMap::new();

        bitmap.insert(8);
        assert!(bitmap.contains(8));
        assert!(!bitmap.contains(10));
        assert_eq!(bitmap.0.len(), 9, "inner vec should be resized");

        bitmap.remove(8);
        assert!(!bitmap.contains(8));
        assert!(!bitmap.contains(10));
        assert_eq!(bitmap.0.len(), 9, "inner vec size should maintain");
    }
}
