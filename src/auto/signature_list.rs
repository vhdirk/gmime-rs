// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::Signature;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GMimeSignatureList")]
    pub struct SignatureList(Object<ffi::GMimeSignatureList, ffi::GMimeSignatureListClass>);

    match fn {
        type_ => || ffi::g_mime_signature_list_get_type(),
    }
}

impl SignatureList {
    #[doc(alias = "g_mime_signature_list_new")]
    pub fn new() -> SignatureList {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::g_mime_signature_list_new()) }
    }
}

impl Default for SignatureList {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SIGNATURE_LIST: Option<&SignatureList> = None;

pub trait SignatureListExt: 'static {
    #[doc(alias = "g_mime_signature_list_add")]
    fn add(&self, sig: &impl IsA<Signature>) -> i32;

    #[doc(alias = "g_mime_signature_list_clear")]
    fn clear(&self);

    #[doc(alias = "g_mime_signature_list_contains")]
    fn contains(&self, sig: &impl IsA<Signature>) -> bool;

    #[doc(alias = "g_mime_signature_list_get_signature")]
    #[doc(alias = "get_signature")]
    fn signature(&self, index: i32) -> Option<Signature>;

    #[doc(alias = "g_mime_signature_list_index_of")]
    fn index_of(&self, sig: &impl IsA<Signature>) -> i32;

    #[doc(alias = "g_mime_signature_list_insert")]
    fn insert(&self, index: i32, sig: &impl IsA<Signature>);

    #[doc(alias = "g_mime_signature_list_length")]
    fn length(&self) -> i32;

    #[doc(alias = "g_mime_signature_list_remove")]
    fn remove(&self, sig: &impl IsA<Signature>) -> bool;

    #[doc(alias = "g_mime_signature_list_remove_at")]
    fn remove_at(&self, index: i32) -> bool;

    #[doc(alias = "g_mime_signature_list_set_signature")]
    fn set_signature(&self, index: i32, sig: &impl IsA<Signature>);
}

impl<O: IsA<SignatureList>> SignatureListExt for O {
    fn add(&self, sig: &impl IsA<Signature>) -> i32 {
        unsafe {
            ffi::g_mime_signature_list_add(
                self.as_ref().to_glib_none().0,
                sig.as_ref().to_glib_none().0,
            )
        }
    }

    fn clear(&self) {
        unsafe {
            ffi::g_mime_signature_list_clear(self.as_ref().to_glib_none().0);
        }
    }

    fn contains(&self, sig: &impl IsA<Signature>) -> bool {
        unsafe {
            from_glib(ffi::g_mime_signature_list_contains(
                self.as_ref().to_glib_none().0,
                sig.as_ref().to_glib_none().0,
            ))
        }
    }

    fn signature(&self, index: i32) -> Option<Signature> {
        unsafe {
            from_glib_none(ffi::g_mime_signature_list_get_signature(
                self.as_ref().to_glib_none().0,
                index,
            ))
        }
    }

    fn index_of(&self, sig: &impl IsA<Signature>) -> i32 {
        unsafe {
            ffi::g_mime_signature_list_index_of(
                self.as_ref().to_glib_none().0,
                sig.as_ref().to_glib_none().0,
            )
        }
    }

    fn insert(&self, index: i32, sig: &impl IsA<Signature>) {
        unsafe {
            ffi::g_mime_signature_list_insert(
                self.as_ref().to_glib_none().0,
                index,
                sig.as_ref().to_glib_none().0,
            );
        }
    }

    fn length(&self) -> i32 {
        unsafe { ffi::g_mime_signature_list_length(self.as_ref().to_glib_none().0) }
    }

    fn remove(&self, sig: &impl IsA<Signature>) -> bool {
        unsafe {
            from_glib(ffi::g_mime_signature_list_remove(
                self.as_ref().to_glib_none().0,
                sig.as_ref().to_glib_none().0,
            ))
        }
    }

    fn remove_at(&self, index: i32) -> bool {
        unsafe {
            from_glib(ffi::g_mime_signature_list_remove_at(
                self.as_ref().to_glib_none().0,
                index,
            ))
        }
    }

    fn set_signature(&self, index: i32, sig: &impl IsA<Signature>) {
        unsafe {
            ffi::g_mime_signature_list_set_signature(
                self.as_ref().to_glib_none().0,
                index,
                sig.as_ref().to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for SignatureList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SignatureList")
    }
}
