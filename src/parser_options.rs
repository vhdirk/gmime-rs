use RfcComplianceMode;
use gmime_sys;
use glib::translate::*;
use gobject_sys;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct ParserOptions(Boxed<gmime_sys::GMimeParserOptions>);

    match fn {
        copy => |ptr| gobject_sys::g_boxed_copy(gmime_sys::g_mime_parser_options_get_type(), ptr as *mut _) as *mut gmime_sys::GMimeParserOptions,
        free => |ptr| gobject_sys::g_boxed_free(gmime_sys::g_mime_parser_options_get_type(), ptr as *mut _),
        get_type => || gmime_sys::g_mime_parser_options_get_type(),
    }
}

impl ParserOptions {
    pub fn new() -> ParserOptions {
        unsafe {
            from_glib_full(gmime_sys::g_mime_parser_options_new())
        }
    }

    pub fn clone(&mut self) -> Option<ParserOptions> {
        unsafe {
            from_glib_full(gmime_sys::g_mime_parser_options_clone(self.to_glib_none_mut().0))
        }
    }

    pub fn get_address_compliance_mode(&mut self) -> RfcComplianceMode {
        unsafe {
            from_glib(gmime_sys::g_mime_parser_options_get_address_compliance_mode(self.to_glib_none_mut().0))
        }
    }

    pub fn get_allow_addresses_without_domain(&mut self) -> bool {
        unsafe {
            from_glib(gmime_sys::g_mime_parser_options_get_allow_addresses_without_domain(self.to_glib_none_mut().0))
        }
    }

    pub fn get_fallback_charsets(&mut self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(gmime_sys::g_mime_parser_options_get_fallback_charsets(self.to_glib_none_mut().0))
        }
    }

    pub fn get_parameter_compliance_mode(&mut self) -> RfcComplianceMode {
        unsafe {
            from_glib(gmime_sys::g_mime_parser_options_get_parameter_compliance_mode(self.to_glib_none_mut().0))
        }
    }

    pub fn get_rfc2047_compliance_mode(&mut self) -> RfcComplianceMode {
        unsafe {
            from_glib(gmime_sys::g_mime_parser_options_get_rfc2047_compliance_mode(self.to_glib_none_mut().0))
        }
    }

    //pub fn get_warning_callback(&mut self) -> /*Unknown conversion*//*Unimplemented*/ParserWarningFunc {
    //    unsafe { TODO: call gmime_sys::g_mime_parser_options_get_warning_callback() }
    //}

    pub fn set_address_compliance_mode(&mut self, mode: RfcComplianceMode) {
        unsafe {
            gmime_sys::g_mime_parser_options_set_address_compliance_mode(self.to_glib_none_mut().0, mode.to_glib());
        }
    }

    pub fn set_allow_addresses_without_domain(&mut self, allow: bool) {
        unsafe {
            gmime_sys::g_mime_parser_options_set_allow_addresses_without_domain(self.to_glib_none_mut().0, allow.to_glib());
        }
    }

    pub fn set_fallback_charsets(&mut self, charsets: &Vec<&str>) {
        unsafe {
            gmime_sys::g_mime_parser_options_set_fallback_charsets(self.to_glib_none_mut().0, charsets.to_glib_none().0);
        }
    }

    pub fn set_parameter_compliance_mode(&mut self, mode: RfcComplianceMode) {
        unsafe {
            gmime_sys::g_mime_parser_options_set_parameter_compliance_mode(self.to_glib_none_mut().0, mode.to_glib());
        }
    }

    pub fn set_rfc2047_compliance_mode(&mut self, mode: RfcComplianceMode) {
        unsafe {
            gmime_sys::g_mime_parser_options_set_rfc2047_compliance_mode(self.to_glib_none_mut().0, mode.to_glib());
        }
    }

    //pub fn set_warning_callback<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&mut self, warning_cb: /*Unknown conversion*//*Unimplemented*/ParserWarningFunc, user_data: P) {
    //    unsafe { TODO: call gmime_sys::g_mime_parser_options_set_warning_callback() }
    //}

    pub fn get_default() -> Option<ParserOptions> {
        unsafe {
            from_glib_full(gmime_sys::g_mime_parser_options_get_default())
        }
    }
}

impl Default for ParserOptions {
    fn default() -> Self {
        Self::new()
    }
}
