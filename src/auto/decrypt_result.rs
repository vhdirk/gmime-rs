// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff+)
// from gir-files (https://github.com/gtk-rs/gir-files @ 33386b3)
// DO NOT EDIT

use CertificateList;
use CipherAlgo;
use DigestAlgo;
use SignatureList;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct DecryptResult(Object<ffi::GMimeDecryptResult, ffi::GMimeDecryptResultClass>);

    match fn {
        get_type => || ffi::g_mime_decrypt_result_get_type(),
    }
}

impl DecryptResult {
    pub fn new() -> DecryptResult {
        unsafe {
            from_glib_full(ffi::g_mime_decrypt_result_new())
        }
    }
}

impl Default for DecryptResult {
    fn default() -> Self {
        Self::new()
    }
}

pub trait DecryptResultExt {
    fn get_cipher(&self) -> CipherAlgo;

    fn get_mdc(&self) -> DigestAlgo;

    fn get_recipients(&self) -> Option<CertificateList>;

    fn get_session_key(&self) -> Option<String>;

    fn get_signatures(&self) -> Option<SignatureList>;

    fn set_cipher(&self, cipher: CipherAlgo);

    fn set_mdc(&self, mdc: DigestAlgo);

    fn set_recipients(&self, recipients: &CertificateList);

    fn set_session_key<'a, P: Into<Option<&'a str>>>(&self, session_key: P);

    fn set_signatures(&self, signatures: &SignatureList);
}

impl<O: IsA<DecryptResult>> DecryptResultExt for O {
    fn get_cipher(&self) -> CipherAlgo {
        unsafe {
            from_glib(ffi::g_mime_decrypt_result_get_cipher(self.to_glib_none().0))
        }
    }

    fn get_mdc(&self) -> DigestAlgo {
        unsafe {
            from_glib(ffi::g_mime_decrypt_result_get_mdc(self.to_glib_none().0))
        }
    }

    fn get_recipients(&self) -> Option<CertificateList> {
        unsafe {
            from_glib_none(ffi::g_mime_decrypt_result_get_recipients(self.to_glib_none().0))
        }
    }

    fn get_session_key(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_mime_decrypt_result_get_session_key(self.to_glib_none().0))
        }
    }

    fn get_signatures(&self) -> Option<SignatureList> {
        unsafe {
            from_glib_none(ffi::g_mime_decrypt_result_get_signatures(self.to_glib_none().0))
        }
    }

    fn set_cipher(&self, cipher: CipherAlgo) {
        unsafe {
            ffi::g_mime_decrypt_result_set_cipher(self.to_glib_none().0, cipher.to_glib());
        }
    }

    fn set_mdc(&self, mdc: DigestAlgo) {
        unsafe {
            ffi::g_mime_decrypt_result_set_mdc(self.to_glib_none().0, mdc.to_glib());
        }
    }

    fn set_recipients(&self, recipients: &CertificateList) {
        unsafe {
            ffi::g_mime_decrypt_result_set_recipients(self.to_glib_none().0, recipients.to_glib_none().0);
        }
    }

    fn set_session_key<'a, P: Into<Option<&'a str>>>(&self, session_key: P) {
        let session_key = session_key.into();
        let session_key = session_key.to_glib_none();
        unsafe {
            ffi::g_mime_decrypt_result_set_session_key(self.to_glib_none().0, session_key.0);
        }
    }

    fn set_signatures(&self, signatures: &SignatureList) {
        unsafe {
            ffi::g_mime_decrypt_result_set_signatures(self.to_glib_none().0, signatures.to_glib_none().0);
        }
    }
}