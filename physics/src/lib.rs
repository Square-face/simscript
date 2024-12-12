//! Library for handling physics calculations in SimScript
//!
//! This library provides structures for relevant physical quantities, such as
//! forces, acceleration, time, and more. The goal is to define a struct for
//! each of these quantities. Currently, only a few are implemented, and there
//! is no comprehensive list of all the quantities that need to be added.
//!
//! ## Local and Global Typestate
//!
//! Many quantities also include a typestate to differentiate between local and
//! global coordinate spaces. 
//! - **Local space**: Refers to a coordinate system that rotates with the object it is associated with.
//! - **Global space**: Refers to a fixed, world-aligned coordinate system.
//!
//! The purpose of this design is to simplify distinguishing between these two
//! types of coordinate systems without requiring runtime checks, such as those
//! involving enums. Additionally, we leverage Rust's type system to enforce
//! correctness at compile time.
//!
//! ## Operations on Quantities
//!
//! This library provides trait implementations that allow you to perform
//! arithmetic operations on physical quantities, producing the correct
//! resulting quantity. For example:
//! - Multiplying `Acceleration` with `Time` yields a `Velocity`.
//!
//! These operations are designed to align with the rules of physics and ensure
//! correctness at the type level. By encoding these relationships into the type
//! system, the library helps prevent invalid operations and provides intuitive
//! support for working with physical calculations.
pub mod components;
pub mod coordinate_systems;
