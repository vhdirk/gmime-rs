#![allow(deprecated)]

extern crate gmime_sys as ffi;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gio_sys as gio_ffi;

#[macro_use]
extern crate glib;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;

extern crate libc;


pub use glib::Error;

pub use gio::File;


#[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
#[cfg_attr(feature = "cargo-clippy", allow(useless_transmute))]
mod auto;
pub use auto::*;
