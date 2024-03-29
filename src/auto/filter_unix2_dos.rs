// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::Filter;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GMimeFilterUnix2Dos")]
    pub struct FilterUnix2Dos(Object<ffi::GMimeFilterUnix2Dos, ffi::GMimeFilterUnix2DosClass>) @extends Filter;

    match fn {
        type_ => || ffi::g_mime_filter_unix2dos_get_type(),
    }
}

impl FilterUnix2Dos {
    #[doc(alias = "g_mime_filter_unix2dos_new")]
    pub fn new(ensure_newline: bool) -> FilterUnix2Dos {
        assert_initialized_main_thread!();
        unsafe {
            Filter::from_glib_full(ffi::g_mime_filter_unix2dos_new(ensure_newline.into_glib()))
                .unsafe_cast()
        }
    }
}

pub const NONE_FILTER_UNIX2_DOS: Option<&FilterUnix2Dos> = None;

impl fmt::Display for FilterUnix2Dos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FilterUnix2Dos")
    }
}
