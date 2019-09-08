// This file was generated by gir (https://github.com/gtk-rs/gir @ 9e3cb65)
// from gir-files (https://github.com/gtk-rs/gir-files @ b215ee8+)
// DO NOT EDIT

use Filter;
#[cfg(any(feature = "v3_2", feature = "dox"))]
use OpenPGPData;
#[cfg(any(feature = "v3_2", feature = "dox"))]
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use gmime_sys;
use std::fmt;

glib_wrapper! {
    pub struct FilterOpenPGP(Object<gmime_sys::GMimeFilterOpenPGP, gmime_sys::GMimeFilterOpenPGPClass, FilterOpenPGPClass>) @extends Filter;

    match fn {
        get_type => || gmime_sys::g_mime_filter_openpgp_get_type(),
    }
}

impl FilterOpenPGP {
    #[cfg(any(feature = "v3_2", feature = "dox"))]
    pub fn new() -> FilterOpenPGP {
        unsafe {
            Filter::from_glib_full(gmime_sys::g_mime_filter_openpgp_new()).unsafe_cast()
        }
    }
}

#[cfg(any(feature = "v3_2", feature = "dox"))]
impl Default for FilterOpenPGP {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_FILTER_OPEN_PGP: Option<&FilterOpenPGP> = None;

pub trait FilterOpenPGPExt: 'static {
    #[cfg(any(feature = "v3_2", feature = "dox"))]
    fn get_begin_offset(&self) -> i64;

    #[cfg(any(feature = "v3_2", feature = "dox"))]
    fn get_data_type(&self) -> OpenPGPData;

    #[cfg(any(feature = "v3_2", feature = "dox"))]
    fn get_end_offset(&self) -> i64;
}

impl<O: IsA<FilterOpenPGP>> FilterOpenPGPExt for O {
    #[cfg(any(feature = "v3_2", feature = "dox"))]
    fn get_begin_offset(&self) -> i64 {
        unsafe {
            gmime_sys::g_mime_filter_openpgp_get_begin_offset(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_2", feature = "dox"))]
    fn get_data_type(&self) -> OpenPGPData {
        unsafe {
            from_glib(gmime_sys::g_mime_filter_openpgp_get_data_type(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_2", feature = "dox"))]
    fn get_end_offset(&self) -> i64 {
        unsafe {
            gmime_sys::g_mime_filter_openpgp_get_end_offset(self.as_ref().to_glib_none().0)
        }
    }
}

impl fmt::Display for FilterOpenPGP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FilterOpenPGP")
    }
}
