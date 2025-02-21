# Rust Mutable Reference Error

This repository demonstrates a common error in Rust: attempting to create multiple mutable references to the same variable.  Rust's borrow checker prevents this to ensure data integrity and prevent race conditions. The `bug.rs` file contains the erroneous code, while `bugSolution.rs` provides a corrected version.