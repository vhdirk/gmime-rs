// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::CryptoContext;
use crate::Multipart;
use crate::Object;
use crate::SignatureList;
use crate::VerifyFlags;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GMimeMultipartSigned")]
    pub struct MultipartSigned(Object<ffi::GMimeMultipartSigned, ffi::GMimeMultipartSignedClass>) @extends Multipart, Object;

    match fn {
        type_ => || ffi::g_mime_multipart_signed_get_type(),
    }
}

impl MultipartSigned {
    #[doc(alias = "g_mime_multipart_signed_new")]
    pub fn new() -> MultipartSigned {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::g_mime_multipart_signed_new()) }
    }

    #[doc(alias = "g_mime_multipart_signed_sign")]
    pub fn sign(
        ctx: &impl IsA<CryptoContext>,
        entity: &impl IsA<Object>,
        userid: &str,
    ) -> Result<Option<MultipartSigned>, glib::Error> {
        skip_assert_initialized!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_mime_multipart_signed_sign(
                ctx.as_ref().to_glib_none().0,
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

impl Default for MultipartSigned {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_MULTIPART_SIGNED: Option<&MultipartSigned> = None;

pub trait MultipartSignedExt: 'static {
    #[doc(alias = "g_mime_multipart_signed_verify")]
    fn verify(&self, flags: VerifyFlags) -> Result<Option<SignatureList>, glib::Error>;
}

impl<O: IsA<MultipartSigned>> MultipartSignedExt for O {
    fn verify(&self, flags: VerifyFlags) -> Result<Option<SignatureList>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_mime_multipart_signed_verify(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
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

impl fmt::Display for MultipartSigned {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MultipartSigned")
    }
}
