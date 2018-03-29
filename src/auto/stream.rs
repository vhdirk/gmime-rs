// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff+)
// from gir-files (https://github.com/gtk-rs/gir-files @ 33386b3)
// DO NOT EDIT

use SeekWhence;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Stream(Object<ffi::GMimeStream, ffi::GMimeStreamClass>);

    match fn {
        get_type => || ffi::g_mime_stream_get_type(),
    }
}

pub trait StreamExt {
    fn buffer_gets(&self, buf: &str, max: usize) -> isize;

    //fn buffer_readln(&self, buffer: /*Ignored*/&glib::ByteArray);

    fn close(&self) -> i32;

    fn construct(&self, start: i64, end: i64);

    fn eos(&self) -> bool;

    fn flush(&self) -> i32;

    fn length(&self) -> i64;

    //fn printf(&self, fmt: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> isize;

    fn read(&self, buf: &[u8]) -> isize;

    fn reset(&self) -> i32;

    fn seek(&self, offset: i64, whence: SeekWhence) -> i64;

    fn set_bounds(&self, start: i64, end: i64);

    fn substream(&self, start: i64, end: i64) -> Option<Stream>;

    fn tell(&self) -> i64;

    fn write(&self, buf: &str) -> isize;

    fn write_string(&self, str: &str) -> isize;

    fn write_to_stream<P: IsA<Stream>>(&self, dest: &P) -> i64;

    //fn writev(&self, vector: /*Ignored*/&mut StreamIOVector, count: usize) -> i64;
}

impl<O: IsA<Stream>> StreamExt for O {
    fn buffer_gets(&self, buf: &str, max: usize) -> isize {
        unsafe {
            ffi::g_mime_stream_buffer_gets(self.to_glib_none().0, buf.to_glib_none().0, max)
        }
    }

    //fn buffer_readln(&self, buffer: /*Ignored*/&glib::ByteArray) {
    //    unsafe { TODO: call ffi::g_mime_stream_buffer_readln() }
    //}

    fn close(&self) -> i32 {
        unsafe {
            ffi::g_mime_stream_close(self.to_glib_none().0)
        }
    }

    fn construct(&self, start: i64, end: i64) {
        unsafe {
            ffi::g_mime_stream_construct(self.to_glib_none().0, start, end);
        }
    }

    fn eos(&self) -> bool {
        unsafe {
            from_glib(ffi::g_mime_stream_eos(self.to_glib_none().0))
        }
    }

    fn flush(&self) -> i32 {
        unsafe {
            ffi::g_mime_stream_flush(self.to_glib_none().0)
        }
    }

    fn length(&self) -> i64 {
        unsafe {
            ffi::g_mime_stream_length(self.to_glib_none().0)
        }
    }

    //fn printf(&self, fmt: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> isize {
    //    unsafe { TODO: call ffi::g_mime_stream_printf() }
    //}

    fn read(&self, buf: &[u8]) -> isize {
        let len = buf.len() as usize;
        unsafe {
            ffi::g_mime_stream_read(self.to_glib_none().0, buf.to_glib_none().0, len)
        }
    }

    fn reset(&self) -> i32 {
        unsafe {
            ffi::g_mime_stream_reset(self.to_glib_none().0)
        }
    }

    fn seek(&self, offset: i64, whence: SeekWhence) -> i64 {
        unsafe {
            ffi::g_mime_stream_seek(self.to_glib_none().0, offset, whence.to_glib())
        }
    }

    fn set_bounds(&self, start: i64, end: i64) {
        unsafe {
            ffi::g_mime_stream_set_bounds(self.to_glib_none().0, start, end);
        }
    }

    fn substream(&self, start: i64, end: i64) -> Option<Stream> {
        unsafe {
            from_glib_full(ffi::g_mime_stream_substream(self.to_glib_none().0, start, end))
        }
    }

    fn tell(&self) -> i64 {
        unsafe {
            ffi::g_mime_stream_tell(self.to_glib_none().0)
        }
    }

    fn write(&self, buf: &str) -> isize {
        let len = buf.len() as usize;
        unsafe {
            ffi::g_mime_stream_write(self.to_glib_none().0, buf.to_glib_none().0, len)
        }
    }

    fn write_string(&self, str: &str) -> isize {
        unsafe {
            ffi::g_mime_stream_write_string(self.to_glib_none().0, str.to_glib_none().0)
        }
    }

    fn write_to_stream<P: IsA<Stream>>(&self, dest: &P) -> i64 {
        unsafe {
            ffi::g_mime_stream_write_to_stream(self.to_glib_none().0, dest.to_glib_none().0)
        }
    }

    //fn writev(&self, vector: /*Ignored*/&mut StreamIOVector, count: usize) -> i64 {
    //    unsafe { TODO: call ffi::g_mime_stream_writev() }
    //}
}
