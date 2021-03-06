// This file was generated by gir (https://github.com/gtk-rs/gir @ 00040a2)
// from gir-files (https://github.com/gtk-rs/gir-files @ 5b96546)
// DO NOT EDIT

use crate::AutocryptHeaderList;
use crate::ContentDisposition;
use crate::ContentType;
use crate::EncodingConstraint;
use crate::FormatOptions;
use crate::HeaderList;
use crate::InternetAddressList;
use crate::ParserOptions;
use crate::Stream;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct Object(Object<ffi::GMimeObject, ffi::GMimeObjectClass>);

    match fn {
        get_type => || ffi::g_mime_object_get_type(),
    }
}

impl Object {
    #[doc(alias = "g_mime_object_new")]
    pub fn new<P: IsA<ContentType>>(options: Option<&ParserOptions>, content_type: &P) -> Object {
        unsafe {
            from_glib_full(ffi::g_mime_object_new(mut_override(options.to_glib_none().0), content_type.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "g_mime_object_new_type")]
    pub fn new_type(options: Option<&ParserOptions>, type_: &str, subtype: &str) -> Object {
        unsafe {
            from_glib_full(ffi::g_mime_object_new_type(mut_override(options.to_glib_none().0), type_.to_glib_none().0, subtype.to_glib_none().0))
        }
    }

    #[doc(alias = "g_mime_object_register_type")]
    pub fn register_type(type_: &str, subtype: &str, object_type: glib::types::Type) {
        unsafe {
            ffi::g_mime_object_register_type(type_.to_glib_none().0, subtype.to_glib_none().0, object_type.to_glib());
        }
    }

    #[doc(alias = "g_mime_object_type_registry_init")]
    pub fn type_registry_init() {
        unsafe {
            ffi::g_mime_object_type_registry_init();
        }
    }

    #[doc(alias = "g_mime_object_type_registry_shutdown")]
    pub fn type_registry_shutdown() {
        unsafe {
            ffi::g_mime_object_type_registry_shutdown();
        }
    }
}

pub const NONE_OBJECT: Option<&Object> = None;

pub trait ObjectExt: 'static {
    #[doc(alias = "g_mime_object_append_header")]
    fn append_header(&self, header: &str, value: &str, charset: &str);

    #[doc(alias = "g_mime_object_encode")]
    fn encode(&self, constraint: EncodingConstraint);

    #[doc(alias = "g_mime_object_get_autocrypt_headers")]
    fn get_autocrypt_headers<P: IsA<InternetAddressList>>(&self, effective_date: &glib::DateTime, matchheader: &str, addresses: &P, keep_incomplete: bool) -> Option<AutocryptHeaderList>;

    #[doc(alias = "g_mime_object_get_content_disposition")]
    fn get_content_disposition(&self) -> Option<ContentDisposition>;

    #[doc(alias = "g_mime_object_get_content_disposition_parameter")]
    fn get_content_disposition_parameter(&self, name: &str) -> Option<glib::GString>;

    #[doc(alias = "g_mime_object_get_content_id")]
    fn get_content_id(&self) -> Option<glib::GString>;

    #[doc(alias = "g_mime_object_get_content_type")]
    fn get_content_type(&self) -> Option<ContentType>;

    #[doc(alias = "g_mime_object_get_content_type_parameter")]
    fn get_content_type_parameter(&self, name: &str) -> Option<glib::GString>;

    #[doc(alias = "g_mime_object_get_disposition")]
    fn get_disposition(&self) -> Option<glib::GString>;

    #[doc(alias = "g_mime_object_get_header")]
    fn get_header(&self, header: &str) -> Option<glib::GString>;

    #[doc(alias = "g_mime_object_get_header_list")]
    fn get_header_list(&self) -> Option<HeaderList>;

    #[doc(alias = "g_mime_object_get_headers")]
    fn get_headers(&self, options: Option<&FormatOptions>) -> Option<glib::GString>;

    #[doc(alias = "g_mime_object_prepend_header")]
    fn prepend_header(&self, header: &str, value: &str, charset: &str);

    #[doc(alias = "g_mime_object_remove_header")]
    fn remove_header(&self, header: &str) -> bool;

    #[doc(alias = "g_mime_object_set_content_disposition")]
    fn set_content_disposition<P: IsA<ContentDisposition>>(&self, disposition: &P);

    #[doc(alias = "g_mime_object_set_content_disposition_parameter")]
    fn set_content_disposition_parameter(&self, name: &str, value: &str);

    #[doc(alias = "g_mime_object_set_content_id")]
    fn set_content_id(&self, content_id: &str);

    #[doc(alias = "g_mime_object_set_content_type")]
    fn set_content_type<P: IsA<ContentType>>(&self, content_type: &P);

    #[doc(alias = "g_mime_object_set_content_type_parameter")]
    fn set_content_type_parameter(&self, name: &str, value: &str);

    #[doc(alias = "g_mime_object_set_disposition")]
    fn set_disposition(&self, disposition: &str);

    #[doc(alias = "g_mime_object_set_header")]
    fn set_header(&self, header: &str, value: &str, charset: &str);

    #[doc(alias = "g_mime_object_to_string")]
    fn to_string(&self, options: Option<&FormatOptions>) -> Option<glib::GString>;

    #[doc(alias = "g_mime_object_write_to_stream")]
    fn write_to_stream<P: IsA<Stream>>(&self, options: Option<&FormatOptions>, stream: &P) -> isize;
}

impl<O: IsA<Object>> ObjectExt for O {
    fn append_header(&self, header: &str, value: &str, charset: &str) {
        unsafe {
            ffi::g_mime_object_append_header(self.as_ref().to_glib_none().0, header.to_glib_none().0, value.to_glib_none().0, charset.to_glib_none().0);
        }
    }

    fn encode(&self, constraint: EncodingConstraint) {
        unsafe {
            ffi::g_mime_object_encode(self.as_ref().to_glib_none().0, constraint.to_glib());
        }
    }

    fn get_autocrypt_headers<P: IsA<InternetAddressList>>(&self, effective_date: &glib::DateTime, matchheader: &str, addresses: &P, keep_incomplete: bool) -> Option<AutocryptHeaderList> {
        unsafe {
            from_glib_none(ffi::g_mime_object_get_autocrypt_headers(self.as_ref().to_glib_none().0, effective_date.to_glib_none().0, matchheader.to_glib_none().0, addresses.as_ref().to_glib_none().0, keep_incomplete.to_glib()))
        }
    }

    fn get_content_disposition(&self) -> Option<ContentDisposition> {
        unsafe {
            from_glib_none(ffi::g_mime_object_get_content_disposition(self.as_ref().to_glib_none().0))
        }
    }

    fn get_content_disposition_parameter(&self, name: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_mime_object_get_content_disposition_parameter(self.as_ref().to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn get_content_id(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_mime_object_get_content_id(self.as_ref().to_glib_none().0))
        }
    }

    fn get_content_type(&self) -> Option<ContentType> {
        unsafe {
            from_glib_none(ffi::g_mime_object_get_content_type(self.as_ref().to_glib_none().0))
        }
    }

    fn get_content_type_parameter(&self, name: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_mime_object_get_content_type_parameter(self.as_ref().to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn get_disposition(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_mime_object_get_disposition(self.as_ref().to_glib_none().0))
        }
    }

    fn get_header(&self, header: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_mime_object_get_header(self.as_ref().to_glib_none().0, header.to_glib_none().0))
        }
    }

    fn get_header_list(&self) -> Option<HeaderList> {
        unsafe {
            from_glib_none(ffi::g_mime_object_get_header_list(self.as_ref().to_glib_none().0))
        }
    }

    fn get_headers(&self, options: Option<&FormatOptions>) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::g_mime_object_get_headers(self.as_ref().to_glib_none().0, mut_override(options.to_glib_none().0)))
        }
    }

    fn prepend_header(&self, header: &str, value: &str, charset: &str) {
        unsafe {
            ffi::g_mime_object_prepend_header(self.as_ref().to_glib_none().0, header.to_glib_none().0, value.to_glib_none().0, charset.to_glib_none().0);
        }
    }

    fn remove_header(&self, header: &str) -> bool {
        unsafe {
            from_glib(ffi::g_mime_object_remove_header(self.as_ref().to_glib_none().0, header.to_glib_none().0))
        }
    }

    fn set_content_disposition<P: IsA<ContentDisposition>>(&self, disposition: &P) {
        unsafe {
            ffi::g_mime_object_set_content_disposition(self.as_ref().to_glib_none().0, disposition.as_ref().to_glib_none().0);
        }
    }

    fn set_content_disposition_parameter(&self, name: &str, value: &str) {
        unsafe {
            ffi::g_mime_object_set_content_disposition_parameter(self.as_ref().to_glib_none().0, name.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_content_id(&self, content_id: &str) {
        unsafe {
            ffi::g_mime_object_set_content_id(self.as_ref().to_glib_none().0, content_id.to_glib_none().0);
        }
    }

    fn set_content_type<P: IsA<ContentType>>(&self, content_type: &P) {
        unsafe {
            ffi::g_mime_object_set_content_type(self.as_ref().to_glib_none().0, content_type.as_ref().to_glib_none().0);
        }
    }

    fn set_content_type_parameter(&self, name: &str, value: &str) {
        unsafe {
            ffi::g_mime_object_set_content_type_parameter(self.as_ref().to_glib_none().0, name.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_disposition(&self, disposition: &str) {
        unsafe {
            ffi::g_mime_object_set_disposition(self.as_ref().to_glib_none().0, disposition.to_glib_none().0);
        }
    }

    fn set_header(&self, header: &str, value: &str, charset: &str) {
        unsafe {
            ffi::g_mime_object_set_header(self.as_ref().to_glib_none().0, header.to_glib_none().0, value.to_glib_none().0, charset.to_glib_none().0);
        }
    }

    fn to_string(&self, options: Option<&FormatOptions>) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::g_mime_object_to_string(self.as_ref().to_glib_none().0, mut_override(options.to_glib_none().0)))
        }
    }

    fn write_to_stream<P: IsA<Stream>>(&self, options: Option<&FormatOptions>, stream: &P) -> isize {
        unsafe {
            ffi::g_mime_object_write_to_stream(self.as_ref().to_glib_none().0, mut_override(options.to_glib_none().0), stream.as_ref().to_glib_none().0)
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Object")
    }
}
