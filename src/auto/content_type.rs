// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff+)
// from gir-files (https://github.com/gtk-rs/gir-files @ 33386b3)
// DO NOT EDIT

use ParamList;
use ParserOptions;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct ContentType(Object<ffi::GMimeContentType, ffi::GMimeContentTypeClass>);

    match fn {
        get_type => || ffi::g_mime_content_type_get_type(),
    }
}

impl ContentType {
    pub fn new(type_: &str, subtype: &str) -> ContentType {
        unsafe {
            from_glib_full(ffi::g_mime_content_type_new(type_.to_glib_none().0, subtype.to_glib_none().0))
        }
    }

    pub fn parse<'a, P: Into<Option<&'a ParserOptions>>>(options: P, str: &str) -> Option<ContentType> {
        let options = options.into();
        unsafe {
            from_glib_full(ffi::g_mime_content_type_parse(options.to_glib_none_mut().0, str.to_glib_none().0))
        }
    }
}

pub trait ContentTypeExt {
    //fn encode<'a, P: Into<Option<&'a /*Ignored*/FormatOptions>>>(&self, options: P) -> Option<String>;

    fn get_media_subtype(&self) -> Option<String>;

    fn get_media_type(&self) -> Option<String>;

    fn get_mime_type(&self) -> Option<String>;

    fn get_parameter(&self, name: &str) -> Option<String>;

    fn get_parameters(&self) -> Option<ParamList>;

    fn is_type(&self, type_: &str, subtype: &str) -> bool;

    fn set_media_subtype(&self, subtype: &str);

    fn set_media_type(&self, type_: &str);

    fn set_parameter(&self, name: &str, value: &str);
}

impl<O: IsA<ContentType>> ContentTypeExt for O {
    //fn encode<'a, P: Into<Option<&'a /*Ignored*/FormatOptions>>>(&self, options: P) -> Option<String> {
    //    unsafe { TODO: call ffi::g_mime_content_type_encode() }
    //}

    fn get_media_subtype(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_mime_content_type_get_media_subtype(self.to_glib_none().0))
        }
    }

    fn get_media_type(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_mime_content_type_get_media_type(self.to_glib_none().0))
        }
    }

    fn get_mime_type(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::g_mime_content_type_get_mime_type(self.to_glib_none().0))
        }
    }

    fn get_parameter(&self, name: &str) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_mime_content_type_get_parameter(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn get_parameters(&self) -> Option<ParamList> {
        unsafe {
            from_glib_none(ffi::g_mime_content_type_get_parameters(self.to_glib_none().0))
        }
    }

    fn is_type(&self, type_: &str, subtype: &str) -> bool {
        unsafe {
            from_glib(ffi::g_mime_content_type_is_type(self.to_glib_none().0, type_.to_glib_none().0, subtype.to_glib_none().0))
        }
    }

    fn set_media_subtype(&self, subtype: &str) {
        unsafe {
            ffi::g_mime_content_type_set_media_subtype(self.to_glib_none().0, subtype.to_glib_none().0);
        }
    }

    fn set_media_type(&self, type_: &str) {
        unsafe {
            ffi::g_mime_content_type_set_media_type(self.to_glib_none().0, type_.to_glib_none().0);
        }
    }

    fn set_parameter(&self, name: &str, value: &str) {
        unsafe {
            ffi::g_mime_content_type_set_parameter(self.to_glib_none().0, name.to_glib_none().0, value.to_glib_none().0);
        }
    }
}
