// This file was generated by gir (https://github.com/gtk-rs/gir @ 9e3cb65)
// from gir-files (https://github.com/gtk-rs/gir-files @ b215ee8+)
// DO NOT EDIT

use Stream;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use gmime_sys;
use std::fmt;

glib_wrapper! {
    pub struct StreamMem(Object<gmime_sys::GMimeStreamMem, gmime_sys::GMimeStreamMemClass, StreamMemClass>) @extends Stream;

    match fn {
        get_type => || gmime_sys::g_mime_stream_mem_get_type(),
    }
}

impl StreamMem {
    pub fn new() -> StreamMem {
        unsafe {
            Stream::from_glib_full(gmime_sys::g_mime_stream_mem_new()).unsafe_cast()
        }
    }

    pub fn new_with_buffer(buffer: &[u8]) -> StreamMem {
        let len = buffer.len() as usize;
        unsafe {
            Stream::from_glib_full(gmime_sys::g_mime_stream_mem_new_with_buffer(buffer.to_glib_none().0, len)).unsafe_cast()
        }
    }

    //pub fn new_with_byte_array(array: /*Ignored*/&glib::ByteArray) -> StreamMem {
    //    unsafe { TODO: call gmime_sys:g_mime_stream_mem_new_with_byte_array() }
    //}
}

impl Default for StreamMem {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_STREAM_MEM: Option<&StreamMem> = None;

pub trait StreamMemExt: 'static {
    //fn get_byte_array(&self) -> /*Ignored*/Option<glib::ByteArray>;

    fn get_owner(&self) -> bool;

    //fn set_byte_array(&self, array: /*Ignored*/&glib::ByteArray);

    fn set_owner(&self, owner: bool);
}

impl<O: IsA<StreamMem>> StreamMemExt for O {
    //fn get_byte_array(&self) -> /*Ignored*/Option<glib::ByteArray> {
    //    unsafe { TODO: call gmime_sys:g_mime_stream_mem_get_byte_array() }
    //}

    fn get_owner(&self) -> bool {
        unsafe {
            from_glib(gmime_sys::g_mime_stream_mem_get_owner(self.as_ref().to_glib_none().0))
        }
    }

    //fn set_byte_array(&self, array: /*Ignored*/&glib::ByteArray) {
    //    unsafe { TODO: call gmime_sys:g_mime_stream_mem_set_byte_array() }
    //}

    fn set_owner(&self, owner: bool) {
        unsafe {
            gmime_sys::g_mime_stream_mem_set_owner(self.as_ref().to_glib_none().0, owner.to_glib());
        }
    }
}

impl fmt::Display for StreamMem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StreamMem")
    }
}
