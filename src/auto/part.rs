// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff+)
// from gir-files (https://github.com/gtk-rs/gir-files @ 33386b3)
// DO NOT EDIT

use DataWrapper;
use Object;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Part(Object<ffi::GMimePart, ffi::GMimePartClass>): Object;

    match fn {
        get_type => || ffi::g_mime_part_get_type(),
    }
}

impl Part {
    pub fn new() -> Part {
        unsafe {
            from_glib_full(ffi::g_mime_part_new())
        }
    }

    pub fn new_with_type(type_: &str, subtype: &str) -> Part {
        unsafe {
            from_glib_full(ffi::g_mime_part_new_with_type(type_.to_glib_none().0, subtype.to_glib_none().0))
        }
    }
}

impl Default for Part {
    fn default() -> Self {
        Self::new()
    }
}

pub trait PartExt {
    //fn get_best_content_encoding(&self, constraint: /*Ignored*/EncodingConstraint) -> /*Ignored*/ContentEncoding;

    fn get_content(&self) -> Option<DataWrapper>;

    fn get_content_description(&self) -> Option<String>;

    //fn get_content_encoding(&self) -> /*Ignored*/ContentEncoding;

    fn get_content_location(&self) -> Option<String>;

    fn get_content_md5(&self) -> Option<String>;

    fn get_filename(&self) -> Option<String>;

    //fn get_openpgp_data(&self) -> /*Ignored*/OpenPGPData;

    fn is_attachment(&self) -> bool;

    //fn openpgp_decrypt<'a, P: Into<Option<&'a str>>>(&self, flags: DecryptFlags, session_key: P, error: /*Ignored*/Option<Error>) -> Option<DecryptResult>;

    //fn openpgp_encrypt<'a, P: Into<Option<&'a str>>>(&self, sign: bool, userid: P, flags: EncryptFlags, recipients: /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 0, id: 28 }, error: /*Ignored*/Option<Error>) -> bool;

    //fn openpgp_sign(&self, userid: &str, error: /*Ignored*/Option<Error>) -> bool;

    //fn openpgp_verify(&self, flags: VerifyFlags, error: /*Ignored*/Option<Error>) -> Option<SignatureList>;

    fn set_content(&self, content: &DataWrapper);

    fn set_content_description(&self, description: &str);

    //fn set_content_encoding(&self, encoding: /*Ignored*/ContentEncoding);

    fn set_content_location(&self, content_location: &str);

    fn set_content_md5(&self, content_md5: &str);

    fn set_filename(&self, filename: &str);

    //fn set_openpgp_data(&self, data: /*Ignored*/OpenPGPData);

    fn verify_content_md5(&self) -> bool;
}

impl<O: IsA<Part>> PartExt for O {
    //fn get_best_content_encoding(&self, constraint: /*Ignored*/EncodingConstraint) -> /*Ignored*/ContentEncoding {
    //    unsafe { TODO: call ffi::g_mime_part_get_best_content_encoding() }
    //}

    fn get_content(&self) -> Option<DataWrapper> {
        unsafe {
            from_glib_none(ffi::g_mime_part_get_content(self.to_glib_none().0))
        }
    }

    fn get_content_description(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_mime_part_get_content_description(self.to_glib_none().0))
        }
    }

    //fn get_content_encoding(&self) -> /*Ignored*/ContentEncoding {
    //    unsafe { TODO: call ffi::g_mime_part_get_content_encoding() }
    //}

    fn get_content_location(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_mime_part_get_content_location(self.to_glib_none().0))
        }
    }

    fn get_content_md5(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_mime_part_get_content_md5(self.to_glib_none().0))
        }
    }

    fn get_filename(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_mime_part_get_filename(self.to_glib_none().0))
        }
    }

    //fn get_openpgp_data(&self) -> /*Ignored*/OpenPGPData {
    //    unsafe { TODO: call ffi::g_mime_part_get_openpgp_data() }
    //}

    fn is_attachment(&self) -> bool {
        unsafe {
            from_glib(ffi::g_mime_part_is_attachment(self.to_glib_none().0))
        }
    }

    //fn openpgp_decrypt<'a, P: Into<Option<&'a str>>>(&self, flags: DecryptFlags, session_key: P, error: /*Ignored*/Option<Error>) -> Option<DecryptResult> {
    //    unsafe { TODO: call ffi::g_mime_part_openpgp_decrypt() }
    //}

    //fn openpgp_encrypt<'a, P: Into<Option<&'a str>>>(&self, sign: bool, userid: P, flags: EncryptFlags, recipients: /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 0, id: 28 }, error: /*Ignored*/Option<Error>) -> bool {
    //    unsafe { TODO: call ffi::g_mime_part_openpgp_encrypt() }
    //}

    //fn openpgp_sign(&self, userid: &str, error: /*Ignored*/Option<Error>) -> bool {
    //    unsafe { TODO: call ffi::g_mime_part_openpgp_sign() }
    //}

    //fn openpgp_verify(&self, flags: VerifyFlags, error: /*Ignored*/Option<Error>) -> Option<SignatureList> {
    //    unsafe { TODO: call ffi::g_mime_part_openpgp_verify() }
    //}

    fn set_content(&self, content: &DataWrapper) {
        unsafe {
            ffi::g_mime_part_set_content(self.to_glib_none().0, content.to_glib_none().0);
        }
    }

    fn set_content_description(&self, description: &str) {
        unsafe {
            ffi::g_mime_part_set_content_description(self.to_glib_none().0, description.to_glib_none().0);
        }
    }

    //fn set_content_encoding(&self, encoding: /*Ignored*/ContentEncoding) {
    //    unsafe { TODO: call ffi::g_mime_part_set_content_encoding() }
    //}

    fn set_content_location(&self, content_location: &str) {
        unsafe {
            ffi::g_mime_part_set_content_location(self.to_glib_none().0, content_location.to_glib_none().0);
        }
    }

    fn set_content_md5(&self, content_md5: &str) {
        unsafe {
            ffi::g_mime_part_set_content_md5(self.to_glib_none().0, content_md5.to_glib_none().0);
        }
    }

    fn set_filename(&self, filename: &str) {
        unsafe {
            ffi::g_mime_part_set_filename(self.to_glib_none().0, filename.to_glib_none().0);
        }
    }

    //fn set_openpgp_data(&self, data: /*Ignored*/OpenPGPData) {
    //    unsafe { TODO: call ffi::g_mime_part_set_openpgp_data() }
    //}

    fn verify_content_md5(&self) -> bool {
        unsafe {
            from_glib(ffi::g_mime_part_verify_content_md5(self.to_glib_none().0))
        }
    }
}
