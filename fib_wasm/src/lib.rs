use fib::Fib;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Fib)]
pub struct JsFib {
    fib: Fib,
}

#[wasm_bindgen(js_class = Fib)]
impl JsFib {
    pub fn new() -> Self {
        JsFib { fib: Fib::new() }
    }

    pub fn next(&mut self) -> u64 {
        self.fib.next().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_fib() {
        let mut fib = JsFib::new();
        let mut seq = vec![];
        for _ in 0..7 {
            seq.push(fib.next());
        }
        assert_eq!(seq, vec![1, 2, 3, 5, 8, 13, 21]);
    }
}
