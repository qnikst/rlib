// Copyright: alexander.vershilov@gmail.com
// License: BSD-3
//! Library for the steamless interporability with R runtime.
//! This library provide several layers that could be used in 
//! a different situations.
//!
//! Lowest layer:
//!
//!   * [internal](internal/index.html)  - are unsafe low level bindings
//!   * [interpreter](interpreter/index.html) - safe highlevel bindings
//!      that hide communication complexity
//!
extern crate libc;

pub mod internal;
pub mod interpreter;
pub mod sexp;

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        println!("Hello world.");
        //let cargs :Vec<_> = std::env::args().map(|x| CString::new(x).unwrap()).collect();
        let cargs :Vec<_> = ["example","--slave","--vanilla"].iter().map(|&x| CString::new(x).unwrap()).collect();
        unsafe {

            Rf_initEmbeddedR(1, cargs.as_ptr());
            let cstr= CString::new("1+2").unwrap();

            let str = Rf_protect(Rf_mkString(cstr.as_ptr()));
            let mut status = ParseStatus::PARSE_NULL;
            let parsed = R_ParseVector(str, -1, &mut status, R_NilValue);
            println!("Parse result: {:?}", status);
            for i in (0..LENGTH(parsed)) {
                println!("Eval result -> {}", i);
                let output = Rf_protect(Rf_eval(VECTOR_ELT(parsed,i), R_GlobalEnv));
                Rf_PrintValue(output);
            }
            println!("inside");
            Rf_endEmbeddedR(0);
        }
    }
}
