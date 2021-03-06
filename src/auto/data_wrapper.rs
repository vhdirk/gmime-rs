// This file was generated by gir (https://github.com/gtk-rs/gir @ 00040a2)
// from gir-files (https://github.com/gtk-rs/gir-files @ 5b96546)
// DO NOT EDIT

use crate::ContentEncoding;
use crate::Stream;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct DataWrapper(Object<ffi::GMimeDataWrapper, ffi::GMimeDataWrapperClass>);

    match fn {
        get_type => || ffi::g_mime_data_wrapper_get_type(),
    }
}

impl DataWrapper {
    #[doc(alias = "g_mime_data_wrapper_new")]
    pub fn new() -> DataWrapper {
        unsafe {
            from_glib_full(ffi::g_mime_data_wrapper_new())
        }
    }

    #[doc(alias = "g_mime_data_wrapper_new_with_stream")]
    pub fn with_stream<P: IsA<Stream>>(stream: &P, encoding: ContentEncoding) -> DataWrapper {
        unsafe {
            from_glib_full(ffi::g_mime_data_wrapper_new_with_stream(stream.as_ref().to_glib_none().0, encoding.to_glib()))
        }
    }
}

impl Default for DataWrapper {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_DATA_WRAPPER: Option<&DataWrapper> = None;

pub trait DataWrapperExt: 'static {
    #[doc(alias = "g_mime_data_wrapper_get_encoding")]
    fn get_encoding(&self) -> ContentEncoding;

    #[doc(alias = "g_mime_data_wrapper_get_stream")]
    fn get_stream(&self) -> Option<Stream>;

    #[doc(alias = "g_mime_data_wrapper_set_encoding")]
    fn set_encoding(&self, encoding: ContentEncoding);

    #[doc(alias = "g_mime_data_wrapper_set_stream")]
    fn set_stream<P: IsA<Stream>>(&self, stream: &P);

    #[doc(alias = "g_mime_data_wrapper_write_to_stream")]
    fn write_to_stream<P: IsA<Stream>>(&self, stream: &P) -> isize;
}

impl<O: IsA<DataWrapper>> DataWrapperExt for O {
    fn get_encoding(&self) -> ContentEncoding {
        unsafe {
            from_glib(ffi::g_mime_data_wrapper_get_encoding(self.as_ref().to_glib_none().0))
        }
    }

    fn get_stream(&self) -> Option<Stream> {
        unsafe {
            from_glib_none(ffi::g_mime_data_wrapper_get_stream(self.as_ref().to_glib_none().0))
        }
    }

    fn set_encoding(&self, encoding: ContentEncoding) {
        unsafe {
            ffi::g_mime_data_wrapper_set_encoding(self.as_ref().to_glib_none().0, encoding.to_glib());
        }
    }

    fn set_stream<P: IsA<Stream>>(&self, stream: &P) {
        unsafe {
            ffi::g_mime_data_wrapper_set_stream(self.as_ref().to_glib_none().0, stream.as_ref().to_glib_none().0);
        }
    }

    fn write_to_stream<P: IsA<Stream>>(&self, stream: &P) -> isize {
        unsafe {
            ffi::g_mime_data_wrapper_write_to_stream(self.as_ref().to_glib_none().0, stream.as_ref().to_glib_none().0)
        }
    }
}

impl fmt::Display for DataWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DataWrapper")
    }
}
