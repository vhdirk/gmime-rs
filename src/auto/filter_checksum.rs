// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff+)
// from gir-files (https://github.com/gtk-rs/gir-files @ 33386b3)
// DO NOT EDIT

use Filter;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct FilterChecksum(Object<ffi::GMimeFilterChecksum, ffi::GMimeFilterChecksumClass>): Filter;

    match fn {
        get_type => || ffi::g_mime_filter_checksum_get_type(),
    }
}

impl FilterChecksum {
    pub fn new(type_: glib::ChecksumType) -> FilterChecksum {
        unsafe {
            Filter::from_glib_full(ffi::g_mime_filter_checksum_new(type_.to_glib())).downcast_unchecked()
        }
    }
}

pub trait FilterChecksumExt {
    fn get_digest(&self, digest: u8, len: usize) -> usize;

    fn get_string(&self) -> Option<String>;
}

impl<O: IsA<FilterChecksum>> FilterChecksumExt for O {
    fn get_digest(&self, digest: u8, len: usize) -> usize {
        unsafe {
            ffi::g_mime_filter_checksum_get_digest(self.to_glib_none().0, digest, len)
        }
    }

    fn get_string(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::g_mime_filter_checksum_get_string(self.to_glib_none().0))
        }
    }
}
