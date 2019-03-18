use fib::Fib;

#[no_mangle]
pub unsafe extern "C" fn fib_new() -> *mut Fib {
    Box::into_raw(Box::new(Fib::new()))
}

#[no_mangle]
pub unsafe extern "C" fn fib_next(fib: *mut Fib) -> u64 {
    match fib.as_mut() {
        Some(fib) => fib.next().unwrap_or(0),
        None => 0
    }
}

#[no_mangle]
pub unsafe extern "C" fn fib_drop(fib: *mut Fib) {
    match fib.as_ref() {
        Some(fib) => drop(fib),
        None => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_fib() {
        unsafe {
            let fib = fib_new();
            let mut seq = vec![];
            for _ in 0..7 {
                seq.push(fib_next(fib));
            }
            fib_drop(fib);
            assert_eq!(seq, vec![1, 2, 3, 5, 8, 13, 21]);
        }
    }
}
