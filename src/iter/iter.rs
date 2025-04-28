use crate::ChunkedVec;

pub struct Iter<'a, T, const N: usize> {
    pub(crate) vec: &'a ChunkedVec<T, N>,
    pub(crate) index: usize,
}

impl<T, const N: usize> ChunkedVec<T, N> {
    pub fn iter(&self) -> Iter<'_, T, N> {
        Iter {
            vec: self,
            index: 0,
        }
    }
}

impl<'a, T, const N: usize> Iterator for Iter<'a, T, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len() {
            let index = self.index;
            self.index += 1;
            Some(&self.vec[index])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let mut vec = ChunkedVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);

        let mut iter = vec.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }
}
