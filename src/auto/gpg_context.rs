// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff+)
// from gir-files (https://github.com/gtk-rs/gir-files @ b215ee8)
// DO NOT EDIT

use CryptoContext;
use ffi;
use glib::object::Downcast;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct GpgContext(Object<ffi::GMimeGpgContext, ffi::GMimeGpgContextClass>): CryptoContext;

    match fn {
        get_type => || ffi::g_mime_gpg_context_get_type(),
    }
}

impl GpgContext {
    pub fn new() -> GpgContext {
        unsafe {
            CryptoContext::from_glib_full(ffi::g_mime_gpg_context_new()).downcast_unchecked()
        }
    }
}

impl Default for GpgContext {
    fn default() -> Self {
        Self::new()
    }
}
