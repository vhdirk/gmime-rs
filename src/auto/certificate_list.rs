// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::Certificate;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GMimeCertificateList")]
    pub struct CertificateList(Object<ffi::GMimeCertificateList, ffi::GMimeCertificateListClass>);

    match fn {
        type_ => || ffi::g_mime_certificate_list_get_type(),
    }
}

impl CertificateList {
    #[doc(alias = "g_mime_certificate_list_new")]
    pub fn new() -> CertificateList {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::g_mime_certificate_list_new()) }
    }
}

impl Default for CertificateList {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_CERTIFICATE_LIST: Option<&CertificateList> = None;

pub trait CertificateListExt: 'static {
    #[doc(alias = "g_mime_certificate_list_add")]
    fn add(&self, cert: &impl IsA<Certificate>) -> i32;

    #[doc(alias = "g_mime_certificate_list_clear")]
    fn clear(&self);

    #[doc(alias = "g_mime_certificate_list_contains")]
    fn contains(&self, cert: &impl IsA<Certificate>) -> bool;

    #[doc(alias = "g_mime_certificate_list_get_certificate")]
    #[doc(alias = "get_certificate")]
    fn certificate(&self, index: i32) -> Option<Certificate>;

    #[doc(alias = "g_mime_certificate_list_index_of")]
    fn index_of(&self, cert: &impl IsA<Certificate>) -> i32;

    #[doc(alias = "g_mime_certificate_list_insert")]
    fn insert(&self, index: i32, cert: &impl IsA<Certificate>);

    #[doc(alias = "g_mime_certificate_list_length")]
    fn length(&self) -> i32;

    #[doc(alias = "g_mime_certificate_list_remove")]
    fn remove(&self, cert: &impl IsA<Certificate>) -> bool;

    #[doc(alias = "g_mime_certificate_list_remove_at")]
    fn remove_at(&self, index: i32) -> bool;

    #[doc(alias = "g_mime_certificate_list_set_certificate")]
    fn set_certificate(&self, index: i32, cert: &impl IsA<Certificate>);
}

impl<O: IsA<CertificateList>> CertificateListExt for O {
    fn add(&self, cert: &impl IsA<Certificate>) -> i32 {
        unsafe {
            ffi::g_mime_certificate_list_add(
                self.as_ref().to_glib_none().0,
                cert.as_ref().to_glib_none().0,
            )
        }
    }

    fn clear(&self) {
        unsafe {
            ffi::g_mime_certificate_list_clear(self.as_ref().to_glib_none().0);
        }
    }

    fn contains(&self, cert: &impl IsA<Certificate>) -> bool {
        unsafe {
            from_glib(ffi::g_mime_certificate_list_contains(
                self.as_ref().to_glib_none().0,
                cert.as_ref().to_glib_none().0,
            ))
        }
    }

    fn certificate(&self, index: i32) -> Option<Certificate> {
        unsafe {
            from_glib_full(ffi::g_mime_certificate_list_get_certificate(
                self.as_ref().to_glib_none().0,
                index,
            ))
        }
    }

    fn index_of(&self, cert: &impl IsA<Certificate>) -> i32 {
        unsafe {
            ffi::g_mime_certificate_list_index_of(
                self.as_ref().to_glib_none().0,
                cert.as_ref().to_glib_none().0,
            )
        }
    }

    fn insert(&self, index: i32, cert: &impl IsA<Certificate>) {
        unsafe {
            ffi::g_mime_certificate_list_insert(
                self.as_ref().to_glib_none().0,
                index,
                cert.as_ref().to_glib_none().0,
            );
        }
    }

    fn length(&self) -> i32 {
        unsafe { ffi::g_mime_certificate_list_length(self.as_ref().to_glib_none().0) }
    }

    fn remove(&self, cert: &impl IsA<Certificate>) -> bool {
        unsafe {
            from_glib(ffi::g_mime_certificate_list_remove(
                self.as_ref().to_glib_none().0,
                cert.as_ref().to_glib_none().0,
            ))
        }
    }

    fn remove_at(&self, index: i32) -> bool {
        unsafe {
            from_glib(ffi::g_mime_certificate_list_remove_at(
                self.as_ref().to_glib_none().0,
                index,
            ))
        }
    }

    fn set_certificate(&self, index: i32, cert: &impl IsA<Certificate>) {
        unsafe {
            ffi::g_mime_certificate_list_set_certificate(
                self.as_ref().to_glib_none().0,
                index,
                cert.as_ref().to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for CertificateList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CertificateList")
    }
}
