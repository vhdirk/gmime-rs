// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::ContentEncoding;
use crate::Filter;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GMimeFilterBasic")]
    pub struct FilterBasic(Object<ffi::GMimeFilterBasic, ffi::GMimeFilterBasicClass>) @extends Filter;

    match fn {
        type_ => || ffi::g_mime_filter_basic_get_type(),
    }
}

impl FilterBasic {
    #[doc(alias = "g_mime_filter_basic_new")]
    pub fn new(encoding: ContentEncoding, encode: bool) -> FilterBasic {
        assert_initialized_main_thread!();
        unsafe {
            Filter::from_glib_full(ffi::g_mime_filter_basic_new(
                encoding.into_glib(),
                encode.into_glib(),
            ))
            .unsafe_cast()
        }
    }
}

pub const NONE_FILTER_BASIC: Option<&FilterBasic> = None;

impl fmt::Display for FilterBasic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FilterBasic")
    }
}
