// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff+)
// from gir-files (https://github.com/gtk-rs/gir-files @ 33386b3)
// DO NOT EDIT

use InternetAddress;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct InternetAddressMailbox(Object<ffi::InternetAddressMailbox, ffi::InternetAddressMailboxClass>): InternetAddress;

    match fn {
        get_type => || ffi::internet_address_mailbox_get_type(),
    }
}

impl InternetAddressMailbox {
    pub fn new(name: &str, addr: &str) -> InternetAddressMailbox {
        unsafe {
            InternetAddress::from_glib_full(ffi::internet_address_mailbox_new(name.to_glib_none().0, addr.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait InternetAddressMailboxExt {
    fn get_addr(&self) -> Option<String>;

    fn get_idn_addr(&self) -> Option<String>;

    fn set_addr(&self, addr: &str);
}

impl<O: IsA<InternetAddressMailbox>> InternetAddressMailboxExt for O {
    fn get_addr(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::internet_address_mailbox_get_addr(self.to_glib_none().0))
        }
    }

    fn get_idn_addr(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::internet_address_mailbox_get_idn_addr(self.to_glib_none().0))
        }
    }

    fn set_addr(&self, addr: &str) {
        unsafe {
            ffi::internet_address_mailbox_set_addr(self.to_glib_none().0, addr.to_glib_none().0);
        }
    }
}
