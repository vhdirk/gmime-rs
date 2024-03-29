// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::Filter;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GMimeFilterEnriched")]
    pub struct FilterEnriched(Object<ffi::GMimeFilterEnriched, ffi::GMimeFilterEnrichedClass>) @extends Filter;

    match fn {
        type_ => || ffi::g_mime_filter_enriched_get_type(),
    }
}

impl FilterEnriched {
    #[doc(alias = "g_mime_filter_enriched_new")]
    pub fn new(flags: u32) -> FilterEnriched {
        assert_initialized_main_thread!();
        unsafe { Filter::from_glib_full(ffi::g_mime_filter_enriched_new(flags)).unsafe_cast() }
    }
}

pub const NONE_FILTER_ENRICHED: Option<&FilterEnriched> = None;

impl fmt::Display for FilterEnriched {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FilterEnriched")
    }
}
