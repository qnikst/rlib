// Copyright: 2016 (C) Alexander Vershilov
// License: BSD

//! High level interpreter of the R code.
//!
//! TBD
use std::ffi::{CString};

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
              preserve(result)
        };
        match status {
            ParseStatus::PARSE_OK => Ok(parsed),
            _ => Err(status)
        }
    }

    pub fn try_eval(&self, expression: &SEXP, environment: *mut R) -> Option<SEXP> {
        let mut result : i32 = 0;
        let v = unsafe {
            R_tryEval(expression.asR(), environment, &mut result)
        };
        match result {
            0 => Some(preserve(v)),
            _ => None
        }
    }

    pub fn print_value(&self, sexp: &SEXP) {
        unsafe {
            Rf_PrintValue(sexp.asR());
        }
    }

    pub fn global_env(&self) -> *mut R {
        R_GlobalEnv
    }

    pub fn lcons(&self, a: &SEXP, b: &SEXP) -> SEXP {
        unsafe {
            preserve(Rf_lcons(a.asR(), b.asR()))
        }
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
