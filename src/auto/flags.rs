// This file was generated by gir (https://github.com/gtk-rs/gir @ 00040a2)
// from gir-files (https://github.com/gtk-rs/gir-files @ 5b96546)
// DO NOT EDIT

use bitflags::bitflags;
use glib::translate::*;
use std::fmt;

bitflags! {
    pub struct DecryptFlags: u32 {
        const NONE = 0;
        const EXPORT_SESSION_KEY = 1;
        const NO_VERIFY = 2;
        const ENABLE_KEYSERVER_LOOKUPS = 32768;
        const ENABLE_ONLINE_CERTIFICATE_CHECKS = 32768;
    }
}

impl fmt::Display for DecryptFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl ToGlib for DecryptFlags {
    type GlibType = ffi::GMimeDecryptFlags;

    fn to_glib(&self) -> ffi::GMimeDecryptFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GMimeDecryptFlags> for DecryptFlags {
    unsafe fn from_glib(value: ffi::GMimeDecryptFlags) -> DecryptFlags {
        DecryptFlags::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct FilterBestFlags: u32 {
        const CHARSET = 1;
        const ENCODING = 2;
    }
}

impl fmt::Display for FilterBestFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl ToGlib for FilterBestFlags {
    type GlibType = ffi::GMimeFilterBestFlags;

    fn to_glib(&self) -> ffi::GMimeFilterBestFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GMimeFilterBestFlags> for FilterBestFlags {
    unsafe fn from_glib(value: ffi::GMimeFilterBestFlags) -> FilterBestFlags {
        FilterBestFlags::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct VerifyFlags: u32 {
        const NONE = 0;
        const ENABLE_KEYSERVER_LOOKUPS = 32768;
        const ENABLE_ONLINE_CERTIFICATE_CHECKS = 32768;
    }
}

impl fmt::Display for VerifyFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl ToGlib for VerifyFlags {
    type GlibType = ffi::GMimeVerifyFlags;

    fn to_glib(&self) -> ffi::GMimeVerifyFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GMimeVerifyFlags> for VerifyFlags {
    unsafe fn from_glib(value: ffi::GMimeVerifyFlags) -> VerifyFlags {
        VerifyFlags::from_bits_truncate(value)
    }
}

