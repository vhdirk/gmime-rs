// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::Filter;
use crate::NewLineFormat;
use crate::ParamEncodingMethod;
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FormatOptions(Boxed<ffi::GMimeFormatOptions>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::g_mime_format_options_get_type(), ptr as *mut _) as *mut ffi::GMimeFormatOptions,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::g_mime_format_options_get_type(), ptr as *mut _),
        type_ => || ffi::g_mime_format_options_get_type(),
    }
}

impl FormatOptions {
    #[doc(alias = "g_mime_format_options_new")]
    pub fn new() -> FormatOptions {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::g_mime_format_options_new()) }
    }

    #[doc(alias = "g_mime_format_options_add_hidden_header")]
    pub fn add_hidden_header(&mut self, header: &str) {
        unsafe {
            ffi::g_mime_format_options_add_hidden_header(
                self.to_glib_none_mut().0,
                header.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_mime_format_options_clear_hidden_headers")]
    pub fn clear_hidden_headers(&mut self) {
        unsafe {
            ffi::g_mime_format_options_clear_hidden_headers(self.to_glib_none_mut().0);
        }
    }

    #[doc(alias = "g_mime_format_options_clone")]
    pub fn clone(&mut self) -> Option<FormatOptions> {
        unsafe { from_glib_full(ffi::g_mime_format_options_clone(self.to_glib_none_mut().0)) }
    }

    #[doc(alias = "g_mime_format_options_create_newline_filter")]
    pub fn create_newline_filter(&mut self, ensure_newline: bool) -> Option<Filter> {
        unsafe {
            from_glib_full(ffi::g_mime_format_options_create_newline_filter(
                self.to_glib_none_mut().0,
                ensure_newline.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_mime_format_options_get_newline")]
    #[doc(alias = "get_newline")]
    pub fn newline(&mut self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_mime_format_options_get_newline(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "g_mime_format_options_get_newline_format")]
    #[doc(alias = "get_newline_format")]
    pub fn newline_format(&mut self) -> NewLineFormat {
        unsafe {
            from_glib(ffi::g_mime_format_options_get_newline_format(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "g_mime_format_options_get_param_encoding_method")]
    #[doc(alias = "get_param_encoding_method")]
    pub fn param_encoding_method(&mut self) -> ParamEncodingMethod {
        unsafe {
            from_glib(ffi::g_mime_format_options_get_param_encoding_method(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "g_mime_format_options_is_hidden_header")]
    pub fn is_hidden_header(&mut self, header: &str) -> bool {
        unsafe {
            from_glib(ffi::g_mime_format_options_is_hidden_header(
                self.to_glib_none_mut().0,
                header.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_mime_format_options_remove_hidden_header")]
    pub fn remove_hidden_header(&mut self, header: &str) {
        unsafe {
            ffi::g_mime_format_options_remove_hidden_header(
                self.to_glib_none_mut().0,
                header.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_mime_format_options_set_newline_format")]
    pub fn set_newline_format(&mut self, newline: NewLineFormat) {
        unsafe {
            ffi::g_mime_format_options_set_newline_format(
                self.to_glib_none_mut().0,
                newline.into_glib(),
            );
        }
    }

    #[doc(alias = "g_mime_format_options_set_param_encoding_method")]
    pub fn set_param_encoding_method(&mut self, method: ParamEncodingMethod) {
        unsafe {
            ffi::g_mime_format_options_set_param_encoding_method(
                self.to_glib_none_mut().0,
                method.into_glib(),
            );
        }
    }

    #[doc(alias = "g_mime_format_options_get_default")]
    #[doc(alias = "get_default")]
    pub fn default() -> Option<FormatOptions> {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::g_mime_format_options_get_default()) }
    }
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self::new()
    }
}
