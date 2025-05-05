/// Creates a new [`ChunkedVec`] using a syntax similar to the standard `vec!` macro.
///
/// This macro provides two main ways to create a ChunkedVec:
///
/// # Creating from a list of elements
/// ```
/// # use chunked_vec::chunked_vec;
/// let vec = chunked_vec![1, 2, 3];
/// assert_eq!(vec[0], 1);
/// assert_eq!(vec[1], 2);
/// assert_eq!(vec[2], 3);
/// ```
///
/// # Creating with repeated elements
/// ```
/// # use chunked_vec::chunked_vec;
/// let vec = chunked_vec![1; 3];
/// assert_eq!(vec[0], 1);
/// assert_eq!(vec[1], 1);
/// assert_eq!(vec[2], 1);
/// ```
///
/// # Empty vector
/// ```
/// # use chunked_vec::chunked_vec;
/// use chunked_vec::ChunkedVec;
/// let vec:ChunkedVec<i32> = chunked_vec![];
/// assert_eq!(vec.len(), 0);
/// ```
///
/// # Notes
/// - Like the standard `vec!` macro, this macro works with any type that implements `Clone`
/// - When using `chunked_vec![elem; n]` syntax, the element will be cloned n times
/// - Trailing commas are supported in the list syntax
///
#[macro_export]
macro_rules! chunked_vec {
    () => {
        $crate::ChunkedVec::new()
    };
    ($elem:expr; $n:expr) => {{
        let mut vec = $crate::ChunkedVec::with_capacity($n);
        vec.extend(::std::iter::repeat($elem).take($n));
        vec
    }};
    ($($x:expr),+ $(,)?) => {{
        let mut vec = $crate::ChunkedVec::new();
        $(vec.push($x);)+
        vec
    }};
}

#[cfg(test)]
mod tests {
    use crate::ChunkedVec;

    #[test]
    fn test_empty_chunked_vec() {
        let v: ChunkedVec<i32> = chunked_vec![];
        assert_eq!(v.len(), 0);
    }

    #[test]
    fn test_chunked_vec_with_elements() {
        let v = chunked_vec![1, 2, 3];
        assert_eq!(v.len(), 3);
        assert_eq!(v[0], 1);
        assert_eq!(v[1], 2);
        assert_eq!(v[2], 3);
    }

    #[test]
    fn test_chunked_vec_with_size() {
        let v = chunked_vec![1; 3];
        assert_eq!(v.len(), 3);
        assert_eq!(v[0], 1);
        assert_eq!(v[1], 1);
        assert_eq!(v[2], 1);
    }

    #[test]
    fn test_chunked_vec_with_trailing_comma() {
        let v = chunked_vec![1, 2, 3,];
        assert_eq!(v.len(), 3);
        assert_eq!(v[0], 1);
        assert_eq!(v[1], 2);
        assert_eq!(v[2], 3);
    }
}