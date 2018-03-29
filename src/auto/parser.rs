// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff+)
// from gir-files (https://github.com/gtk-rs/gir-files @ 33386b3)
// DO NOT EDIT

use Format;
use Message;
use Object;
use ParserOptions;
use Stream;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Parser(Object<ffi::GMimeParser, ffi::GMimeParserClass>);

    match fn {
        get_type => || ffi::g_mime_parser_get_type(),
    }
}

impl Parser {
    pub fn new() -> Parser {
        unsafe {
            from_glib_full(ffi::g_mime_parser_new())
        }
    }

    pub fn new_with_stream<P: IsA<Stream>>(stream: &P) -> Parser {
        unsafe {
            from_glib_full(ffi::g_mime_parser_new_with_stream(stream.to_glib_none().0))
        }
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ParserExt {
    fn construct_message<'a, P: Into<Option<&'a ParserOptions>>>(&self, options: P) -> Option<Message>;

    fn construct_part<'a, P: Into<Option<&'a ParserOptions>>>(&self, options: P) -> Option<Object>;

    fn eos(&self) -> bool;

    fn get_format(&self) -> Format;

    fn get_headers_begin(&self) -> i64;

    fn get_headers_end(&self) -> i64;

    fn get_mbox_marker(&self) -> Option<String>;

    fn get_mbox_marker_offset(&self) -> i64;

    fn get_persist_stream(&self) -> bool;

    fn get_respect_content_length(&self) -> bool;

    fn init_with_stream<P: IsA<Stream>>(&self, stream: &P);

    fn set_format(&self, format: Format);

    //fn set_header_regex<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, regex: &str, header_cb: /*Unknown conversion*//*Unimplemented*/ParserHeaderRegexFunc, user_data: P);

    fn set_persist_stream(&self, persist: bool);

    fn set_respect_content_length(&self, respect_content_length: bool);

    fn tell(&self) -> i64;
}

impl<O: IsA<Parser>> ParserExt for O {
    fn construct_message<'a, P: Into<Option<&'a ParserOptions>>>(&self, options: P) -> Option<Message> {
        let options = options.into();
        unsafe {
            from_glib_full(ffi::g_mime_parser_construct_message(self.to_glib_none().0, options.to_glib_none_mut().0))
        }
    }

    fn construct_part<'a, P: Into<Option<&'a ParserOptions>>>(&self, options: P) -> Option<Object> {
        let options = options.into();
        unsafe {
            from_glib_full(ffi::g_mime_parser_construct_part(self.to_glib_none().0, options.to_glib_none_mut().0))
        }
    }

    fn eos(&self) -> bool {
        unsafe {
            from_glib(ffi::g_mime_parser_eos(self.to_glib_none().0))
        }
    }

    fn get_format(&self) -> Format {
        unsafe {
            from_glib(ffi::g_mime_parser_get_format(self.to_glib_none().0))
        }
    }

    fn get_headers_begin(&self) -> i64 {
        unsafe {
            ffi::g_mime_parser_get_headers_begin(self.to_glib_none().0)
        }
    }

    fn get_headers_end(&self) -> i64 {
        unsafe {
            ffi::g_mime_parser_get_headers_end(self.to_glib_none().0)
        }
    }

    fn get_mbox_marker(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::g_mime_parser_get_mbox_marker(self.to_glib_none().0))
        }
    }

    fn get_mbox_marker_offset(&self) -> i64 {
        unsafe {
            ffi::g_mime_parser_get_mbox_marker_offset(self.to_glib_none().0)
        }
    }

    fn get_persist_stream(&self) -> bool {
        unsafe {
            from_glib(ffi::g_mime_parser_get_persist_stream(self.to_glib_none().0))
        }
    }

    fn get_respect_content_length(&self) -> bool {
        unsafe {
            from_glib(ffi::g_mime_parser_get_respect_content_length(self.to_glib_none().0))
        }
    }

    fn init_with_stream<P: IsA<Stream>>(&self, stream: &P) {
        unsafe {
            ffi::g_mime_parser_init_with_stream(self.to_glib_none().0, stream.to_glib_none().0);
        }
    }

    fn set_format(&self, format: Format) {
        unsafe {
            ffi::g_mime_parser_set_format(self.to_glib_none().0, format.to_glib());
        }
    }

    //fn set_header_regex<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, regex: &str, header_cb: /*Unknown conversion*//*Unimplemented*/ParserHeaderRegexFunc, user_data: P) {
    //    unsafe { TODO: call ffi::g_mime_parser_set_header_regex() }
    //}

    fn set_persist_stream(&self, persist: bool) {
        unsafe {
            ffi::g_mime_parser_set_persist_stream(self.to_glib_none().0, persist.to_glib());
        }
    }

    fn set_respect_content_length(&self, respect_content_length: bool) {
        unsafe {
            ffi::g_mime_parser_set_respect_content_length(self.to_glib_none().0, respect_content_length.to_glib());
        }
    }

    fn tell(&self) -> i64 {
        unsafe {
            ffi::g_mime_parser_tell(self.to_glib_none().0)
        }
    }
}
