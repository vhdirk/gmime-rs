// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff+)
// from gir-files (https://github.com/gtk-rs/gir-files @ 33386b3)
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
    pub struct StreamMmap(Object<ffi::GMimeStreamMmap, ffi::GMimeStreamMmapClass>): Stream;

    match fn {
        get_type => || ffi::g_mime_stream_mmap_get_type(),
    }
}

impl StreamMmap {
    pub fn new(fd: i32, prot: i32, flags: i32) -> StreamMmap {
        unsafe {
            Stream::from_glib_full(ffi::g_mime_stream_mmap_new(fd, prot, flags)).downcast_unchecked()
        }
    }

    pub fn new_with_bounds(fd: i32, prot: i32, flags: i32, start: i64, end: i64) -> StreamMmap {
        unsafe {
            Stream::from_glib_full(ffi::g_mime_stream_mmap_new_with_bounds(fd, prot, flags, start, end)).downcast_unchecked()
        }
    }
}

pub trait StreamMmapExt {
    #[cfg(any(feature = "v3_2", feature = "dox"))]
    fn get_owner(&self) -> bool;

    #[cfg(any(feature = "v3_2", feature = "dox"))]
    fn set_owner(&self, owner: bool);
}

impl<O: IsA<StreamMmap>> StreamMmapExt for O {
    #[cfg(any(feature = "v3_2", feature = "dox"))]
    fn get_owner(&self) -> bool {
        unsafe {
            from_glib(ffi::g_mime_stream_mmap_get_owner(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_2", feature = "dox"))]
    fn set_owner(&self, owner: bool) {
        unsafe {
            ffi::g_mime_stream_mmap_set_owner(self.to_glib_none().0, owner.to_glib());
        }
    }
}
