mod type_of;

/// Custom lightweight random value generator, inspired from `fastrand`.
pub mod rand;

/// Debugging utilities for contributors
pub mod debug {
    pub use super::type_of::*;
}
