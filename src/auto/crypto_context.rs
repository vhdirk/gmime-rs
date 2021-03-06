// This file was generated by gir (https://github.com/gtk-rs/gir @ 00040a2)
// from gir-files (https://github.com/gtk-rs/gir-files @ 5b96546)
// DO NOT EDIT

use crate::DecryptFlags;
use crate::DecryptResult;
use crate::DigestAlgo;
use crate::EncryptFlags;
use crate::SignatureList;
use crate::Stream;
use crate::VerifyFlags;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::wrapper! {
    pub struct CryptoContext(Object<ffi::GMimeCryptoContext, ffi::GMimeCryptoContextClass>);

    match fn {
        get_type => || ffi::g_mime_crypto_context_get_type(),
    }
}

impl CryptoContext {
    #[doc(alias = "g_mime_crypto_context_new")]
    pub fn new(protocol: &str) -> Option<CryptoContext> {
        unsafe {
            from_glib_full(ffi::g_mime_crypto_context_new(protocol.to_glib_none().0))
        }
    }

    //#[doc(alias = "g_mime_crypto_context_register")]
    //pub fn register<P: Fn() -> CryptoContext + 'static>(protocol: &str, callback: P) {
    //    unsafe { TODO: call ffi:g_mime_crypto_context_register() }
    //}
}

pub const NONE_CRYPTO_CONTEXT: Option<&CryptoContext> = None;

pub trait CryptoContextExt: 'static {
    #[doc(alias = "g_mime_crypto_context_decrypt")]
    fn decrypt<P: IsA<Stream>, Q: IsA<Stream>>(&self, flags: DecryptFlags, session_key: Option<&str>, istream: &P, ostream: &Q) -> Result<DecryptResult, glib::Error>;

    #[doc(alias = "g_mime_crypto_context_digest_id")]
    fn digest_id(&self, name: &str) -> DigestAlgo;

    #[doc(alias = "g_mime_crypto_context_digest_name")]
    fn digest_name(&self, digest: DigestAlgo) -> Option<glib::GString>;

    #[doc(alias = "g_mime_crypto_context_encrypt")]
    fn encrypt<P: IsA<Stream>, Q: IsA<Stream>>(&self, sign: bool, userid: Option<&str>, flags: EncryptFlags, recipients: &[&str], istream: &P, ostream: &Q) -> Result<i32, glib::Error>;

    #[doc(alias = "g_mime_crypto_context_export_keys")]
    fn export_keys<P: IsA<Stream>>(&self, keys: &str, ostream: &P) -> Result<i32, glib::Error>;

    #[doc(alias = "g_mime_crypto_context_get_encryption_protocol")]
    fn get_encryption_protocol(&self) -> Option<glib::GString>;

    #[doc(alias = "g_mime_crypto_context_get_key_exchange_protocol")]
    fn get_key_exchange_protocol(&self) -> Option<glib::GString>;

    #[doc(alias = "g_mime_crypto_context_get_signature_protocol")]
    fn get_signature_protocol(&self) -> Option<glib::GString>;

    #[doc(alias = "g_mime_crypto_context_import_keys")]
    fn import_keys<P: IsA<Stream>>(&self, istream: &P) -> Result<i32, glib::Error>;

    //#[doc(alias = "g_mime_crypto_context_set_request_password")]
    //fn set_request_password<P: Fn(&CryptoContext, &str, &str, bool, &Stream, Option<&glib::Error>) -> bool + 'static>(&self, request_passwd: P);

    #[doc(alias = "g_mime_crypto_context_sign")]
    fn sign<P: IsA<Stream>, Q: IsA<Stream>>(&self, detach: bool, userid: &str, istream: &P, ostream: &Q) -> Result<i32, glib::Error>;

    #[doc(alias = "g_mime_crypto_context_verify")]
    fn verify<P: IsA<Stream>, Q: IsA<Stream>, R: IsA<Stream>>(&self, flags: VerifyFlags, istream: &P, sigstream: Option<&Q>, ostream: Option<&R>) -> Result<Option<SignatureList>, glib::Error>;
}

impl<O: IsA<CryptoContext>> CryptoContextExt for O {
    fn decrypt<P: IsA<Stream>, Q: IsA<Stream>>(&self, flags: DecryptFlags, session_key: Option<&str>, istream: &P, ostream: &Q) -> Result<DecryptResult, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_mime_crypto_context_decrypt(self.as_ref().to_glib_none().0, flags.to_glib(), session_key.to_glib_none().0, istream.as_ref().to_glib_none().0, ostream.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn digest_id(&self, name: &str) -> DigestAlgo {
        unsafe {
            from_glib(ffi::g_mime_crypto_context_digest_id(self.as_ref().to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn digest_name(&self, digest: DigestAlgo) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_mime_crypto_context_digest_name(self.as_ref().to_glib_none().0, digest.to_glib()))
        }
    }

    fn encrypt<P: IsA<Stream>, Q: IsA<Stream>>(&self, sign: bool, userid: Option<&str>, flags: EncryptFlags, recipients: &[&str], istream: &P, ostream: &Q) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_mime_crypto_context_encrypt(self.as_ref().to_glib_none().0, sign.to_glib(), userid.to_glib_none().0, flags.to_glib(), recipients.to_glib_none().0, istream.as_ref().to_glib_none().0, ostream.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn export_keys<P: IsA<Stream>>(&self, keys: &str, ostream: &P) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_mime_crypto_context_export_keys(self.as_ref().to_glib_none().0, keys.to_glib_none().0, ostream.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_encryption_protocol(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_mime_crypto_context_get_encryption_protocol(self.as_ref().to_glib_none().0))
        }
    }

    fn get_key_exchange_protocol(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_mime_crypto_context_get_key_exchange_protocol(self.as_ref().to_glib_none().0))
        }
    }

    fn get_signature_protocol(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_mime_crypto_context_get_signature_protocol(self.as_ref().to_glib_none().0))
        }
    }

    fn import_keys<P: IsA<Stream>>(&self, istream: &P) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_mime_crypto_context_import_keys(self.as_ref().to_glib_none().0, istream.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    //fn set_request_password<P: Fn(&CryptoContext, &str, &str, bool, &Stream, Option<&glib::Error>) -> bool + 'static>(&self, request_passwd: P) {
    //    unsafe { TODO: call ffi:g_mime_crypto_context_set_request_password() }
    //}

    fn sign<P: IsA<Stream>, Q: IsA<Stream>>(&self, detach: bool, userid: &str, istream: &P, ostream: &Q) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_mime_crypto_context_sign(self.as_ref().to_glib_none().0, detach.to_glib(), userid.to_glib_none().0, istream.as_ref().to_glib_none().0, ostream.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn verify<P: IsA<Stream>, Q: IsA<Stream>, R: IsA<Stream>>(&self, flags: VerifyFlags, istream: &P, sigstream: Option<&Q>, ostream: Option<&R>) -> Result<Option<SignatureList>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_mime_crypto_context_verify(self.as_ref().to_glib_none().0, flags.to_glib(), istream.as_ref().to_glib_none().0, sigstream.map(|p| p.as_ref()).to_glib_none().0, ostream.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}

impl fmt::Display for CryptoContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CryptoContext")
    }
}
