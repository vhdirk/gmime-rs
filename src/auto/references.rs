// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::ParserOptions;
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct References(Boxed<ffi::GMimeReferences>);

    match fn {
        copy => |ptr| ffi::g_mime_references_copy(mut_override(ptr)),
        free => |ptr| ffi::g_mime_references_free(ptr),
        type_ => || ffi::g_mime_references_get_type(),
    }
}

impl References {
    #[doc(alias = "g_mime_references_new")]
    pub fn new() -> References {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::g_mime_references_new()) }
    }

    #[doc(alias = "g_mime_references_append")]
    pub fn append(&mut self, msgid: &str) {
        unsafe {
            ffi::g_mime_references_append(self.to_glib_none_mut().0, msgid.to_glib_none().0);
        }
    }

    #[doc(alias = "g_mime_references_clear")]
    pub fn clear(&mut self) {
        unsafe {
            ffi::g_mime_references_clear(self.to_glib_none_mut().0);
        }
    }

    #[doc(alias = "g_mime_references_get_message_id")]
    #[doc(alias = "get_message_id")]
    pub fn message_id(&mut self, index: i32) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_mime_references_get_message_id(
                self.to_glib_none_mut().0,
                index,
            ))
        }
    }

    #[doc(alias = "g_mime_references_length")]
    pub fn length(&mut self) -> i32 {
        unsafe { ffi::g_mime_references_length(self.to_glib_none_mut().0) }
    }

    #[doc(alias = "g_mime_references_set_message_id")]
    pub fn set_message_id(&mut self, index: i32, msgid: &str) {
        unsafe {
            ffi::g_mime_references_set_message_id(
                self.to_glib_none_mut().0,
                index,
                msgid.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_mime_references_parse")]
    pub fn parse(options: Option<&ParserOptions>, text: &str) -> Option<References> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::g_mime_references_parse(
                mut_override(options.to_glib_none().0),
                text.to_glib_none().0,
            ))
        }
    }
}

impl Default for References {
    fn default() -> Self {
        Self::new()
    }
}
