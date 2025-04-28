use crate::ChunkedVec;

pub struct IterMut<'a, T, const N: usize> {
    pub(crate) vec: &'a mut ChunkedVec<T, N>,
    pub(crate) index: usize,
}

impl<T, const N: usize> ChunkedVec<T, N> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T, N> {
        IterMut {
            vec: self,
            index: 0,
        }
    }
}

impl<'a, T, const N: usize> Iterator for IterMut<'a, T, N> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len() {
            let index = self.index;
            self.index += 1;
            unsafe {
                // [TODO] : 没有完全理解
                let ptr = self.vec.get_unchecked_mut(index) as *mut T;
                Some(&mut *ptr)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_mut() {
        let mut vec = ChunkedVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);

        let mut iter = vec.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        let elem = iter.next();
        assert_eq!(elem, Some(&mut 3));
        *elem.unwrap() = 4;
        assert_eq!(iter.next(), None);
        assert_eq!(vec[2], 4);
    }
}
