pub struct Fib {
    curr: u64,
    next: u64,
}

impl Fib {
    pub fn new() -> Self {
        Fib { curr: 1, next: 1 }
    }
}

impl Iterator for Fib {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_fib() {
        let seq: Vec<u64> = Fib::new().take(7).collect();
        assert_eq!(seq, vec![1, 2, 3, 5, 8, 13, 21]);
    }
}
