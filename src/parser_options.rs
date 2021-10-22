use crate::ParserWarning;
use crate::RfcComplianceMode;
use glib::translate::*;
use std::boxed::Box as Box_;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ParserOptions(Boxed<ffi::GMimeParserOptions>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::g_mime_parser_options_get_type(), ptr as *mut _) as *mut ffi::GMimeParserOptions,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::g_mime_parser_options_get_type(), ptr as *mut _),
        type_ => || ffi::g_mime_parser_options_get_type(),
    }
}

impl ParserOptions {
    #[doc(alias = "g_mime_parser_options_new")]
    pub fn new() -> ParserOptions {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::g_mime_parser_options_new()) }
    }

    #[doc(alias = "g_mime_parser_options_clone")]
    pub fn clone(&mut self) -> Option<ParserOptions> {
        unsafe { from_glib_full(ffi::g_mime_parser_options_clone(self.to_glib_none_mut().0)) }
    }

    #[doc(alias = "g_mime_parser_options_get_address_compliance_mode")]
    #[doc(alias = "get_address_compliance_mode")]
    pub fn address_compliance_mode(&mut self) -> RfcComplianceMode {
        unsafe {
            from_glib(ffi::g_mime_parser_options_get_address_compliance_mode(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "g_mime_parser_options_get_allow_addresses_without_domain")]
    #[doc(alias = "get_allow_addresses_without_domain")]
    pub fn allows_addresses_without_domain(&mut self) -> bool {
        unsafe {
            from_glib(
                ffi::g_mime_parser_options_get_allow_addresses_without_domain(
                    self.to_glib_none_mut().0,
                ),
            )
        }
    }

    #[doc(alias = "g_mime_parser_options_get_fallback_charsets")]
    #[doc(alias = "get_fallback_charsets")]
    pub fn fallback_charsets(&mut self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::g_mime_parser_options_get_fallback_charsets(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "g_mime_parser_options_get_parameter_compliance_mode")]
    #[doc(alias = "get_parameter_compliance_mode")]
    pub fn parameter_compliance_mode(&mut self) -> RfcComplianceMode {
        unsafe {
            from_glib(ffi::g_mime_parser_options_get_parameter_compliance_mode(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "g_mime_parser_options_get_rfc2047_compliance_mode")]
    #[doc(alias = "get_rfc2047_compliance_mode")]
    pub fn rfc2047_compliance_mode(&mut self) -> RfcComplianceMode {
        unsafe {
            from_glib(ffi::g_mime_parser_options_get_rfc2047_compliance_mode(
                self.to_glib_none_mut().0,
            ))
        }
    }

    //#[doc(alias = "g_mime_parser_options_get_warning_callback")]
    //#[doc(alias = "get_warning_callback")]
    //pub fn warning_callback(&mut self) -> Option<Box_<dyn Fn(i64, &ParserWarning, &str) + 'static>> {
    //    unsafe { TODO: call ffi:g_mime_parser_options_get_warning_callback() }
    //}

    #[doc(alias = "g_mime_parser_options_set_address_compliance_mode")]
    pub fn set_address_compliance_mode(&mut self, mode: RfcComplianceMode) {
        unsafe {
            ffi::g_mime_parser_options_set_address_compliance_mode(
                self.to_glib_none_mut().0,
                mode.into_glib(),
            );
        }
    }

    #[doc(alias = "g_mime_parser_options_set_allow_addresses_without_domain")]
    pub fn set_allow_addresses_without_domain(&mut self, allow: bool) {
        unsafe {
            ffi::g_mime_parser_options_set_allow_addresses_without_domain(
                self.to_glib_none_mut().0,
                allow.into_glib(),
            );
        }
    }

    #[doc(alias = "g_mime_parser_options_set_fallback_charsets")]
    pub fn set_fallback_charsets(&mut self, charsets: &[&str]) {
        unsafe {
            ffi::g_mime_parser_options_set_fallback_charsets(
                self.to_glib_none_mut().0,
                charsets.to_glib_none().0,
            );
        }
    }
    #[doc(alias = "g_mime_parser_options_set_parameter_compliance_mode")]
    pub fn set_parameter_compliance_mode(&mut self, mode: RfcComplianceMode) {
        unsafe {
            ffi::g_mime_parser_options_set_parameter_compliance_mode(
                self.to_glib_none_mut().0,
                mode.into_glib(),
            );
        }
    }

    #[doc(alias = "g_mime_parser_options_set_rfc2047_compliance_mode")]
    pub fn set_rfc2047_compliance_mode(&mut self, mode: RfcComplianceMode) {
        unsafe {
            ffi::g_mime_parser_options_set_rfc2047_compliance_mode(
                self.to_glib_none_mut().0,
                mode.into_glib(),
            );
        }
    }

    #[doc(alias = "g_mime_parser_options_set_warning_callback")]
    pub fn set_warning_callback<P: Fn(i64, &ParserWarning, &str) + 'static>(
        &mut self,
        warning_cb: P,
    ) {
        let warning_cb_data: Box_<P> = Box_::new(warning_cb);
        unsafe extern "C" fn warning_cb_func<P: Fn(i64, &ParserWarning, &str) + 'static>(
            offset: i64,
            errcode: ffi::GMimeParserWarning,
            item: *const libc::c_char,
            user_data: glib::ffi::gpointer,
        ) {
            let errcode = from_glib(errcode);
            let item: Borrowed<glib::GString> = from_glib_borrow(item);
            let callback: &P = &*(user_data as *mut _);
            (*callback)(offset, &errcode, item.as_str());
        }
        let warning_cb = Some(warning_cb_func::<P> as _);
        let super_callback0: Box_<P> = warning_cb_data;
        unsafe {
            ffi::g_mime_parser_options_set_warning_callback(
                self.to_glib_none_mut().0,
                warning_cb,
                Box_::into_raw(super_callback0) as *mut _,
            );
        }
    }

    #[doc(alias = "g_mime_parser_options_get_default")]
    #[doc(alias = "get_default")]
    pub fn default() -> Option<ParserOptions> {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::g_mime_parser_options_get_default()) }
    }
}

impl Default for ParserOptions {
    fn default() -> Self {
        Self::new()
    }
}
