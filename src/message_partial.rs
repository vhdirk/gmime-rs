use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use std::fmt;

use crate::Message;
use crate::MessagePartial;

pub trait MessagePartialExtManual: 'static {
    #[doc(alias = "g_mime_message_partial_reconstruct_message")]
    fn reconstruct_message(partials: &Vec<MessagePartial>) -> Option<Message>;
}

impl<O: IsA<MessagePartial>> MessagePartialExtManual for O {
    fn reconstruct_message(partials: &Vec<MessagePartial>) -> Option<Message> {
        unsafe {
            from_glib_full(ffi::g_mime_message_partial_reconstruct_message(
                partials.to_glib_none().0,
                partials.len(),
            ))
        }
    }
}
