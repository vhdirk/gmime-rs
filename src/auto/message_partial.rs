// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff+)
// from gir-files (https://github.com/gtk-rs/gir-files @ b215ee8)
// DO NOT EDIT

use Object;
use Part;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct MessagePartial(Object<ffi::GMimeMessagePartial, ffi::GMimeMessagePartialClass>): Part, Object;

    match fn {
        get_type => || ffi::g_mime_message_partial_get_type(),
    }
}

impl MessagePartial {
    pub fn new(id: &str, number: i32, total: i32) -> MessagePartial {
        unsafe {
            from_glib_full(ffi::g_mime_message_partial_new(id.to_glib_none().0, number, total))
        }
    }
}

pub trait MessagePartialExt {
    fn get_id(&self) -> Option<String>;

    fn get_number(&self) -> i32;

    fn get_total(&self) -> i32;
}

impl<O: IsA<MessagePartial>> MessagePartialExt for O {
    fn get_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_mime_message_partial_get_id(self.to_glib_none().0))
        }
    }

    fn get_number(&self) -> i32 {
        unsafe {
            ffi::g_mime_message_partial_get_number(self.to_glib_none().0)
        }
    }

    fn get_total(&self) -> i32 {
        unsafe {
            ffi::g_mime_message_partial_get_total(self.to_glib_none().0)
        }
    }
}
