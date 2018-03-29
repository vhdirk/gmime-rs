// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff+)
// from gir-files (https://github.com/gtk-rs/gir-files @ 33386b3)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct InternetAddress(Object<ffi::InternetAddress, ffi::InternetAddressClass>);

    match fn {
        get_type => || ffi::internet_address_get_type(),
    }
}

pub trait InternetAddressExt {
    fn get_charset(&self) -> Option<String>;

    fn get_name(&self) -> Option<String>;

    fn set_charset<'a, P: Into<Option<&'a str>>>(&self, charset: P);

    fn set_name(&self, name: &str);

    //fn to_string<'a, P: Into<Option<&'a /*Ignored*/FormatOptions>>>(&self, options: P, encode: bool) -> String;
}

impl<O: IsA<InternetAddress>> InternetAddressExt for O {
    fn get_charset(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::internet_address_get_charset(self.to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::internet_address_get_name(self.to_glib_none().0))
        }
    }

    fn set_charset<'a, P: Into<Option<&'a str>>>(&self, charset: P) {
        let charset = charset.into();
        let charset = charset.to_glib_none();
        unsafe {
            ffi::internet_address_set_charset(self.to_glib_none().0, charset.0);
        }
    }

    fn set_name(&self, name: &str) {
        unsafe {
            ffi::internet_address_set_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    //fn to_string<'a, P: Into<Option<&'a /*Ignored*/FormatOptions>>>(&self, options: P, encode: bool) -> String {
    //    unsafe { TODO: call ffi::internet_address_to_string() }
    //}
}
