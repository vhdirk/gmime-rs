use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use std::fmt;

use crate::Message;
use crate::ParamList;

pub trait ParamListExtManual: 'static {
    #[doc(alias = "g_mime_param_list_encode")]
    fn encode(&self, options: &mut FormatOptions, fold: bool) -> String;
}

impl<O: IsA<ParamList>> ParamListExtManual for O {
    fn encode(&self, options: &mut FormatOptions, fold: bool) -> String {
        unsafe {
         let s = glib::ffi::g_string_new(ptr::null());
            ffi::g_mime_param_list_encode(
                self.as_ref().to_glib_none().0,
                options.to_glib_none_mut().0,
                fold.into_glib(),
                s,
            );
            from_glib_full(glib::ffi::g_string_free(s, glib::ffi::GFALSE))
        }
    }
}
