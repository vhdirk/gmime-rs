// This file was generated by gir (https://github.com/gtk-rs/gir @ 7f5a2b5)
// from gir-files (https://github.com/gtk-rs/gir-files @ 4740f5e+)
// DO NOT EDIT

use ParserOptions;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct HeaderList(Object<ffi::GMimeHeaderList, ffi::GMimeHeaderListClass>);

    match fn {
        get_type => || ffi::g_mime_header_list_get_type(),
    }
}

impl HeaderList {
    pub fn new<'a, P: Into<Option<&'a ParserOptions>>>(options: P) -> HeaderList {
        let options = options.into();
        unsafe {
            from_glib_full(ffi::g_mime_header_list_new(options.to_glib_none_mut().0))
        }
    }
}

pub trait HeaderListExt {
    fn append(&self, name: &str, value: &str, charset: &str);

    fn clear(&self);

    fn contains(&self, name: &str) -> bool;

    fn get_count(&self) -> i32;

    //fn get_header(&self, name: &str) -> /*Ignored*/Option<Header>;

    //fn get_header_at(&self, index: i32) -> /*Ignored*/Option<Header>;

    fn prepend(&self, name: &str, value: &str, charset: &str);

    fn remove(&self, name: &str) -> bool;

    fn remove_at(&self, index: i32);

    fn set(&self, name: &str, value: &str, charset: &str);

    //fn to_string<'a, P: Into<Option<&'a /*Ignored*/FormatOptions>>>(&self, options: P) -> String;

    //fn write_to_stream<'a, P: Into<Option<&'a /*Ignored*/FormatOptions>>, Q: IsA<Stream>>(&self, options: P, stream: &Q) -> isize;
}

impl<O: IsA<HeaderList>> HeaderListExt for O {
    fn append(&self, name: &str, value: &str, charset: &str) {
        unsafe {
            ffi::g_mime_header_list_append(self.to_glib_none().0, name.to_glib_none().0, value.to_glib_none().0, charset.to_glib_none().0);
        }
    }

    fn clear(&self) {
        unsafe {
            ffi::g_mime_header_list_clear(self.to_glib_none().0);
        }
    }

    fn contains(&self, name: &str) -> bool {
        unsafe {
            from_glib(ffi::g_mime_header_list_contains(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn get_count(&self) -> i32 {
        unsafe {
            ffi::g_mime_header_list_get_count(self.to_glib_none().0)
        }
    }

    //fn get_header(&self, name: &str) -> /*Ignored*/Option<Header> {
    //    unsafe { TODO: call ffi::g_mime_header_list_get_header() }
    //}

    //fn get_header_at(&self, index: i32) -> /*Ignored*/Option<Header> {
    //    unsafe { TODO: call ffi::g_mime_header_list_get_header_at() }
    //}

    fn prepend(&self, name: &str, value: &str, charset: &str) {
        unsafe {
            ffi::g_mime_header_list_prepend(self.to_glib_none().0, name.to_glib_none().0, value.to_glib_none().0, charset.to_glib_none().0);
        }
    }

    fn remove(&self, name: &str) -> bool {
        unsafe {
            from_glib(ffi::g_mime_header_list_remove(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn remove_at(&self, index: i32) {
        unsafe {
            ffi::g_mime_header_list_remove_at(self.to_glib_none().0, index);
        }
    }

    fn set(&self, name: &str, value: &str, charset: &str) {
        unsafe {
            ffi::g_mime_header_list_set(self.to_glib_none().0, name.to_glib_none().0, value.to_glib_none().0, charset.to_glib_none().0);
        }
    }

    //fn to_string<'a, P: Into<Option<&'a /*Ignored*/FormatOptions>>>(&self, options: P) -> String {
    //    unsafe { TODO: call ffi::g_mime_header_list_to_string() }
    //}

    //fn write_to_stream<'a, P: Into<Option<&'a /*Ignored*/FormatOptions>>, Q: IsA<Stream>>(&self, options: P, stream: &Q) -> isize {
    //    unsafe { TODO: call ffi::g_mime_header_list_write_to_stream() }
    //}
}
