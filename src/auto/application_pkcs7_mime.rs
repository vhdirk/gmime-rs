// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::EncryptFlags;
use crate::Object;
use crate::Part;
use crate::SecureMimeType;
use crate::SignatureList;
use crate::VerifyFlags;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GMimeApplicationPkcs7Mime")]
    pub struct ApplicationPkcs7Mime(Object<ffi::GMimeApplicationPkcs7Mime, ffi::GMimeApplicationPkcs7MimeClass>) @extends Part, Object;

    match fn {
        type_ => || ffi::g_mime_application_pkcs7_mime_get_type(),
    }
}

impl ApplicationPkcs7Mime {
    #[doc(alias = "g_mime_application_pkcs7_mime_new")]
    pub fn new(type_: SecureMimeType) -> ApplicationPkcs7Mime {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::g_mime_application_pkcs7_mime_new(type_.into_glib())) }
    }

    #[doc(alias = "g_mime_application_pkcs7_mime_encrypt")]
    pub fn encrypt(
        entity: &impl IsA<Object>,
        flags: EncryptFlags,
        recipients: &[&str],
    ) -> Result<Option<ApplicationPkcs7Mime>, glib::Error> {
        skip_assert_initialized!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_mime_application_pkcs7_mime_encrypt(
                entity.as_ref().to_glib_none().0,
                flags.into_glib(),
                recipients.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_mime_application_pkcs7_mime_sign")]
    pub fn sign(
        entity: &impl IsA<Object>,
        userid: &str,
    ) -> Result<Option<ApplicationPkcs7Mime>, glib::Error> {
        skip_assert_initialized!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_mime_application_pkcs7_mime_sign(
                entity.as_ref().to_glib_none().0,
                userid.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

pub const NONE_APPLICATION_PKCS7_MIME: Option<&ApplicationPkcs7Mime> = None;

pub trait ApplicationPkcs7MimeExt: 'static {
    #[doc(alias = "g_mime_application_pkcs7_mime_get_smime_type")]
    #[doc(alias = "get_smime_type")]
    fn smime_type(&self) -> SecureMimeType;

    #[doc(alias = "g_mime_application_pkcs7_mime_verify")]
    fn verify(&self, flags: VerifyFlags) -> Result<(Option<SignatureList>, Object), glib::Error>;
}

impl<O: IsA<ApplicationPkcs7Mime>> ApplicationPkcs7MimeExt for O {
    fn smime_type(&self) -> SecureMimeType {
        unsafe {
            from_glib(ffi::g_mime_application_pkcs7_mime_get_smime_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn verify(&self, flags: VerifyFlags) -> Result<(Option<SignatureList>, Object), glib::Error> {
        unsafe {
            let mut entity = ptr::null_mut();
            let mut error = ptr::null_mut();
            let ret = ffi::g_mime_application_pkcs7_mime_verify(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                &mut entity,
                &mut error,
            );
            if error.is_null() {
                Ok((from_glib_full(ret), from_glib_full(entity)))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl fmt::Display for ApplicationPkcs7Mime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ApplicationPkcs7Mime")
    }
}
