// Copyright: 2016 (C) Alexander Vershilov
// License: BSD
//! Internal lowest level bindings that are used for 
//! communication with R runtime. Nothing is safe here.
//!
//! This module is used internally by the library or in the
//! case usecase can't be represented using a higher level
//! approach.
//!
//! This module may be not carefuly documented and is a subject
//! of change even in the minor versions, use it at your own risk.
use libc::{c_void, c_double};

/// Opaque pointer to the R value.
pub enum R {}

/// Represent result of a parse.
#[repr(C)]
#[derive(Debug)]
pub enum ParseStatus {
    /// Parse had not happened.
    PARSE_NULL,
    /// Parse successful.
    PARSE_OK,
    /// Parse is incomplete.
    PARSE_INCOMPLETE,
    /// Error during parse.
    PARSE_ERROR,
    /// End of file during parse.
    PARSE_EOF    
}

/// Type of the R Value
#[repr(C)]
#[derive(Debug)]
pub enum SEXPTYPE {
    /// NULL
    NILSXP = 0,
    /// Symbols
    SYMSXP = 1,
    /// List of dotted pairs
    LISTSXP = 2,
    /// Closures
    CLOSXP = 3,
    /// Environments
    ENVSXP = 4,
    /// Promises (unevaluated closure arguments)
    PROMSXP = 5,
    /// Language constructs (special lists)
    LANGSXP = 6,
    /// Special forms
    SPECIALSXP = 7,
    /// Builtin non-special forms
    BUILINSXP = 8,
    /// "scalar" string type (internal only)
    CHARSXP = 9,
    /// Logical vector
    LGLSXP= 10,
    /// Integer vector
    INTSXP = 13,
    /// Real vector
    REALSXP = 14,
    /// Complex vector
    CPLSXP = 15,
    /// String vector
    STRSXP = 16,
    /// dot-dot-dot object
    DOTSXP = 17,
    /// "any" argument
    ANYSXP = 18,
    /// generic vector
    VECSXP = 19,
    /// Expression vector
    EPRSXP = 20,
    /// Bytecode
    BCODESXP = 21,
    /// External pointer
    EXTPRSXP = 22,
    /// Weak reference
    WEAKREFSXP = 23,
    /// Raw bytes
    RAWSXP = 24,
    /// S4 non-vector
    S4SXP = 25,
    /// Fresh node created in new page
    NEWSXP = 30,
    /// node released by GC
    FREESXP = 31,
    /// Closure or Builtin
    FUNSXP = 99
}

#[link(name="R")]
extern {
    /// Initialize R runtime.
    ///
    /// # Panic
    /// This function will panic if runtime is already initilized.
    pub fn Rf_initEmbeddedR(argc: i32, d: *const (*const i8) /*CString*/);
    pub fn Rf_endEmbeddedR(fatal: i32);

    pub fn R_ParseVector(expr: *const R, max: i32, c_result: *mut ParseStatus, env: *const R) -> *mut R;
    pub fn R_tryEval(expression: *const R, environment: *mut R, result: *mut i32) -> *mut R;

    pub fn Rf_mkString(input: *const i8 /*CString*/) -> *mut R;
    pub fn Rf_allocVector(type_: SEXPTYPE, size: i32) -> *mut R;

    pub fn R_MakeExternalPtr(f: extern "C" fn(*mut c_void) -> *mut c_void, a: *const R, b: *const R) -> *mut R;

    pub fn Rf_protect(input: *mut R) -> *mut R;
    pub fn Rf_unprotect(num: i32) -> ();
    pub fn Rf_PrintValue(sexp: *const R);
    pub fn Rf_eval(expression: *const R, environment: *mut R) -> *mut R;

    pub fn Rf_install(query: *const i8 /* CString */) -> *mut R;

    /// Find length of the R value.
    pub fn Rf_length(sexp: *const R) -> i32;
    pub fn VECTOR_ELT(sexp: *const R, n: i32) -> *mut R;

    pub fn Rf_cons(head: *const R, tail: *const R) -> *mut R;
    pub fn Rf_lcons(head: *const R, tail: *const R) -> *mut R;
    pub fn SET_TAG(a: *const R, b: *const R);

    pub fn Rf_lang3(a: *const R, b: *const R, c: *const R) -> *mut R;
    pub fn Rf_lang4(a: *const R, b: *const R, c: *const R, d: *const R) -> *mut R;

    //access
    pub fn Rf_asReal(a: *const R) -> c_double;

    // SEXP access
    pub fn REAL(a: *const R) -> *mut c_double;

    pub static R_Interactive: i32;
    pub static R_NilValue: *const R;
    pub static R_GlobalEnv: *mut R;
    pub static R_DotsSymbol: *mut R;
    pub static R_MissingArg: *mut R;
}
