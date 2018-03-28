// This file was generated by gir (https://github.com/gtk-rs/gir @ 7f5a2b5)
// from gir-files (https://github.com/gtk-rs/gir-files @ 4740f5e+)
// DO NOT EDIT

use Filter;
use ffi;
use glib::object::Downcast;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct FilterDos2Unix(Object<ffi::GMimeFilterDos2Unix, ffi::GMimeFilterDos2UnixClass>): Filter;

    match fn {
        get_type => || ffi::g_mime_filter_dos2unix_get_type(),
    }
}

impl FilterDos2Unix {
    pub fn new(ensure_newline: bool) -> FilterDos2Unix {
        unsafe {
            Filter::from_glib_full(ffi::g_mime_filter_dos2unix_new(ensure_newline.to_glib())).downcast_unchecked()
        }
    }
}
