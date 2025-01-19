mod matrix;
mod vector;
mod metrics;

pub use matrix::{Matrix, multiply}; 
pub use vector::{dot_product, Vector};
pub use metrics::Metrics;