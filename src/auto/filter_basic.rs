// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff+)
// from gir-files (https://github.com/gtk-rs/gir-files @ 33386b3)
// DO NOT EDIT

use Filter;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct FilterBasic(Object<ffi::GMimeFilterBasic, ffi::GMimeFilterBasicClass>): Filter;

    match fn {
        get_type => || ffi::g_mime_filter_basic_get_type(),
    }
}

impl FilterBasic {
    //pub fn new(encoding: /*Ignored*/ContentEncoding, encode: bool) -> FilterBasic {
    //    unsafe { TODO: call ffi::g_mime_filter_basic_new() }
    //}
}