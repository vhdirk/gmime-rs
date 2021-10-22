use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use std::fmt;
use std::ptr;

use crate::Message;

pub trait MessageExtManual: 'static {
    #[doc(alias = "g_mime_message_partial_split_message")]
    fn split(&self, max_size: usize) -> Vec<Message>;
}

impl<O: IsA<Message>> MessageExtManual for O {
    fn split(&self, max_size: usize) -> Vec<Message> {
        unsafe {
            let mut n_parts = ::std::mem::uninitialized();
            let parts = ffi::g_mime_message_partial_split_message(
                self.as_ref().to_glib_none().0,
                max_size,
                &mut n_parts,
            );
            FromGlibContainer::from_glib_full_num(parts, n_parts as usize)
        }
    }
}
