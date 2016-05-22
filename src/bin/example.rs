// Copyright: 2016 (C) Alexander Vershilov
// License: BSD
extern crate rlib;
extern crate libc;


use rlib::interpreter;
use rlib::sexp::{preserve, AsR};
use rlib::internal::*;

use std::mem;

#[no_mangle]
pub extern "C" fn fn_test(a: *mut R, b: *mut R) -> *mut R {
    unsafe {
      let result = preserve(Rf_allocVector(SEXPTYPE::REALSXP, 1));
      let v = REAL(result.asR());
      *v.offset(0) = Rf_asReal(a) + Rf_asReal(b);
      result.asR()
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
        let d3 = REAL(a3.asR());
        println!("offset");
        *d3.offset(0) = 3.0;

        println!("alloc-vec");
        let a5  = preserve(Rf_allocVector(SEXPTYPE::REALSXP, 1));
        println!("unwrap");
        let d5 = REAL(a5.asR());
        println!("offset");
        *d5.offset(0) = 5.0;

        println!("unwrap");
        let val = preserve(Rf_lang4(r.sexp_call,ext.asR(), a3.asR(), a5.asR()));

        println!("eval");
        let output = r.try_eval(&val, r.global_env()).unwrap();
        r.print_value(&output);

        // Doesn't work
        let z = preserve({ 
            let a7  = preserve(Rf_allocVector(SEXPTYPE::REALSXP, 1));
            let d7 = REAL(a7.asR());
            *d7.offset(0) = 7.0;
            a7.asR()
            });
        let a100  = preserve(Rf_allocVector(SEXPTYPE::REALSXP, 100));
        let val = preserve(Rf_lang4(r.sexp_call, ext.asR(), a3.asR(), z.asR()));
        let output = r.try_eval(&val, r.global_env()).unwrap();
        r.print_value(&output);
    }

}

