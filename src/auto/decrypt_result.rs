// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::CertificateList;
use crate::CipherAlgo;
use crate::DigestAlgo;
use crate::SignatureList;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GMimeDecryptResult")]
    pub struct DecryptResult(Object<ffi::GMimeDecryptResult, ffi::GMimeDecryptResultClass>);

    match fn {
        type_ => || ffi::g_mime_decrypt_result_get_type(),
    }
}

impl DecryptResult {
    #[doc(alias = "g_mime_decrypt_result_new")]
    pub fn new() -> DecryptResult {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::g_mime_decrypt_result_new()) }
    }
}

impl Default for DecryptResult {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_DECRYPT_RESULT: Option<&DecryptResult> = None;

pub trait DecryptResultExt: 'static {
    #[doc(alias = "g_mime_decrypt_result_get_cipher")]
    #[doc(alias = "get_cipher")]
    fn cipher(&self) -> CipherAlgo;

    #[doc(alias = "g_mime_decrypt_result_get_mdc")]
    #[doc(alias = "get_mdc")]
    fn mdc(&self) -> DigestAlgo;

    #[doc(alias = "g_mime_decrypt_result_get_recipients")]
    #[doc(alias = "get_recipients")]
    fn recipients(&self) -> Option<CertificateList>;

    #[doc(alias = "g_mime_decrypt_result_get_session_key")]
    #[doc(alias = "get_session_key")]
    fn session_key(&self) -> Option<glib::GString>;

    #[doc(alias = "g_mime_decrypt_result_get_signatures")]
    #[doc(alias = "get_signatures")]
    fn signatures(&self) -> Option<SignatureList>;

    #[doc(alias = "g_mime_decrypt_result_set_cipher")]
    fn set_cipher(&self, cipher: CipherAlgo);

    #[doc(alias = "g_mime_decrypt_result_set_mdc")]
    fn set_mdc(&self, mdc: DigestAlgo);

    #[doc(alias = "g_mime_decrypt_result_set_recipients")]
    fn set_recipients(&self, recipients: &impl IsA<CertificateList>);

    #[doc(alias = "g_mime_decrypt_result_set_session_key")]
    fn set_session_key(&self, session_key: Option<&str>);

    #[doc(alias = "g_mime_decrypt_result_set_signatures")]
    fn set_signatures(&self, signatures: &impl IsA<SignatureList>);
}

impl<O: IsA<DecryptResult>> DecryptResultExt for O {
    fn cipher(&self) -> CipherAlgo {
        unsafe {
            from_glib(ffi::g_mime_decrypt_result_get_cipher(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn mdc(&self) -> DigestAlgo {
        unsafe {
            from_glib(ffi::g_mime_decrypt_result_get_mdc(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn recipients(&self) -> Option<CertificateList> {
        unsafe {
            from_glib_none(ffi::g_mime_decrypt_result_get_recipients(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn session_key(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_mime_decrypt_result_get_session_key(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn signatures(&self) -> Option<SignatureList> {
        unsafe {
            from_glib_none(ffi::g_mime_decrypt_result_get_signatures(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_cipher(&self, cipher: CipherAlgo) {
        unsafe {
            ffi::g_mime_decrypt_result_set_cipher(
                self.as_ref().to_glib_none().0,
                cipher.into_glib(),
            );
        }
    }

    fn set_mdc(&self, mdc: DigestAlgo) {
        unsafe {
            ffi::g_mime_decrypt_result_set_mdc(self.as_ref().to_glib_none().0, mdc.into_glib());
        }
    }

    fn set_recipients(&self, recipients: &impl IsA<CertificateList>) {
        unsafe {
            ffi::g_mime_decrypt_result_set_recipients(
                self.as_ref().to_glib_none().0,
                recipients.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_session_key(&self, session_key: Option<&str>) {
        unsafe {
            ffi::g_mime_decrypt_result_set_session_key(
                self.as_ref().to_glib_none().0,
                session_key.to_glib_none().0,
            );
        }
    }

    fn set_signatures(&self, signatures: &impl IsA<SignatureList>) {
        unsafe {
            ffi::g_mime_decrypt_result_set_signatures(
                self.as_ref().to_glib_none().0,
                signatures.as_ref().to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for DecryptResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DecryptResult")
    }
}
