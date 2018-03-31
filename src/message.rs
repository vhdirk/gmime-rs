use AddressType;
use AutocryptHeader;
use AutocryptHeaderList;
use DecryptFlags;
use Error;
use InternetAddressList;
use Object;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Message(Object<ffi::GMimeMessage, ffi::GMimeMessageClass>): Object;

    match fn {
        get_type => || ffi::g_mime_message_get_type(),
    }
}

impl Message {
    pub fn new(pretty_headers: bool) -> Message {
        unsafe {
            from_glib_full(ffi::g_mime_message_new(pretty_headers.to_glib()))
        }
    }
}

pub trait MessageExt {
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

    fn get_message_id(&self) -> Option<String>;

    fn get_mime_part(&self) -> Option<Object>;

    fn get_reply_to(&self) -> Option<InternetAddressList>;

    fn get_sender(&self) -> Option<InternetAddressList>;

    fn get_subject(&self) -> Option<String>;

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
            ffi::g_mime_message_add_mailbox(self.to_glib_none().0, type_.to_glib(), name.to_glib_none().0, addr.to_glib_none().0);
        }
    }

    //fn foreach<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, callback: /*Unknown conversion*//*Unimplemented*/ObjectForeachFunc, user_data: P) {
    //    unsafe { TODO: call ffi::g_mime_message_foreach() }
    //}

    fn get_addresses(&self, type_: AddressType) -> Option<InternetAddressList> {
        unsafe {
            from_glib_none(ffi::g_mime_message_get_addresses(self.to_glib_none().0, type_.to_glib()))
        }
    }

    fn get_all_recipients(&self) -> Option<InternetAddressList> {
        unsafe {
            from_glib_full(ffi::g_mime_message_get_all_recipients(self.to_glib_none().0))
        }
    }

    fn get_autocrypt_gossip_headers(&self, now: &glib::DateTime, flags: DecryptFlags, session_key: &str) -> Result<AutocryptHeaderList, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_mime_message_get_autocrypt_gossip_headers(self.to_glib_none().0, now.to_glib_none().0, flags.to_glib(), session_key.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_autocrypt_gossip_headers_from_inner_part<P: IsA<Object>>(&self, now: &glib::DateTime, inner_part: &P) -> Option<AutocryptHeaderList> {
        unsafe {
            from_glib_full(ffi::g_mime_message_get_autocrypt_gossip_headers_from_inner_part(self.to_glib_none().0, now.to_glib_none().0, inner_part.to_glib_none().0))
        }
    }

    fn get_autocrypt_header(&self, now: &glib::DateTime) -> Option<AutocryptHeader> {
        unsafe {
            from_glib_full(ffi::g_mime_message_get_autocrypt_header(self.to_glib_none().0, now.to_glib_none().0))
        }
    }

    fn get_bcc(&self) -> Option<InternetAddressList> {
        unsafe {
            from_glib_none(ffi::g_mime_message_get_bcc(self.to_glib_none().0))
        }
    }

    fn get_body(&self) -> Option<Object> {
        unsafe {
            from_glib_none(ffi::g_mime_message_get_body(self.to_glib_none().0))
        }
    }

    fn get_cc(&self) -> Option<InternetAddressList> {
        unsafe {
            from_glib_none(ffi::g_mime_message_get_cc(self.to_glib_none().0))
        }
    }

    fn get_date(&self) -> Option<glib::DateTime> {
        unsafe {
            from_glib_full(ffi::g_mime_message_get_date(self.to_glib_none().0))
        }
    }

    fn get_from(&self) -> Option<InternetAddressList> {
        unsafe {
            from_glib_none(ffi::g_mime_message_get_from(self.to_glib_none().0))
        }
    }

    fn get_message_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_mime_message_get_message_id(self.to_glib_none().0))
        }
    }

    fn get_mime_part(&self) -> Option<Object> {
        unsafe {
            from_glib_none(ffi::g_mime_message_get_mime_part(self.to_glib_none().0))
        }
    }

    fn get_reply_to(&self) -> Option<InternetAddressList> {
        unsafe {
            from_glib_none(ffi::g_mime_message_get_reply_to(self.to_glib_none().0))
        }
    }

    fn get_sender(&self) -> Option<InternetAddressList> {
        unsafe {
            from_glib_none(ffi::g_mime_message_get_sender(self.to_glib_none().0))
        }
    }

    fn get_subject(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_mime_message_get_subject(self.to_glib_none().0))
        }
    }

    fn get_to(&self) -> Option<InternetAddressList> {
        unsafe {
            from_glib_none(ffi::g_mime_message_get_to(self.to_glib_none().0))
        }
    }

    fn split(&self, max_size: usize) -> Vec<Message> {
        unsafe {
            let mut n_parts = mem::uninitialized();
            let parts = ffi::g_mime_message_partial_split_message(self.to_glib_none().0, max_size, &mut n_parts);
            FromGlibContainer::from_glib_full_num(parts, n_parts as usize)
        }
    }

    fn set_date(&self, date: &glib::DateTime) {
        unsafe {
            ffi::g_mime_message_set_date(self.to_glib_none().0, date.to_glib_none().0);
        }
    }

    fn set_message_id(&self, message_id: &str) {
        unsafe {
            ffi::g_mime_message_set_message_id(self.to_glib_none().0, message_id.to_glib_none().0);
        }
    }

    fn set_mime_part<P: IsA<Object>>(&self, mime_part: &P) {
        unsafe {
            ffi::g_mime_message_set_mime_part(self.to_glib_none().0, mime_part.to_glib_none().0);
        }
    }

    fn set_subject(&self, subject: &str, charset: &str) {
        unsafe {
            ffi::g_mime_message_set_subject(self.to_glib_none().0, subject.to_glib_none().0, charset.to_glib_none().0);
        }
    }
}
