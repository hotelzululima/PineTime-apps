//! LittlevGL (LVGL) API for Rust. Contains Rust bindings for LittlevGL API for C, generated by `bindgen`.
//! Also includes safe versions of LittlevGL APIs created specially for Rust.

#![no_std]                        //  Don't link with standard Rust library, which is not compatible with embedded systems
#![feature(const_transmute)]      //  Allow `transmute` for initialising Lvgl structs
#![feature(trace_macros)]         //  Enable tracing of macros
#![feature(proc_macro_hygiene)]   //  Allow proc macros to be unhygienic

extern crate macros as lvgl_macros;  //  Import Procedural Macros from `macros` library

#[allow(unused_imports)]          //  Allow unused imports
pub mod core;                     //  Lvgl Core API. Export folder `core` as Rust module `lvgl::core`

//  TODO: pub mod draw;           //  Lvgl Draw API. Export folder `draw` as Rust module `lvgl::draw`
//  TODO: pub mod font;           //  Lvgl Font API. Export folder `font` as Rust module `lvgl::font`
//  TODO: pub mod hal;            //  Lvgl HAL API. Export folder `hal` as Rust module `lvgl::hal`
//  TODO: pub mod misc;           //  Lvgl Misc API. Export folder `misc` as Rust module `lvgl::misc`

#[allow(unused_imports)]          //  Allow unused imports
pub mod objx;                     //  Lvgl Objx API. Export folder `objx` as Rust module `lvgl::objx`

//  TODO: pub mod themes;         //  Lvgl Themes API. Export folder `themes` as Rust module `lvgl::themes`

#[macro_use]                      //  Allow macros from Rust module `util`
pub mod util;                     //  Lvgl Utility API. Export folder `util` as Rust module `lvgl::util`

pub mod console;                  //  Lvgl Console API. Export `console.rs` as Rust module `lvgl::console`

/// Return type and error codes for Lvgl API
pub mod result {
    /// Common return type for Lvgl API.  If no error, returns `Ok(val)` where val has type T.
    /// Upon error, returns `Err(err)` where err is the LvglError error code.
    pub type LvglResult<T> = ::core::result::Result<T, LvglError>;

    /// Error codes for Lvgl API
    #[repr(i32)]
    #[derive(PartialEq)]
    #[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
    pub enum LvglError {
        /// Error code 0 means no error.
        SYS_EOK         = 0,
        SYS_EUNKNOWN    = 1,
    }

    /// Cast `LvglError` to `i32`
    impl From<LvglError> for i32 {
        /// Cast `LvglError` to `i32`
        fn from(err: LvglError) -> Self {
            err as i32
        }
    }

    /// Cast `i32` to `LvglError`
    impl From<i32> for LvglError {
        /// Cast `i32` to `LvglError`
        fn from(num: i32) -> Self {
            unsafe { 
                ::core::mem::transmute::
                    <i32, LvglError>
                    (num)
            }  
        }
    }

    /// Cast `()` to `LvglError`
    impl From<()> for LvglError {
        /// Cast `()` to `LvglError`
        fn from(_: ()) -> Self {
            LvglError::SYS_EUNKNOWN
        }
    }

    /// Implement formatted output for LvglError
    impl core::fmt::Debug for LvglError {
        fn fmt(&self, _fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            //  TODO
            Ok(())
        }
    }
}

/// Represents a null-terminated string, suitable for passing to Lvgl APIs as `* const char`.
/// The string could be a null-terminated byte string created in Rust, or a pointer to a null-terminated string returned by C.
/// Pointer may be null.
#[derive(Clone, Copy)]  //  Strn may be copied
pub struct Strn {
    /// Either a byte string terminated with null, or a pointer to a null-terminated string
    pub rep: StrnRep
}

/// Either a byte string or a string pointer
#[derive(Clone, Copy)]  //  StrnRep may be copied
#[repr(u8)]
pub enum StrnRep {
    /// Byte string terminated with null
    ByteStr(&'static [u8]),
    /// Pointer to a null-terminated string
    CStr(*const u8),
}

impl Strn {
    /// Create a new `Strn` with a byte string. Fail if the last byte is not zero.
    /// ```
    /// Strn::new(b"network\0")
    /// strn!("network")
    /// ```
    pub fn new(bs: &'static [u8]) -> Strn {
        assert_eq!(bs.last(), Some(&0u8), "no null");  //  Last byte must be 0.
        Strn { 
            rep: StrnRep::ByteStr(bs)
        }
    }

    /// Create a new `Strn` with a null-terminated string pointer returned by C.
    pub fn from_cstr(cstr: *const u8) -> Strn {
        Strn { 
            rep: StrnRep::CStr(cstr)
        }
    }

    /// Return a pointer to the string
    pub fn as_ptr(&self) -> *const u8 {
        match self.rep {
            StrnRep::ByteStr(bs) => { bs.as_ptr() }
            StrnRep::CStr(cstr)  => { cstr }
        }
    }

    /// Return the length of the string, excluding the terminating null. For safety, we limit to 128.
    pub fn len(&self) -> usize {
        match self.rep {
            StrnRep::ByteStr(bs) => { 
                assert_eq!(bs.last(), Some(&0u8), "no null");  //  Last byte must be 0.
                bs.len() - 1  //  Don't count the terminating null.
            }
            StrnRep::CStr(cstr)  => { 
                //  Look for the null termination.
                if cstr.is_null() { return 0; }
                for len in 0..127 {
                    let ptr: *const u8 =  ((cstr as u32) + len) as *const u8;
                    if unsafe { *ptr } == 0 { return len as usize; }                    
                }
                assert!(false, "big strn");  //  String too long
                return 128 as usize;
            }
        }
    }

    /// Return true if the string is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return the byte string as a null-terminated `* const char` C-style string.
    /// Fail if the last byte is not zero.
    pub fn as_cstr(&self) -> *const u8 {
        match self.rep {
            StrnRep::ByteStr(bs) => { 
                assert_eq!(bs.last(), Some(&0u8), "no null");  //  Last byte must be 0.
                bs.as_ptr() as *const u8
            }
            StrnRep::CStr(cstr)  => { cstr }
        }
    }

    /// Return the byte string.
    /// Fail if the last byte is not zero.
    pub fn as_bytestr(&self) -> &'static [u8] {
        match self.rep {
            StrnRep::ByteStr(bs) => {                
                assert_eq!(bs.last(), Some(&0u8), "no null");  //  Last byte must be 0.
                &bs
            }
            StrnRep::CStr(_cstr)  => { 
                assert!(false, "strn cstr");  //  Not implemented
                b"\0"
            }
        }
    }

    /// Fail if the last byte is not zero.
    pub fn validate(&self) {
        match self.rep {
            StrnRep::ByteStr(bs) => {         
                assert_eq!(bs.last(), Some(&0u8), "no null");  //  Last byte must be 0.
            }
            StrnRep::CStr(_cstr)  => {}
        }
    }

    /// Fail if the last byte is not zero.
    pub fn validate_bytestr(bs: &'static [u8]) {
        assert_eq!(bs.last(), Some(&0u8), "no null");  //  Last byte must be 0.
    }
}

///  Allow threads to share Strn, since it is static.
unsafe impl Send for Strn {}

///  Allow threads to share Strn, since it is static.
unsafe impl Sync for Strn {}

///  Declare a pointer that will be used by C functions to return a value
pub type Out<T> = &'static mut T;

///  Declare a `void *` pointer that will be passed to C functions
pub type Ptr = *mut ::cty::c_void;

///  Declare a `NULL` pointer that will be passed to C functions
pub const NULL: Ptr = ::core::ptr::null_mut();
