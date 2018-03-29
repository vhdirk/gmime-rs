// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff+)
// from gir-files (https://github.com/gtk-rs/gir-files @ b215ee8)
// DO NOT EDIT

use CryptoContext;
use Error;
use Multipart;
use Object;
use SignatureList;
use VerifyFlags;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct MultipartSigned(Object<ffi::GMimeMultipartSigned, ffi::GMimeMultipartSignedClass>): Multipart, Object;

    match fn {
        get_type => || ffi::g_mime_multipart_signed_get_type(),
    }
}

impl MultipartSigned {
    pub fn new() -> MultipartSigned {
        unsafe {
            from_glib_full(ffi::g_mime_multipart_signed_new())
        }
    }

    pub fn sign<P: IsA<CryptoContext>, Q: IsA<Object>>(ctx: &P, entity: &Q, userid: &str) -> Result<Option<MultipartSigned>, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_mime_multipart_signed_sign(ctx.to_glib_none().0, entity.to_glib_none().0, userid.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}

impl Default for MultipartSigned {
    fn default() -> Self {
        Self::new()
    }
}

pub trait MultipartSignedExt {
    fn verify(&self, flags: VerifyFlags) -> Result<Option<SignatureList>, Error>;
}

impl<O: IsA<MultipartSigned>> MultipartSignedExt for O {
    fn verify(&self, flags: VerifyFlags) -> Result<Option<SignatureList>, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_mime_multipart_signed_verify(self.to_glib_none().0, flags.to_glib(), &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}
