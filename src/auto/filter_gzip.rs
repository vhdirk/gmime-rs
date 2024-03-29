// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::Filter;
use crate::FilterGZipMode;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GMimeFilterGZip")]
    pub struct FilterGZip(Object<ffi::GMimeFilterGZip, ffi::GMimeFilterGZipClass>) @extends Filter;

    match fn {
        type_ => || ffi::g_mime_filter_gzip_get_type(),
    }
}

impl FilterGZip {
    #[doc(alias = "g_mime_filter_gzip_new")]
    pub fn new(mode: FilterGZipMode, level: i32) -> FilterGZip {
        assert_initialized_main_thread!();
        unsafe {
            Filter::from_glib_full(ffi::g_mime_filter_gzip_new(mode.into_glib(), level))
                .unsafe_cast()
        }
    }
}

pub const NONE_FILTER_GZIP: Option<&FilterGZip> = None;

pub trait FilterGZipExt: 'static {
    #[cfg(any(feature = "v3_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_2")))]
    #[doc(alias = "g_mime_filter_gzip_get_comment")]
    #[doc(alias = "get_comment")]
    fn comment(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v3_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_2")))]
    #[doc(alias = "g_mime_filter_gzip_get_filename")]
    #[doc(alias = "get_filename")]
    fn filename(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v3_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_2")))]
    #[doc(alias = "g_mime_filter_gzip_set_comment")]
    fn set_comment(&self, comment: &str);

    #[cfg(any(feature = "v3_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_2")))]
    #[doc(alias = "g_mime_filter_gzip_set_filename")]
    fn set_filename(&self, filename: &str);
}

impl<O: IsA<FilterGZip>> FilterGZipExt for O {
    #[cfg(any(feature = "v3_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_2")))]
    fn comment(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_mime_filter_gzip_get_comment(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_2")))]
    fn filename(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_mime_filter_gzip_get_filename(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_2")))]
    fn set_comment(&self, comment: &str) {
        unsafe {
            ffi::g_mime_filter_gzip_set_comment(
                self.as_ref().to_glib_none().0,
                comment.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v3_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_2")))]
    fn set_filename(&self, filename: &str) {
        unsafe {
            ffi::g_mime_filter_gzip_set_filename(
                self.as_ref().to_glib_none().0,
                filename.to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for FilterGZip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FilterGZip")
    }
}
