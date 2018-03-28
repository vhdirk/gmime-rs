// This file was generated by gir (https://github.com/gtk-rs/gir @ 7f5a2b5)
// from gir-files (https://github.com/gtk-rs/gir-files @ 4740f5e+)
// DO NOT EDIT

use Message;
use Object;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct MessagePart(Object<ffi::GMimeMessagePart, ffi::GMimeMessagePartClass>): Object;

    match fn {
        get_type => || ffi::g_mime_message_part_get_type(),
    }
}

impl MessagePart {
    pub fn new(subtype: &str) -> MessagePart {
        unsafe {
            from_glib_full(ffi::g_mime_message_part_new(subtype.to_glib_none().0))
        }
    }

    pub fn new_with_message(subtype: &str, message: &Message) -> MessagePart {
        unsafe {
            from_glib_full(ffi::g_mime_message_part_new_with_message(subtype.to_glib_none().0, message.to_glib_none().0))
        }
    }
}

pub trait MessagePartExt {
    fn get_message(&self) -> Option<Message>;

    fn set_message(&self, message: &Message);
}

impl<O: IsA<MessagePart>> MessagePartExt for O {
    fn get_message(&self) -> Option<Message> {
        unsafe {
            from_glib_none(ffi::g_mime_message_part_get_message(self.to_glib_none().0))
        }
    }

    fn set_message(&self, message: &Message) {
        unsafe {
            ffi::g_mime_message_part_set_message(self.to_glib_none().0, message.to_glib_none().0);
        }
    }
}
