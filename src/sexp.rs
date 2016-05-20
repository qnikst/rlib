// Copyright: 2016 (C) Alexander Vershilov
// License: BSD
//! High level S Expression support
//!
//! Highlevel type that hide all R complexity from the user,
//! without introducing any performance overhead.
//!
//! # Regions and memory safety
//!
//! TBD
use internal::*;

/// S-Expression that wraps mutable R variable.
///
/// See GC section for more details.
pub struct SEXP(pub *mut R);

impl Drop for SEXP {
    fn drop(&mut self) {
        println!("DROP!");
        unsafe {
            Rf_unprotect(1);
        }
    }
}

/// /O(1)/ Protect R object for later use.
///
/// Protect is used to wrap raw R object into SEXP and add
/// value to the protection stack, this guarantee that value
/// will not be freed by R GC until we will not exit rust (!) scope.
///
/// See GC section for more details.
pub fn protect(sexp:*mut R) -> SEXP {
    unsafe { SEXP(Rf_protect(sexp)) }
}

/// Iterator over values.
pub struct RIterator {
    current: i32,
    length: i32,
    sexp: *mut R,
}

impl Iterator for RIterator {
    type Item = SEXP;

    fn next(&mut self) -> Option<SEXP> {
        if self.current < self.length {
            let c = self.current;
            self.current+=1;
            unsafe {
                Some(protect(VECTOR_ELT(self.sexp, c))) //here we should just return *R with corrent lifetime
            }
        } else {
            None
        }
    }

}

impl IntoIterator for SEXP {
    type Item = SEXP;
    type IntoIter = RIterator;
    fn into_iter(self) -> Self::IntoIter {
        let SEXP(ptr) = self;
        let l = unsafe { Rf_length(ptr) };
        RIterator{current: 0, length: l, sexp:ptr}
    }
}
