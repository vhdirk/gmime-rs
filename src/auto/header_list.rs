// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::FormatOptions;
use crate::Header;
use crate::ParserOptions;
use crate::Stream;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GMimeHeaderList")]
    pub struct HeaderList(Object<ffi::GMimeHeaderList, ffi::GMimeHeaderListClass>);

    match fn {
        type_ => || ffi::g_mime_header_list_get_type(),
    }
}

impl HeaderList {
    #[doc(alias = "g_mime_header_list_new")]
    pub fn new(options: Option<&ParserOptions>) -> HeaderList {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::g_mime_header_list_new(mut_override(
                options.to_glib_none().0,
            )))
        }
    }
}

pub const NONE_HEADER_LIST: Option<&HeaderList> = None;

pub trait HeaderListExt: 'static {
    #[doc(alias = "g_mime_header_list_append")]
    fn append(&self, name: &str, value: &str, charset: &str);

    #[doc(alias = "g_mime_header_list_clear")]
    fn clear(&self);

    #[doc(alias = "g_mime_header_list_contains")]
    fn contains(&self, name: &str) -> bool;

    #[doc(alias = "g_mime_header_list_get_count")]
    #[doc(alias = "get_count")]
    fn count(&self) -> i32;

    #[doc(alias = "g_mime_header_list_get_header")]
    #[doc(alias = "get_header")]
    fn header(&self, name: &str) -> Option<Header>;

    #[doc(alias = "g_mime_header_list_get_header_at")]
    #[doc(alias = "get_header_at")]
    fn header_at(&self, index: i32) -> Option<Header>;

    #[doc(alias = "g_mime_header_list_prepend")]
    fn prepend(&self, name: &str, value: &str, charset: &str);

    #[doc(alias = "g_mime_header_list_remove")]
    fn remove(&self, name: &str) -> bool;

    #[doc(alias = "g_mime_header_list_remove_at")]
    fn remove_at(&self, index: i32);

    #[doc(alias = "g_mime_header_list_set")]
    fn set(&self, name: &str, value: &str, charset: &str);

    #[doc(alias = "g_mime_header_list_to_string")]
    fn to_string(&self, options: Option<&FormatOptions>) -> Option<glib::GString>;

    #[doc(alias = "g_mime_header_list_write_to_stream")]
    fn write_to_stream(&self, options: Option<&FormatOptions>, stream: &impl IsA<Stream>) -> isize;
}

impl<O: IsA<HeaderList>> HeaderListExt for O {
    fn append(&self, name: &str, value: &str, charset: &str) {
        unsafe {
            ffi::g_mime_header_list_append(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                value.to_glib_none().0,
                charset.to_glib_none().0,
            );
        }
    }

    fn clear(&self) {
        unsafe {
            ffi::g_mime_header_list_clear(self.as_ref().to_glib_none().0);
        }
    }

    fn contains(&self, name: &str) -> bool {
        unsafe {
            from_glib(ffi::g_mime_header_list_contains(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    fn count(&self) -> i32 {
        unsafe { ffi::g_mime_header_list_get_count(self.as_ref().to_glib_none().0) }
    }

    fn header(&self, name: &str) -> Option<Header> {
        unsafe {
            from_glib_none(ffi::g_mime_header_list_get_header(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    fn header_at(&self, index: i32) -> Option<Header> {
        unsafe {
            from_glib_none(ffi::g_mime_header_list_get_header_at(
                self.as_ref().to_glib_none().0,
                index,
            ))
        }
    }

    fn prepend(&self, name: &str, value: &str, charset: &str) {
        unsafe {
            ffi::g_mime_header_list_prepend(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                value.to_glib_none().0,
                charset.to_glib_none().0,
            );
        }
    }

    fn remove(&self, name: &str) -> bool {
        unsafe {
            from_glib(ffi::g_mime_header_list_remove(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    fn remove_at(&self, index: i32) {
        unsafe {
            ffi::g_mime_header_list_remove_at(self.as_ref().to_glib_none().0, index);
        }
    }

    fn set(&self, name: &str, value: &str, charset: &str) {
        unsafe {
            ffi::g_mime_header_list_set(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                value.to_glib_none().0,
                charset.to_glib_none().0,
            );
        }
    }

    fn to_string(&self, options: Option<&FormatOptions>) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::g_mime_header_list_to_string(
                self.as_ref().to_glib_none().0,
                mut_override(options.to_glib_none().0),
            ))
        }
    }

    fn write_to_stream(&self, options: Option<&FormatOptions>, stream: &impl IsA<Stream>) -> isize {
        unsafe {
            ffi::g_mime_header_list_write_to_stream(
                self.as_ref().to_glib_none().0,
                mut_override(options.to_glib_none().0),
                stream.as_ref().to_glib_none().0,
            )
        }
    }
}

impl fmt::Display for HeaderList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("HeaderList")
    }
}
