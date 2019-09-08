use Message;
use Object;
use Part;
use gmime_sys;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct MessagePartial(Object<gmime_sys::GMimeMessagePartial, gmime_sys::GMimeMessagePartialClass, MessagePartialClass>) @extends Part, Object;

    match fn {
        get_type => || gmime_sys::g_mime_message_partial_get_type(),
    }
}

impl MessagePartial {
    pub fn new(id: &str, number: i32, total: i32) -> MessagePartial {
        unsafe {
            from_glib_full(gmime_sys::g_mime_message_partial_new(id.to_glib_none().0, number, total))
        }
    }

    pub fn reconstruct_message(partials: &Vec<MessagePartial>) -> Option<Message> {
        unsafe {
            from_glib_full(gmime_sys::g_mime_message_partial_reconstruct_message(partials.to_glib_none().0, partials.len()))
        }
    }
}

pub const NONE_MESSAGE_PARTIAL: Option<&MessagePartial> = None;

pub trait MessagePartialExt: 'static {
    fn get_id(&self) -> Option<GString>;

    fn get_number(&self) -> i32;

    fn get_total(&self) -> i32;
}

impl<O: IsA<MessagePartial>> MessagePartialExt for O {
    fn get_id(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gmime_sys::g_mime_message_partial_get_id(self.as_ref().to_glib_none().0))
        }
    }

    fn get_number(&self) -> i32 {
        unsafe {
            gmime_sys::g_mime_message_partial_get_number(self.as_ref().to_glib_none().0)
        }
    }

    fn get_total(&self) -> i32 {
        unsafe {
            gmime_sys::g_mime_message_partial_get_total(self.as_ref().to_glib_none().0)
        }
    }
}

impl fmt::Display for MessagePartial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MessagePartial")
    }
}
