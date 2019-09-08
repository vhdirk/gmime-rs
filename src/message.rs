use AddressType;
use AutocryptHeader;
use AutocryptHeaderList;
use DecryptFlags;
use Error;
use InternetAddressList;
use Object;
use gmime_sys;
use glib;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib_wrapper! {
    pub struct Message(Object<gmime_sys::GMimeMessage, gmime_sys::GMimeMessageClass, MessageClass>) @extends Object;

    match fn {
        get_type => || gmime_sys::g_mime_message_get_type(),
    }
}

impl Message {
    pub fn new(pretty_headers: bool) -> Message {
        unsafe {
            from_glib_full(gmime_sys::g_mime_message_new(pretty_headers.to_glib()))
        }
    }
}

pub const NONE_MESSAGE: Option<&Message> = None;

pub trait MessageExt: 'static {
    fn add_mailbox(&self, type_: AddressType, name: &str, addr: &str);

    //fn foreach<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, callback: /*Unknown conversion*//*Unimplemented*/ObjectForeachFunc, user_data: P);

    fn get_addresses(&self, type_: AddressType) -> Option<InternetAddressList>;

    fn get_all_recipients(&self) -> Option<InternetAddressList>;

    fn get_autocrypt_gossip_headers(&self, now: &glib::DateTime, flags: DecryptFlags, session_key: &str) -> Result<AutocryptHeaderList, Error>;

    fn get_autocrypt_gossip_headers_from_inner_part<P: IsA<Object>>(&self, now: &glib::DateTime, inner_part: &P) -> Option<AutocryptHeaderList>;

    fn get_autocrypt_header(&self, now: &glib::DateTime) -> Option<AutocryptHeader>;

    fn get_bcc(&self) -> Option<InternetAddressList>;

    fn get_body(&self) -> Option<Object>;

    fn get_cc(&self) -> Option<InternetAddressList>;

    fn get_date(&self) -> Option<glib::DateTime>;

    fn get_from(&self) -> Option<InternetAddressList>;

    fn get_message_id(&self) -> Option<GString>;

    fn get_mime_part(&self) -> Option<Object>;

    fn get_reply_to(&self) -> Option<InternetAddressList>;

    fn get_sender(&self) -> Option<InternetAddressList>;

    fn get_subject(&self) -> Option<GString>;

    fn get_to(&self) -> Option<InternetAddressList>;

    fn split(&self, max_size: usize) -> Vec<Message>;

    fn set_date(&self, date: &glib::DateTime);

    fn set_message_id(&self, message_id: &str);

    fn set_mime_part<P: IsA<Object>>(&self, mime_part: &P);

    fn set_subject(&self, subject: &str, charset: &str);
}

impl<O: IsA<Message>> MessageExt for O {
    fn add_mailbox(&self, type_: AddressType, name: &str, addr: &str) {
        unsafe {
            gmime_sys::g_mime_message_add_mailbox(self.as_ref().to_glib_none().0, type_.to_glib(), name.to_glib_none().0, addr.to_glib_none().0);
        }
    }

    //fn foreach<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, callback: /*Unknown conversion*//*Unimplemented*/ObjectForeachFunc, user_data: P) {
    //    unsafe { TODO: call gmime_sys::g_mime_message_foreach() }
    //}

    fn get_addresses(&self, type_: AddressType) -> Option<InternetAddressList> {
        unsafe {
            from_glib_none(gmime_sys::g_mime_message_get_addresses(self.as_ref().to_glib_none().0, type_.to_glib()))
        }
    }

    fn get_all_recipients(&self) -> Option<InternetAddressList> {
        unsafe {
            from_glib_full(gmime_sys::g_mime_message_get_all_recipients(self.as_ref().to_glib_none().0))
        }
    }

    fn get_autocrypt_gossip_headers(&self, now: &glib::DateTime, flags: DecryptFlags, session_key: &str) -> Result<AutocryptHeaderList, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gmime_sys::g_mime_message_get_autocrypt_gossip_headers(self.as_ref().to_glib_none().0, now.to_glib_none().0, flags.to_glib(), session_key.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_autocrypt_gossip_headers_from_inner_part<P: IsA<Object>>(&self, now: &glib::DateTime, inner_part: &P) -> Option<AutocryptHeaderList> {
        unsafe {
            from_glib_full(gmime_sys::g_mime_message_get_autocrypt_gossip_headers_from_inner_part(self.as_ref().to_glib_none().0, now.to_glib_none().0, inner_part.as_ref().to_glib_none().0))
        }
    }

    fn get_autocrypt_header(&self, now: &glib::DateTime) -> Option<AutocryptHeader> {
        unsafe {
            from_glib_full(gmime_sys::g_mime_message_get_autocrypt_header(self.as_ref().to_glib_none().0, now.to_glib_none().0))
        }
    }

    fn get_bcc(&self) -> Option<InternetAddressList> {
        unsafe {
            from_glib_none(gmime_sys::g_mime_message_get_bcc(self.as_ref().to_glib_none().0))
        }
    }

    fn get_body(&self) -> Option<Object> {
        unsafe {
            from_glib_none(gmime_sys::g_mime_message_get_body(self.as_ref().to_glib_none().0))
        }
    }

    fn get_cc(&self) -> Option<InternetAddressList> {
        unsafe {
            from_glib_none(gmime_sys::g_mime_message_get_cc(self.as_ref().to_glib_none().0))
        }
    }

    fn get_date(&self) -> Option<glib::DateTime> {
        unsafe {
            from_glib_full(gmime_sys::g_mime_message_get_date(self.as_ref().to_glib_none().0))
        }
    }

    fn get_from(&self) -> Option<InternetAddressList> {
        unsafe {
            from_glib_none(gmime_sys::g_mime_message_get_from(self.as_ref().to_glib_none().0))
        }
    }

    fn get_message_id(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gmime_sys::g_mime_message_get_message_id(self.as_ref().to_glib_none().0))
        }
    }

    fn get_mime_part(&self) -> Option<Object> {
        unsafe {
            from_glib_none(gmime_sys::g_mime_message_get_mime_part(self.as_ref().to_glib_none().0))
        }
    }

    fn get_reply_to(&self) -> Option<InternetAddressList> {
        unsafe {
            from_glib_none(gmime_sys::g_mime_message_get_reply_to(self.as_ref().to_glib_none().0))
        }
    }

    fn get_sender(&self) -> Option<InternetAddressList> {
        unsafe {
            from_glib_none(gmime_sys::g_mime_message_get_sender(self.as_ref().to_glib_none().0))
        }
    }

    fn get_subject(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gmime_sys::g_mime_message_get_subject(self.as_ref().to_glib_none().0))
        }
    }

    fn get_to(&self) -> Option<InternetAddressList> {
        unsafe {
            from_glib_none(gmime_sys::g_mime_message_get_to(self.as_ref().to_glib_none().0))
        }
    }

    fn split(&self, max_size: usize) -> Vec<Message> {
        unsafe {
            let mut n_parts = ::std::mem::uninitialized();
            let parts = gmime_sys::g_mime_message_partial_split_message(self.as_ref().to_glib_none().0, max_size, &mut n_parts);
            FromGlibContainer::from_glib_full_num(parts, n_parts as usize)
        }
    }

    fn set_date(&self, date: &glib::DateTime) {
        unsafe {
            gmime_sys::g_mime_message_set_date(self.as_ref().to_glib_none().0, date.to_glib_none().0);
        }
    }

    fn set_message_id(&self, message_id: &str) {
        unsafe {
            gmime_sys::g_mime_message_set_message_id(self.as_ref().to_glib_none().0, message_id.to_glib_none().0);
        }
    }

    fn set_mime_part<P: IsA<Object>>(&self, mime_part: &P) {
        unsafe {
            gmime_sys::g_mime_message_set_mime_part(self.as_ref().to_glib_none().0, mime_part.as_ref().to_glib_none().0);
        }
    }

    fn set_subject(&self, subject: &str, charset: &str) {
        unsafe {
            gmime_sys::g_mime_message_set_subject(self.as_ref().to_glib_none().0, subject.to_glib_none().0, charset.to_glib_none().0);
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Message")
    }
}
