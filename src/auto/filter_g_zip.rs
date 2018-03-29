// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff+)
// from gir-files (https://github.com/gtk-rs/gir-files @ 33386b3)
// DO NOT EDIT

use Filter;
use FilterGZipMode;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct FilterGZip(Object<ffi::GMimeFilterGZip, ffi::GMimeFilterGZipClass>): Filter;

    match fn {
        get_type => || ffi::g_mime_filter_gzip_get_type(),
    }
}

impl FilterGZip {
    pub fn new(mode: FilterGZipMode, level: i32) -> FilterGZip {
        unsafe {
            Filter::from_glib_full(ffi::g_mime_filter_gzip_new(mode.to_glib(), level)).downcast_unchecked()
        }
    }
}

pub trait FilterGZipExt {
    #[cfg(any(feature = "v3_2", feature = "dox"))]
    fn get_comment(&self) -> Option<String>;

    #[cfg(any(feature = "v3_2", feature = "dox"))]
    fn get_filename(&self) -> Option<String>;

    #[cfg(any(feature = "v3_2", feature = "dox"))]
    fn set_comment(&self, comment: &str);

    #[cfg(any(feature = "v3_2", feature = "dox"))]
    fn set_filename(&self, filename: &str);
}

impl<O: IsA<FilterGZip>> FilterGZipExt for O {
    #[cfg(any(feature = "v3_2", feature = "dox"))]
    fn get_comment(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_mime_filter_gzip_get_comment(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_2", feature = "dox"))]
    fn get_filename(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_mime_filter_gzip_get_filename(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_2", feature = "dox"))]
    fn set_comment(&self, comment: &str) {
        unsafe {
            ffi::g_mime_filter_gzip_set_comment(self.to_glib_none().0, comment.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_2", feature = "dox"))]
    fn set_filename(&self, filename: &str) {
        unsafe {
            ffi::g_mime_filter_gzip_set_filename(self.to_glib_none().0, filename.to_glib_none().0);
        }
    }
}
