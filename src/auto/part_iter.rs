// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff+)
// from gir-files (https://github.com/gtk-rs/gir-files @ 33386b3)
// DO NOT EDIT

use Object;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct PartIter(Boxed<ffi::GMimePartIter>);

    match fn {
        copy => |ptr| gobject_ffi::g_boxed_copy(ffi::g_mime_part_iter_get_type(), ptr as *mut _) as *mut ffi::GMimePartIter,
        free => |ptr| gobject_ffi::g_boxed_free(ffi::g_mime_part_iter_get_type(), ptr as *mut _),
        get_type => || ffi::g_mime_part_iter_get_type(),
    }
}

impl PartIter {
    pub fn new<P: IsA<Object>>(toplevel: &P) -> PartIter {
        unsafe {
            from_glib_full(ffi::g_mime_part_iter_new(toplevel.to_glib_none().0))
        }
    }

    pub fn clone(&mut self) -> Option<PartIter> {
        unsafe {
            from_glib_full(ffi::g_mime_part_iter_clone(self.to_glib_none_mut().0))
        }
    }

    pub fn get_current(&mut self) -> Option<Object> {
        unsafe {
            from_glib_none(ffi::g_mime_part_iter_get_current(self.to_glib_none_mut().0))
        }
    }

    pub fn get_parent(&mut self) -> Option<Object> {
        unsafe {
            from_glib_none(ffi::g_mime_part_iter_get_parent(self.to_glib_none_mut().0))
        }
    }

    pub fn get_path(&mut self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::g_mime_part_iter_get_path(self.to_glib_none_mut().0))
        }
    }

    pub fn get_toplevel(&mut self) -> Option<Object> {
        unsafe {
            from_glib_none(ffi::g_mime_part_iter_get_toplevel(self.to_glib_none_mut().0))
        }
    }

    pub fn is_valid(&mut self) -> bool {
        unsafe {
            from_glib(ffi::g_mime_part_iter_is_valid(self.to_glib_none_mut().0))
        }
    }

    pub fn jump_to(&mut self, path: &str) -> bool {
        unsafe {
            from_glib(ffi::g_mime_part_iter_jump_to(self.to_glib_none_mut().0, path.to_glib_none().0))
        }
    }

    pub fn next(&mut self) -> bool {
        unsafe {
            from_glib(ffi::g_mime_part_iter_next(self.to_glib_none_mut().0))
        }
    }

    pub fn prev(&mut self) -> bool {
        unsafe {
            from_glib(ffi::g_mime_part_iter_prev(self.to_glib_none_mut().0))
        }
    }

    pub fn remove(&mut self) -> bool {
        unsafe {
            from_glib(ffi::g_mime_part_iter_remove(self.to_glib_none_mut().0))
        }
    }

    pub fn replace<P: IsA<Object>>(&mut self, replacement: &P) -> bool {
        unsafe {
            from_glib(ffi::g_mime_part_iter_replace(self.to_glib_none_mut().0, replacement.to_glib_none().0))
        }
    }

    pub fn reset(&mut self) {
        unsafe {
            ffi::g_mime_part_iter_reset(self.to_glib_none_mut().0);
        }
    }
}
