// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff+)
// from gir-files (https://github.com/gtk-rs/gir-files @ 33386b3)
// DO NOT EDIT

use InternetAddressList;
use Object;
use ffi;
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
    //fn add_mailbox(&self, type_: /*Ignored*/AddressType, name: &str, addr: &str);

    //fn foreach<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, callback: /*Unknown conversion*//*Unimplemented*/ObjectForeachFunc, user_data: P);

    //fn get_addresses(&self, type_: /*Ignored*/AddressType) -> Option<InternetAddressList>;

    fn get_all_recipients(&self) -> Option<InternetAddressList>;

    //fn get_autocrypt_gossip_headers(&self, now: /*Ignored*/&glib::DateTime, flags: DecryptFlags, session_key: &str, error: /*Ignored*/Option<Error>) -> /*Ignored*/Option<AutocryptHeaderList>;

    //fn get_autocrypt_gossip_headers_from_inner_part<P: IsA<Object>>(&self, now: /*Ignored*/&glib::DateTime, inner_part: &P) -> /*Ignored*/Option<AutocryptHeaderList>;

    //fn get_autocrypt_header(&self, now: /*Ignored*/&glib::DateTime) -> /*Ignored*/Option<AutocryptHeader>;

    fn get_bcc(&self) -> Option<InternetAddressList>;

    fn get_body(&self) -> Option<Object>;

    fn get_cc(&self) -> Option<InternetAddressList>;

    //fn get_date(&self) -> /*Ignored*/Option<glib::DateTime>;

    fn get_from(&self) -> Option<InternetAddressList>;

    fn get_message_id(&self) -> Option<String>;

    fn get_mime_part(&self) -> Option<Object>;

    fn get_reply_to(&self) -> Option<InternetAddressList>;

    fn get_sender(&self) -> Option<InternetAddressList>;

    fn get_subject(&self) -> Option<String>;

    fn get_to(&self) -> Option<InternetAddressList>;

    fn partial_split_message(&self, max_size: usize) -> (Option<Message>, usize);

    //fn set_date(&self, date: /*Ignored*/&glib::DateTime);

    fn set_message_id(&self, message_id: &str);

    fn set_mime_part<P: IsA<Object>>(&self, mime_part: &P);

    fn set_subject(&self, subject: &str, charset: &str);
}

impl<O: IsA<Message>> MessageExt for O {
    //fn add_mailbox(&self, type_: /*Ignored*/AddressType, name: &str, addr: &str) {
    //    unsafe { TODO: call ffi::g_mime_message_add_mailbox() }
    //}

    //fn foreach<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, callback: /*Unknown conversion*//*Unimplemented*/ObjectForeachFunc, user_data: P) {
    //    unsafe { TODO: call ffi::g_mime_message_foreach() }
    //}

    //fn get_addresses(&self, type_: /*Ignored*/AddressType) -> Option<InternetAddressList> {
    //    unsafe { TODO: call ffi::g_mime_message_get_addresses() }
    //}

    fn get_all_recipients(&self) -> Option<InternetAddressList> {
        unsafe {
            from_glib_full(ffi::g_mime_message_get_all_recipients(self.to_glib_none().0))
        }
    }

    //fn get_autocrypt_gossip_headers(&self, now: /*Ignored*/&glib::DateTime, flags: DecryptFlags, session_key: &str, error: /*Ignored*/Option<Error>) -> /*Ignored*/Option<AutocryptHeaderList> {
    //    unsafe { TODO: call ffi::g_mime_message_get_autocrypt_gossip_headers() }
    //}

    //fn get_autocrypt_gossip_headers_from_inner_part<P: IsA<Object>>(&self, now: /*Ignored*/&glib::DateTime, inner_part: &P) -> /*Ignored*/Option<AutocryptHeaderList> {
    //    unsafe { TODO: call ffi::g_mime_message_get_autocrypt_gossip_headers_from_inner_part() }
    //}

    //fn get_autocrypt_header(&self, now: /*Ignored*/&glib::DateTime) -> /*Ignored*/Option<AutocryptHeader> {
    //    unsafe { TODO: call ffi::g_mime_message_get_autocrypt_header() }
    //}

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

    //fn get_date(&self) -> /*Ignored*/Option<glib::DateTime> {
    //    unsafe { TODO: call ffi::g_mime_message_get_date() }
    //}

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

    fn partial_split_message(&self, max_size: usize) -> (Option<Message>, usize) {
        unsafe {
            let mut nparts = mem::uninitialized();
            let ret = from_glib_full(ffi::g_mime_message_partial_split_message(self.to_glib_none().0, max_size, &mut nparts));
            (ret, nparts)
        }
    }

    //fn set_date(&self, date: /*Ignored*/&glib::DateTime) {
    //    unsafe { TODO: call ffi::g_mime_message_set_date() }
    //}

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
