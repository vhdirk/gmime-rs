// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff+)
// from gir-files (https://github.com/gtk-rs/gir-files @ 33386b3)
// DO NOT EDIT

use DigestAlgo;
use PubKeyAlgo;
use Trust;
use Validity;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Certificate(Object<ffi::GMimeCertificate, ffi::GMimeCertificateClass>);

    match fn {
        get_type => || ffi::g_mime_certificate_get_type(),
    }
}

impl Certificate {
    pub fn new() -> Certificate {
        unsafe {
            from_glib_full(ffi::g_mime_certificate_new())
        }
    }
}

impl Default for Certificate {
    fn default() -> Self {
        Self::new()
    }
}

pub trait CertificateExt {
    fn get_created(&self) -> libc::c_long;

    fn get_digest_algo(&self) -> DigestAlgo;

    fn get_email(&self) -> Option<String>;

    fn get_expires(&self) -> libc::c_long;

    fn get_fingerprint(&self) -> Option<String>;

    fn get_id_validity(&self) -> Validity;

    fn get_issuer_name(&self) -> Option<String>;

    fn get_issuer_serial(&self) -> Option<String>;

    fn get_key_id(&self) -> Option<String>;

    fn get_name(&self) -> Option<String>;

    fn get_pubkey_algo(&self) -> PubKeyAlgo;

    fn get_trust(&self) -> Trust;

    fn get_user_id(&self) -> Option<String>;

    fn set_created(&self, created: libc::c_long);

    fn set_digest_algo(&self, algo: DigestAlgo);

    fn set_email(&self, email: &str);

    fn set_expires(&self, expires: libc::c_long);

    fn set_fingerprint(&self, fingerprint: &str);

    fn set_id_validity(&self, validity: Validity);

    fn set_issuer_name(&self, issuer_name: &str);

    fn set_issuer_serial(&self, issuer_serial: &str);

    fn set_key_id(&self, key_id: &str);

    fn set_name(&self, name: &str);

    fn set_pubkey_algo(&self, algo: PubKeyAlgo);

    fn set_trust(&self, trust: Trust);

    fn set_user_id(&self, user_id: &str);
}

impl<O: IsA<Certificate>> CertificateExt for O {
    fn get_created(&self) -> libc::c_long {
        unsafe {
            ffi::g_mime_certificate_get_created(self.to_glib_none().0)
        }
    }

    fn get_digest_algo(&self) -> DigestAlgo {
        unsafe {
            from_glib(ffi::g_mime_certificate_get_digest_algo(self.to_glib_none().0))
        }
    }

    fn get_email(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_mime_certificate_get_email(self.to_glib_none().0))
        }
    }

    fn get_expires(&self) -> libc::c_long {
        unsafe {
            ffi::g_mime_certificate_get_expires(self.to_glib_none().0)
        }
    }

    fn get_fingerprint(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_mime_certificate_get_fingerprint(self.to_glib_none().0))
        }
    }

    fn get_id_validity(&self) -> Validity {
        unsafe {
            from_glib(ffi::g_mime_certificate_get_id_validity(self.to_glib_none().0))
        }
    }

    fn get_issuer_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_mime_certificate_get_issuer_name(self.to_glib_none().0))
        }
    }

    fn get_issuer_serial(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_mime_certificate_get_issuer_serial(self.to_glib_none().0))
        }
    }

    fn get_key_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_mime_certificate_get_key_id(self.to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_mime_certificate_get_name(self.to_glib_none().0))
        }
    }

    fn get_pubkey_algo(&self) -> PubKeyAlgo {
        unsafe {
            from_glib(ffi::g_mime_certificate_get_pubkey_algo(self.to_glib_none().0))
        }
    }

    fn get_trust(&self) -> Trust {
        unsafe {
            from_glib(ffi::g_mime_certificate_get_trust(self.to_glib_none().0))
        }
    }

    fn get_user_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_mime_certificate_get_user_id(self.to_glib_none().0))
        }
    }

    fn set_created(&self, created: libc::c_long) {
        unsafe {
            ffi::g_mime_certificate_set_created(self.to_glib_none().0, created);
        }
    }

    fn set_digest_algo(&self, algo: DigestAlgo) {
        unsafe {
            ffi::g_mime_certificate_set_digest_algo(self.to_glib_none().0, algo.to_glib());
        }
    }

    fn set_email(&self, email: &str) {
        unsafe {
            ffi::g_mime_certificate_set_email(self.to_glib_none().0, email.to_glib_none().0);
        }
    }

    fn set_expires(&self, expires: libc::c_long) {
        unsafe {
            ffi::g_mime_certificate_set_expires(self.to_glib_none().0, expires);
        }
    }

    fn set_fingerprint(&self, fingerprint: &str) {
        unsafe {
            ffi::g_mime_certificate_set_fingerprint(self.to_glib_none().0, fingerprint.to_glib_none().0);
        }
    }

    fn set_id_validity(&self, validity: Validity) {
        unsafe {
            ffi::g_mime_certificate_set_id_validity(self.to_glib_none().0, validity.to_glib());
        }
    }

    fn set_issuer_name(&self, issuer_name: &str) {
        unsafe {
            ffi::g_mime_certificate_set_issuer_name(self.to_glib_none().0, issuer_name.to_glib_none().0);
        }
    }

    fn set_issuer_serial(&self, issuer_serial: &str) {
        unsafe {
            ffi::g_mime_certificate_set_issuer_serial(self.to_glib_none().0, issuer_serial.to_glib_none().0);
        }
    }

    fn set_key_id(&self, key_id: &str) {
        unsafe {
            ffi::g_mime_certificate_set_key_id(self.to_glib_none().0, key_id.to_glib_none().0);
        }
    }

    fn set_name(&self, name: &str) {
        unsafe {
            ffi::g_mime_certificate_set_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    fn set_pubkey_algo(&self, algo: PubKeyAlgo) {
        unsafe {
            ffi::g_mime_certificate_set_pubkey_algo(self.to_glib_none().0, algo.to_glib());
        }
    }

    fn set_trust(&self, trust: Trust) {
        unsafe {
            ffi::g_mime_certificate_set_trust(self.to_glib_none().0, trust.to_glib());
        }
    }

    fn set_user_id(&self, user_id: &str) {
        unsafe {
            ffi::g_mime_certificate_set_user_id(self.to_glib_none().0, user_id.to_glib_none().0);
        }
    }
}
