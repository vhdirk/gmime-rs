// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff+)
// from gir-files (https://github.com/gtk-rs/gir-files @ 33386b3)
// DO NOT EDIT

use Filter;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct FilterBest(Object<ffi::GMimeFilterBest, ffi::GMimeFilterBestClass>): Filter;

    match fn {
        get_type => || ffi::g_mime_filter_best_get_type(),
    }
}

impl FilterBest {
    //pub fn new(flags: /*Ignored*/FilterBestFlags) -> FilterBest {
    //    unsafe { TODO: call ffi::g_mime_filter_best_new() }
    //}
}

pub trait FilterBestExt {
    fn charset(&self) -> Option<String>;

    //fn encoding(&self, constraint: /*Ignored*/EncodingConstraint) -> /*Ignored*/ContentEncoding;
}

impl<O: IsA<FilterBest>> FilterBestExt for O {
    fn charset(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_mime_filter_best_charset(self.to_glib_none().0))
        }
    }

    //fn encoding(&self, constraint: /*Ignored*/EncodingConstraint) -> /*Ignored*/ContentEncoding {
    //    unsafe { TODO: call ffi::g_mime_filter_best_encoding() }
    //}
}
