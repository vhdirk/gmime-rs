// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff+)
// from gir-files (https://github.com/gtk-rs/gir-files @ 33386b3)
// DO NOT EDIT

use Stream;
use ffi;
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
    //pub fn new<P: IsA</*Ignored*/gio::File>>(file: &P) -> StreamGIO {
    //    unsafe { TODO: call ffi::g_mime_stream_gio_new() }
    //}

    //pub fn new_with_bounds<P: IsA</*Ignored*/gio::File>>(file: &P, start: i64, end: i64) -> StreamGIO {
    //    unsafe { TODO: call ffi::g_mime_stream_gio_new_with_bounds() }
    //}
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