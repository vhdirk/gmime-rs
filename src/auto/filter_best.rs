// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::ContentEncoding;
use crate::EncodingConstraint;
use crate::Filter;
use crate::FilterBestFlags;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GMimeFilterBest")]
    pub struct FilterBest(Object<ffi::GMimeFilterBest, ffi::GMimeFilterBestClass>) @extends Filter;

    match fn {
        type_ => || ffi::g_mime_filter_best_get_type(),
    }
}

impl FilterBest {
    #[doc(alias = "g_mime_filter_best_new")]
    pub fn new(flags: FilterBestFlags) -> FilterBest {
        assert_initialized_main_thread!();
        unsafe {
            Filter::from_glib_full(ffi::g_mime_filter_best_new(flags.into_glib())).unsafe_cast()
        }
    }
}

pub const NONE_FILTER_BEST: Option<&FilterBest> = None;

pub trait FilterBestExt: 'static {
    #[doc(alias = "g_mime_filter_best_charset")]
    fn charset(&self) -> Option<glib::GString>;

    #[doc(alias = "g_mime_filter_best_encoding")]
    fn encoding(&self, constraint: EncodingConstraint) -> ContentEncoding;
}

impl<O: IsA<FilterBest>> FilterBestExt for O {
    fn charset(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_mime_filter_best_charset(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn encoding(&self, constraint: EncodingConstraint) -> ContentEncoding {
        unsafe {
            from_glib(ffi::g_mime_filter_best_encoding(
                self.as_ref().to_glib_none().0,
                constraint.into_glib(),
            ))
        }
    }
}

impl fmt::Display for FilterBest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FilterBest")
    }
}
