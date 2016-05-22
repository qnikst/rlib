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
pub struct SEXP(*mut R);

pub trait AsR {
    fn asR(&self) -> *mut R;
}

impl Drop for SEXP {
    fn drop(&mut self) {
        println!("DROP!");
        let SEXP(x) = *self;
        unsafe {
            R_ReleaseObject(x);
        }
    }
}

impl AsR for SEXP {
    fn asR(&self) -> *mut R {
        let SEXP(x) = *self;
        x
    }
}

/// /O(n)/ Protect R object for later use.
///
/// /n/ denotes a number of elements protected in that way.
///
/// Protect is used to wrap raw R object into SEXP and add
/// value to the protection stack, this guarantee that value
/// will not be freed by R GC until we will not exit rust (!) scope.
///
/// See GC section for more details.
pub fn preserve(sexp:*mut R) -> SEXP {
    unsafe { R_PreserveObject(sexp) }
    SEXP(sexp)
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
                Some(preserve(VECTOR_ELT(self.sexp, c))) //here we should just return *R with corrent lifetime
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
