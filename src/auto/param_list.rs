// This file was generated by gir (https://github.com/gtk-rs/gir @ 00040a2)
// from gir-files (https://github.com/gtk-rs/gir-files @ 5b96546)
// DO NOT EDIT

use crate::Param;
use crate::ParserOptions;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct ParamList(Object<ffi::GMimeParamList, ffi::GMimeParamListClass>);

    match fn {
        get_type => || ffi::g_mime_param_list_get_type(),
    }
}

impl ParamList {
    #[doc(alias = "g_mime_param_list_new")]
    pub fn new() -> ParamList {
        unsafe {
            from_glib_full(ffi::g_mime_param_list_new())
        }
    }

    #[doc(alias = "g_mime_param_list_parse")]
    pub fn parse(options: &mut ParserOptions, str: &str) -> Option<ParamList> {
        unsafe {
            from_glib_full(ffi::g_mime_param_list_parse(options.to_glib_none_mut().0, str.to_glib_none().0))
        }
    }
}

impl Default for ParamList {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_PARAM_LIST: Option<&ParamList> = None;

pub trait ParamListExt: 'static {
    #[doc(alias = "g_mime_param_list_clear")]
    fn clear(&self);

    //#[doc(alias = "g_mime_param_list_encode")]
    //fn encode(&self, options: &mut FormatOptions, fold: bool, str: /*Ignored*/&mut glib::String);

    #[doc(alias = "g_mime_param_list_get_parameter")]
    fn get_parameter(&self, name: &str) -> Option<Param>;

    #[doc(alias = "g_mime_param_list_get_parameter_at")]
    fn get_parameter_at(&self, index: i32) -> Option<Param>;

    #[doc(alias = "g_mime_param_list_length")]
    fn length(&self) -> i32;

    #[doc(alias = "g_mime_param_list_remove")]
    fn remove(&self, name: &str) -> bool;

    #[doc(alias = "g_mime_param_list_remove_at")]
    fn remove_at(&self, index: i32) -> bool;

    #[doc(alias = "g_mime_param_list_set_parameter")]
    fn set_parameter(&self, name: &str, value: &str);
}

impl<O: IsA<ParamList>> ParamListExt for O {
    fn clear(&self) {
        unsafe {
            ffi::g_mime_param_list_clear(self.as_ref().to_glib_none().0);
        }
    }

    //fn encode(&self, options: &mut FormatOptions, fold: bool, str: /*Ignored*/&mut glib::String) {
    //    unsafe { TODO: call ffi:g_mime_param_list_encode() }
    //}

    fn get_parameter(&self, name: &str) -> Option<Param> {
        unsafe {
            from_glib_none(ffi::g_mime_param_list_get_parameter(self.as_ref().to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn get_parameter_at(&self, index: i32) -> Option<Param> {
        unsafe {
            from_glib_none(ffi::g_mime_param_list_get_parameter_at(self.as_ref().to_glib_none().0, index))
        }
    }

    fn length(&self) -> i32 {
        unsafe {
            ffi::g_mime_param_list_length(self.as_ref().to_glib_none().0)
        }
    }

    fn remove(&self, name: &str) -> bool {
        unsafe {
            from_glib(ffi::g_mime_param_list_remove(self.as_ref().to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn remove_at(&self, index: i32) -> bool {
        unsafe {
            from_glib(ffi::g_mime_param_list_remove_at(self.as_ref().to_glib_none().0, index))
        }
    }

    fn set_parameter(&self, name: &str, value: &str) {
        unsafe {
            ffi::g_mime_param_list_set_parameter(self.as_ref().to_glib_none().0, name.to_glib_none().0, value.to_glib_none().0);
        }
    }
}

impl fmt::Display for ParamList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ParamList")
    }
}
