//! Question #19: Mutex with Spin-then-Block
//! Category: Concurrency
//! Difficulty: Hard
//! Concepts: mutex, spin-then-block, futex, latency
//! Description: Design a mutex that spins briefly in userspace and only then issues a system call to park the thread.
//!
//! TODO: Implement this solution.

pub fn mutex_with_spin_then_block() {
    // Implementation goes here.
    // See questions.json for full question details.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutex_with_spin_then_block() {
        // TODO: Write tests for question #19
    }
}
