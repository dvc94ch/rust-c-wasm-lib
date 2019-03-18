use fib::Fib;
use std::ffi::c_void;

#[repr(C)]
pub struct FibHandle(*mut c_void);

#[no_mangle]
pub unsafe extern "C" fn fib_new() -> FibHandle {
    FibHandle(Box::into_raw(Box::new(Fib::new())) as *mut _)
}

#[no_mangle]
pub unsafe extern "C" fn fib_next(fib: &FibHandle) -> u64 {
    let ptr = fib.0 as *mut Fib;
    match ptr.as_mut() {
        Some(fib) => fib.next().unwrap_or(0),
        None => 0
    }
}

#[no_mangle]
pub unsafe extern "C" fn fib_drop(fib: FibHandle) {
    let ptr = fib.0 as *mut Fib;
    match ptr.as_ref() {
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
                seq.push(fib_next(&fib));
            }
            fib_drop(fib);
            assert_eq!(seq, vec![1, 2, 3, 5, 8, 13, 21]);
        }
    }
}
