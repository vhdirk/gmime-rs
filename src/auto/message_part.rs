// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff+)
// from gir-files (https://github.com/gtk-rs/gir-files @ b215ee8)
// DO NOT EDIT

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

    //pub fn new_with_message(subtype: &str, message: /*Ignored*/&Message) -> MessagePart {
    //    unsafe { TODO: call ffi::g_mime_message_part_new_with_message() }
    //}
}

pub trait MessagePartExt {
    //fn get_message(&self) -> /*Ignored*/Option<Message>;

    //fn set_message(&self, message: /*Ignored*/&Message);
}

impl<O: IsA<MessagePart>> MessagePartExt for O {
    //fn get_message(&self) -> /*Ignored*/Option<Message> {
    //    unsafe { TODO: call ffi::g_mime_message_part_get_message() }
    //}

    //fn set_message(&self, message: /*Ignored*/&Message) {
    //    unsafe { TODO: call ffi::g_mime_message_part_set_message() }
    //}
}
