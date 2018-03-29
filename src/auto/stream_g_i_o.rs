// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff+)
// from gir-files (https://github.com/gtk-rs/gir-files @ b215ee8)
// DO NOT EDIT

use Stream;
use ffi;
use gio;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct StreamGIO(Object<ffi::GMimeStreamGIO, ffi::GMimeStreamGIOClass>): Stream;

    match fn {
        get_type => || ffi::g_mime_stream_gio_get_type(),
    }
}

impl StreamGIO {
    pub fn new<P: IsA<gio::File>>(file: &P) -> StreamGIO {
        unsafe {
            Stream::from_glib_full(ffi::g_mime_stream_gio_new(file.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_bounds<P: IsA<gio::File>>(file: &P, start: i64, end: i64) -> StreamGIO {
        unsafe {
            Stream::from_glib_full(ffi::g_mime_stream_gio_new_with_bounds(file.to_glib_none().0, start, end)).downcast_unchecked()
        }
    }
}

pub trait StreamGIOExt {
    fn get_owner(&self) -> bool;

    fn set_owner(&self, owner: bool);
}

impl<O: IsA<StreamGIO>> StreamGIOExt for O {
    fn get_owner(&self) -> bool {
        unsafe {
            from_glib(ffi::g_mime_stream_gio_get_owner(self.to_glib_none().0))
        }
    }

    fn set_owner(&self, owner: bool) {
        unsafe {
            ffi::g_mime_stream_gio_set_owner(self.to_glib_none().0, owner.to_glib());
        }
    }
}
