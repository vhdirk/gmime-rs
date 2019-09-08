// This file was generated by gir (https://github.com/gtk-rs/gir @ 9e3cb65)
// from gir-files (https://github.com/gtk-rs/gir-files @ b215ee8+)
// DO NOT EDIT

use Filter;
use glib::object::Cast;
use glib::translate::*;
use gmime_sys;
use std::fmt;

glib_wrapper! {
    pub struct FilterUnix2Dos(Object<gmime_sys::GMimeFilterUnix2Dos, gmime_sys::GMimeFilterUnix2DosClass, FilterUnix2DosClass>) @extends Filter;

    match fn {
        get_type => || gmime_sys::g_mime_filter_unix2dos_get_type(),
    }
}

impl FilterUnix2Dos {
    pub fn new(ensure_newline: bool) -> FilterUnix2Dos {
        unsafe {
            Filter::from_glib_full(gmime_sys::g_mime_filter_unix2dos_new(ensure_newline.to_glib())).unsafe_cast()
        }
    }
}

pub const NONE_FILTER_UNIX2_DOS: Option<&FilterUnix2Dos> = None;

impl fmt::Display for FilterUnix2Dos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FilterUnix2Dos")
    }
}
