// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff+)
// from gir-files (https://github.com/gtk-rs/gir-files @ b215ee8)
// DO NOT EDIT

use ParamList;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct ContentDisposition(Object<ffi::GMimeContentDisposition, ffi::GMimeContentDispositionClass>);

    match fn {
        get_type => || ffi::g_mime_content_disposition_get_type(),
    }
}

impl ContentDisposition {
    pub fn new() -> ContentDisposition {
        unsafe {
            from_glib_full(ffi::g_mime_content_disposition_new())
        }
    }

    //pub fn parse<'a, P: Into<Option<&'a /*Ignored*/ParserOptions>>>(options: P, str: &str) -> Option<ContentDisposition> {
    //    unsafe { TODO: call ffi::g_mime_content_disposition_parse() }
    //}
}

impl Default for ContentDisposition {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ContentDispositionExt {
    //fn encode<'a, P: Into<Option<&'a /*Ignored*/FormatOptions>>>(&self, options: P) -> Option<String>;

    fn get_disposition(&self) -> Option<String>;

    fn get_parameter(&self, name: &str) -> Option<String>;

    fn get_parameters(&self) -> Option<ParamList>;

    fn is_attachment(&self) -> bool;

    fn set_disposition(&self, value: &str);

    fn set_parameter(&self, name: &str, value: &str);
}

impl<O: IsA<ContentDisposition>> ContentDispositionExt for O {
    //fn encode<'a, P: Into<Option<&'a /*Ignored*/FormatOptions>>>(&self, options: P) -> Option<String> {
    //    unsafe { TODO: call ffi::g_mime_content_disposition_encode() }
    //}

    fn get_disposition(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_mime_content_disposition_get_disposition(self.to_glib_none().0))
        }
    }

    fn get_parameter(&self, name: &str) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_mime_content_disposition_get_parameter(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn get_parameters(&self) -> Option<ParamList> {
        unsafe {
            from_glib_none(ffi::g_mime_content_disposition_get_parameters(self.to_glib_none().0))
        }
    }

    fn is_attachment(&self) -> bool {
        unsafe {
            from_glib(ffi::g_mime_content_disposition_is_attachment(self.to_glib_none().0))
        }
    }

    fn set_disposition(&self, value: &str) {
        unsafe {
            ffi::g_mime_content_disposition_set_disposition(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_parameter(&self, name: &str, value: &str) {
        unsafe {
            ffi::g_mime_content_disposition_set_parameter(self.to_glib_none().0, name.to_glib_none().0, value.to_glib_none().0);
        }
    }
}
