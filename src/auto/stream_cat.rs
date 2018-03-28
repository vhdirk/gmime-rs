// This file was generated by gir (https://github.com/gtk-rs/gir @ 7f5a2b5)
// from gir-files (https://github.com/gtk-rs/gir-files @ 4740f5e+)
// DO NOT EDIT

use Stream;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct StreamCat(Object<ffi::GMimeStreamCat, ffi::GMimeStreamCatClass>): Stream;

    match fn {
        get_type => || ffi::g_mime_stream_cat_get_type(),
    }
}

impl StreamCat {
    pub fn new() -> StreamCat {
        unsafe {
            Stream::from_glib_full(ffi::g_mime_stream_cat_new()).downcast_unchecked()
        }
    }
}

impl Default for StreamCat {
    fn default() -> Self {
        Self::new()
    }
}

pub trait StreamCatExt {
    fn add_source<P: IsA<Stream>>(&self, source: &P) -> i32;
}

impl<O: IsA<StreamCat>> StreamCatExt for O {
    fn add_source<P: IsA<Stream>>(&self, source: &P) -> i32 {
        unsafe {
            ffi::g_mime_stream_cat_add_source(self.to_glib_none().0, source.to_glib_none().0)
        }
    }
}
