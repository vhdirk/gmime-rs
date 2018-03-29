// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff+)
// from gir-files (https://github.com/gtk-rs/gir-files @ b215ee8)
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
    pub struct FilterUnix2Dos(Object<ffi::GMimeFilterUnix2Dos, ffi::GMimeFilterUnix2DosClass>): Filter;

    match fn {
        get_type => || ffi::g_mime_filter_unix2dos_get_type(),
    }
}

impl FilterUnix2Dos {
    pub fn new(ensure_newline: bool) -> FilterUnix2Dos {
        unsafe {
            Filter::from_glib_full(ffi::g_mime_filter_unix2dos_new(ensure_newline.to_glib())).downcast_unchecked()
        }
    }
}
