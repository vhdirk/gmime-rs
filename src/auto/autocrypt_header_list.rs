// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::AutocryptHeader;
use crate::InternetAddressList;
use crate::InternetAddressMailbox;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GMimeAutocryptHeaderList")]
    pub struct AutocryptHeaderList(Object<ffi::GMimeAutocryptHeaderList, ffi::GMimeAutocryptHeaderListClass>);

    match fn {
        type_ => || ffi::g_mime_autocrypt_header_list_get_type(),
    }
}

impl AutocryptHeaderList {
    #[doc(alias = "g_mime_autocrypt_header_list_new")]
    pub fn new() -> AutocryptHeaderList {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::g_mime_autocrypt_header_list_new()) }
    }
}

impl Default for AutocryptHeaderList {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_AUTOCRYPT_HEADER_LIST: Option<&AutocryptHeaderList> = None;

pub trait AutocryptHeaderListExt: 'static {
    #[doc(alias = "g_mime_autocrypt_header_list_add")]
    fn add(&self, header: &impl IsA<AutocryptHeader>);

    #[doc(alias = "g_mime_autocrypt_header_list_add_missing_addresses")]
    fn add_missing_addresses(&self, addresses: &impl IsA<InternetAddressList>) -> u32;

    #[doc(alias = "g_mime_autocrypt_header_list_get_count")]
    #[doc(alias = "get_count")]
    fn count(&self) -> u32;

    #[doc(alias = "g_mime_autocrypt_header_list_get_header_at")]
    #[doc(alias = "get_header_at")]
    fn header_at(&self, index: u32) -> Option<AutocryptHeader>;

    #[doc(alias = "g_mime_autocrypt_header_list_get_header_for_address")]
    #[doc(alias = "get_header_for_address")]
    fn header_for_address(
        &self,
        mailbox: &impl IsA<InternetAddressMailbox>,
    ) -> Option<AutocryptHeader>;

    #[doc(alias = "g_mime_autocrypt_header_list_remove_incomplete")]
    fn remove_incomplete(&self);
}

impl<O: IsA<AutocryptHeaderList>> AutocryptHeaderListExt for O {
    fn add(&self, header: &impl IsA<AutocryptHeader>) {
        unsafe {
            ffi::g_mime_autocrypt_header_list_add(
                self.as_ref().to_glib_none().0,
                header.as_ref().to_glib_none().0,
            );
        }
    }

    fn add_missing_addresses(&self, addresses: &impl IsA<InternetAddressList>) -> u32 {
        unsafe {
            ffi::g_mime_autocrypt_header_list_add_missing_addresses(
                self.as_ref().to_glib_none().0,
                addresses.as_ref().to_glib_none().0,
            )
        }
    }

    fn count(&self) -> u32 {
        unsafe { ffi::g_mime_autocrypt_header_list_get_count(self.as_ref().to_glib_none().0) }
    }

    fn header_at(&self, index: u32) -> Option<AutocryptHeader> {
        unsafe {
            from_glib_none(ffi::g_mime_autocrypt_header_list_get_header_at(
                self.as_ref().to_glib_none().0,
                index,
            ))
        }
    }

    fn header_for_address(
        &self,
        mailbox: &impl IsA<InternetAddressMailbox>,
    ) -> Option<AutocryptHeader> {
        unsafe {
            from_glib_none(ffi::g_mime_autocrypt_header_list_get_header_for_address(
                self.as_ref().to_glib_none().0,
                mailbox.as_ref().to_glib_none().0,
            ))
        }
    }

    fn remove_incomplete(&self) {
        unsafe {
            ffi::g_mime_autocrypt_header_list_remove_incomplete(self.as_ref().to_glib_none().0);
        }
    }
}

impl fmt::Display for AutocryptHeaderList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AutocryptHeaderList")
    }
}
