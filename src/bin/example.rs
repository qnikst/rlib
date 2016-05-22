// Copyright: 2016 (C) Alexander Vershilov
// License: BSD
extern crate rlib;
extern crate libc;


use rlib::interpreter;
use rlib::sexp::{preserve, AsR, double};
use rlib::internal::*;

use std::mem;

#[no_mangle]
pub extern "C" fn fn_test(a: *mut R, b: *mut R) -> *mut R {
    unsafe {
      let result = preserve(Rf_allocVector(SEXPTYPE::REALSXP, 1));
      double(&result).unwrap()[0] = Rf_asReal(a) + Rf_asReal(b);
      result.asR()
    }
}

fn main() {
    let r = interpreter::new().unwrap();
    let parsed = r.parse("1+2; gctorture(TRUE);", -1).unwrap();
    for e in parsed {
        println!("Eval>>");
        let output = r.try_eval(&e, r.global_env()).unwrap();
        r.print_value(&output);
    };
    unsafe {
        let ffn = mem::transmute(fn_test);
        let ext = preserve(R_MakeExternalPtr(ffn, r.sexp_nativesym, R_NilValue));
        let a3  = preserve(Rf_allocVector(SEXPTYPE::REALSXP, 1));
        double(&a3).unwrap()[0] = 3.0;

        let a5  = preserve(Rf_allocVector(SEXPTYPE::REALSXP, 1));
        double(&a5).unwrap()[0] = 5.0;

        let val = preserve(Rf_lang4(r.sexp_call,ext.asR(), a3.asR(), a5.asR()));

        let output = r.try_eval(&val, r.global_env()).unwrap();
        r.print_value(&output);

        // Doesn't work
        let z = preserve({ 
            let a7  = preserve(Rf_allocVector(SEXPTYPE::REALSXP, 1));
            double(&a7).unwrap()[0] = 7.0;
            a7.asR()
            });
        let a100  = preserve(Rf_allocVector(SEXPTYPE::REALSXP, 100));
        let val = preserve(Rf_lang4(r.sexp_call, ext.asR(), a3.asR(), z.asR()));
        let output = r.try_eval(&val, r.global_env()).unwrap();
        r.print_value(&output);
    }

}

