//! ChunkedVec is a vector-like data structure that stores elements in fixed-size chunks.
//!
//! # Features
//! - Fixed-size chunk-based storage for better memory management
//! - Standard vector-like interface
//! - Index-based access with bounds checking
//!
//! # Example
//! ```
//! use chunked_vec::ChunkedVec;
//!
//! let mut vec = ChunkedVec::<i32>::new();
//! vec.push(1);
//! vec.push(2);
//! assert_eq!(vec[0], 1);
//! assert_eq!(vec.len(), 2);
//! ```

const DEFAULT_CHUNK_SIZE: usize = 64;

mod chunked_vec;
mod constructors;
mod drop;
mod index;
pub(crate) mod internal;
mod iterators;
mod operations;
mod traits;

pub use chunked_vec::*;
