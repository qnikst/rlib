// Copyright: 2016 (C) Alexander Vershilov
// License: BSD

//! High level interpreter of the R code.
//!
//! TBD
use std::ffi::{CString};
use libc::c_void;

use sexp::*;
use internal::*;

// TODO: implement some RC counter?

/// Interpreter structure
pub struct Interpreter{
  /// `.Call` symbol from R runtime
  pub sexp_call: *mut R,
  /// `function` symbol from R runtime
  pub sexp_function: *mut R,
  /// `native` symbol from R runtime
  pub sexp_nativesym: *mut R
}

impl Drop for Interpreter {
    fn drop(&mut self) {
       unsafe { Rf_endEmbeddedR(0); }
    }
}

impl Interpreter {
    pub fn parse(&self, text: &str, max_results: i32) -> Result<SEXP, ParseStatus> {
        let cstr = CString::new(text).unwrap();
        let mut status = ParseStatus::PARSE_NULL;
        let parsed = unsafe {
              let str = Rf_protect(Rf_mkString(cstr.as_ptr()));
              let result = R_ParseVector(str, max_results, &mut status, R_NilValue);
              Rf_unprotect(1);
              protect(result)
        };
        match status {
            ParseStatus::PARSE_OK => Ok(parsed),
            _ => Err(status)
        }
    }

    pub fn try_eval(&self, expression: &SEXP, environment: *mut R) -> Option<SEXP> {
        let SEXP(pexpression) = *expression;
        let mut result : i32 = 0;
        let v = unsafe {
            R_tryEval(pexpression, environment, &mut result)
        };
        match result {
            0 => Some(protect(v)),
            _ => None
        }
    }

    pub fn print_value(&self, sexp: &SEXP) {
        let SEXP(ptr) = *sexp;
        unsafe {
            Rf_PrintValue(ptr);
        }
    }

    pub fn global_env(&self) -> *mut R {
        R_GlobalEnv
    }

    pub fn lcons(&self, a: &SEXP, b: &SEXP) -> SEXP {
        let SEXP(pa) = *a;
        let SEXP(pb) = *b;
        unsafe {
            protect(Rf_lcons(pa, pb))
        }
    }

    pub fn wrap_static(&self, pf: extern "C" fn(*mut c_void) -> (*mut c_void)) -> SEXP {
        let value = unsafe {
            let SEXP(out) = protect(R_MakeExternalPtr(pf, self.sexp_nativesym, R_NilValue));
            let SEXP(value) = protect(Rf_lang3(self.sexp_call, out, R_DotsSymbol));
            let SEXP(formals) = protect(Rf_cons(R_MissingArg, R_NilValue));
            SET_TAG(formals, R_DotsSymbol);
            let SEXP(result) = protect(Rf_lang4(self.sexp_function, formals, value, R_NilValue));
            result
        };
        protect(value)
    }


}

// TODO: add parameters and config.
pub fn new() -> Option<Interpreter> {
   let cargs :Vec<_> = ["example","--slave","--vanilla"].iter().map(|&x| CString::new(x).unwrap().as_ptr()).collect();
   unsafe {
     Rf_initEmbeddedR(cargs.len() as i32, cargs.as_ptr());
   }
   let s_call = unsafe { Rf_install(CString::new(".Call").unwrap().as_ptr()) };
   let s_function = unsafe { Rf_install(CString::new("function").unwrap().as_ptr()) };
   let s_native = unsafe { Rf_install(CString::new("native symbol").unwrap().as_ptr()) };
   Some(Interpreter{ sexp_call: s_call
                   , sexp_function: s_function
                   , sexp_nativesym: s_native})
}

