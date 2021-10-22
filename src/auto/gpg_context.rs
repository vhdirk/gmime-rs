// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::CryptoContext;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GMimeGpgContext")]
    pub struct GpgContext(Object<ffi::GMimeGpgContext, ffi::GMimeGpgContextClass>) @extends CryptoContext;

    match fn {
        type_ => || ffi::g_mime_gpg_context_get_type(),
    }
}

impl GpgContext {
    #[doc(alias = "g_mime_gpg_context_new")]
    pub fn new() -> GpgContext {
        assert_initialized_main_thread!();
        unsafe { CryptoContext::from_glib_full(ffi::g_mime_gpg_context_new()).unsafe_cast() }
    }
}

impl Default for GpgContext {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for GpgContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GpgContext")
    }
}
