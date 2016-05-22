// Copyright: 2016 (C) Alexander Vershilov
// License: BSD
extern crate rlib;
extern crate libc;


use rlib::interpreter;
use rlib::sexp::{SEXP,preserve};
use rlib::internal::*;

use std::mem;

#[no_mangle]
pub extern "C" fn fn_test(a: *mut R, b: *mut R) -> *mut R {
    unsafe {
      let result = preserve(Rf_allocVector(SEXPTYPE::REALSXP, 1));
      let SEXP(r) = result;
      let v = REAL(r);
      *v.offset(0) = Rf_asReal(a) + Rf_asReal(b);
      r
    }
}

fn main() {
    let r = interpreter::new().unwrap();
    //let parsed = r.parse("1+2;", -1).unwrap();
    let parsed = r.parse("1+2; gctorture(TRUE);", -1).unwrap();
    for e in parsed {
        println!("Eval>>");
        let output = r.try_eval(&e, r.global_env()).unwrap();
        r.print_value(&output);
    };
    unsafe {
        println!("transmute");
        let ffn = mem::transmute(fn_test);
        println!("make-ext");
        let ext = preserve(R_MakeExternalPtr(ffn, r.sexp_nativesym, R_NilValue));
        println!("alloc-vec");
        let a3  = preserve(Rf_allocVector(SEXPTYPE::REALSXP, 1));
        println!("unwrap");
        let SEXP(r3) = a3;
        println!("real");
        let d3 = REAL(r3);
        println!("offset");
        *d3.offset(0) = 3.0;

        println!("alloc-vec");
        let a5  = preserve(Rf_allocVector(SEXPTYPE::REALSXP, 1));
        println!("unwrap");
        let SEXP(r5) = a5;
        println!("real");
        let d5 = REAL(r5);
        println!("offset");
        *d5.offset(0) = 5.0;

        println!("unwrap");
        let SEXP(pe) = ext;
        println!("lang4");
        let val = preserve(Rf_lang4(r.sexp_call,pe, r3, r5));

        println!("eval");
        let output = r.try_eval(&val, r.global_env()).unwrap();
        r.print_value(&output);

        // Doesn't work
        let z = preserve({ 
            let a7  = preserve(Rf_allocVector(SEXPTYPE::REALSXP, 1));
            let SEXP(r7) = a7;
            let d7 = REAL(r7);
            *d7.offset(0) = 7.0;
            r7
            });
        let SEXP(pz) = z;
        let a100  = preserve(Rf_allocVector(SEXPTYPE::REALSXP, 100));
        let val = preserve(Rf_lang4(r.sexp_call, pe, r3, pz));
        let output = r.try_eval(&val, r.global_env()).unwrap();
        r.print_value(&output);
    }

}

