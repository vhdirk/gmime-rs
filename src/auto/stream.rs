// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::SeekWhence;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GMimeStream")]
    pub struct Stream(Object<ffi::GMimeStream, ffi::GMimeStreamClass>);

    match fn {
        type_ => || ffi::g_mime_stream_get_type(),
    }
}

pub const NONE_STREAM: Option<&Stream> = None;

pub trait StreamExt: 'static {
    #[doc(alias = "g_mime_stream_buffer_gets")]
    fn buffer_gets(&self, buf: &str, max: usize) -> isize;

    #[doc(alias = "g_mime_stream_buffer_readln")]
    fn buffer_readln(&self, buffer: &glib::ByteArray);

    #[doc(alias = "g_mime_stream_close")]
    fn close(&self) -> i32;

    #[doc(alias = "g_mime_stream_construct")]
    fn construct(&self, start: i64, end: i64);

    #[doc(alias = "g_mime_stream_eos")]
    fn eos(&self) -> bool;

    #[doc(alias = "g_mime_stream_flush")]
    fn flush(&self) -> i32;

    #[doc(alias = "g_mime_stream_length")]
    fn length(&self) -> i64;

    //#[doc(alias = "g_mime_stream_printf")]
    //fn printf(&self, fmt: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> isize;

    #[doc(alias = "g_mime_stream_read")]
    fn read(&self, buf: &[u8]) -> isize;

    #[doc(alias = "g_mime_stream_reset")]
    fn reset(&self) -> i32;

    #[doc(alias = "g_mime_stream_seek")]
    fn seek(&self, offset: i64, whence: SeekWhence) -> i64;

    #[doc(alias = "g_mime_stream_set_bounds")]
    fn set_bounds(&self, start: i64, end: i64);

    #[doc(alias = "g_mime_stream_substream")]
    fn substream(&self, start: i64, end: i64) -> Option<Stream>;

    #[doc(alias = "g_mime_stream_tell")]
    fn tell(&self) -> i64;

    #[doc(alias = "g_mime_stream_write")]
    fn write(&self, buf: &str) -> isize;

    #[doc(alias = "g_mime_stream_write_string")]
    fn write_string(&self, str: &str) -> isize;

    #[doc(alias = "g_mime_stream_write_to_stream")]
    fn write_to_stream(&self, dest: &impl IsA<Stream>) -> i64;

    //#[doc(alias = "g_mime_stream_writev")]
    //fn writev(&self, vector: /*Ignored*/&mut StreamIOVector, count: usize) -> i64;
}

impl<O: IsA<Stream>> StreamExt for O {
    fn buffer_gets(&self, buf: &str, max: usize) -> isize {
        unsafe {
            ffi::g_mime_stream_buffer_gets(
                self.as_ref().to_glib_none().0,
                buf.to_glib_none().0,
                max,
            )
        }
    }

    fn buffer_readln(&self, buffer: &glib::ByteArray) {
        unsafe {
            ffi::g_mime_stream_buffer_readln(
                self.as_ref().to_glib_none().0,
                buffer.to_glib_none().0,
            );
        }
    }

    fn close(&self) -> i32 {
        unsafe { ffi::g_mime_stream_close(self.as_ref().to_glib_none().0) }
    }

    fn construct(&self, start: i64, end: i64) {
        unsafe {
            ffi::g_mime_stream_construct(self.as_ref().to_glib_none().0, start, end);
        }
    }

    fn eos(&self) -> bool {
        unsafe { from_glib(ffi::g_mime_stream_eos(self.as_ref().to_glib_none().0)) }
    }

    fn flush(&self) -> i32 {
        unsafe { ffi::g_mime_stream_flush(self.as_ref().to_glib_none().0) }
    }

    fn length(&self) -> i64 {
        unsafe { ffi::g_mime_stream_length(self.as_ref().to_glib_none().0) }
    }

    //fn printf(&self, fmt: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> isize {
    //    unsafe { TODO: call ffi:g_mime_stream_printf() }
    //}

    fn read(&self, buf: &[u8]) -> isize {
        let len = buf.len() as usize;
        unsafe {
            ffi::g_mime_stream_read(self.as_ref().to_glib_none().0, buf.to_glib_none().0, len)
        }
    }

    fn reset(&self) -> i32 {
        unsafe { ffi::g_mime_stream_reset(self.as_ref().to_glib_none().0) }
    }

    fn seek(&self, offset: i64, whence: SeekWhence) -> i64 {
        unsafe {
            ffi::g_mime_stream_seek(self.as_ref().to_glib_none().0, offset, whence.into_glib())
        }
    }

    fn set_bounds(&self, start: i64, end: i64) {
        unsafe {
            ffi::g_mime_stream_set_bounds(self.as_ref().to_glib_none().0, start, end);
        }
    }

    fn substream(&self, start: i64, end: i64) -> Option<Stream> {
        unsafe {
            from_glib_full(ffi::g_mime_stream_substream(
                self.as_ref().to_glib_none().0,
                start,
                end,
            ))
        }
    }

    fn tell(&self) -> i64 {
        unsafe { ffi::g_mime_stream_tell(self.as_ref().to_glib_none().0) }
    }

    fn write(&self, buf: &str) -> isize {
        let len = buf.len() as usize;
        unsafe {
            ffi::g_mime_stream_write(self.as_ref().to_glib_none().0, buf.to_glib_none().0, len)
        }
    }

    fn write_string(&self, str: &str) -> isize {
        unsafe {
            ffi::g_mime_stream_write_string(self.as_ref().to_glib_none().0, str.to_glib_none().0)
        }
    }

    fn write_to_stream(&self, dest: &impl IsA<Stream>) -> i64 {
        unsafe {
            ffi::g_mime_stream_write_to_stream(
                self.as_ref().to_glib_none().0,
                dest.as_ref().to_glib_none().0,
            )
        }
    }

    //fn writev(&self, vector: /*Ignored*/&mut StreamIOVector, count: usize) -> i64 {
    //    unsafe { TODO: call ffi:g_mime_stream_writev() }
    //}
}

impl fmt::Display for Stream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Stream")
    }
}
