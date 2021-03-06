#![allow(deprecated)]

extern crate gmime_sys;
extern crate glib_sys;
extern crate gobject_sys;
extern crate gio_sys;

#[macro_use]
extern crate glib;
extern crate gio;
#[macro_use]
extern crate bitflags;

extern crate libc;


pub use glib::{
    Bytes,
    Error,
    Variant,
    VariantType,
    IOCondition,
    SeekType,
    Source,
    Priority,
    DateTime,
    ChecksumType
};

pub use gio::File;


#[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
#[cfg_attr(feature = "cargo-clippy", allow(useless_transmute))]
mod auto;
pub use auto::*;

mod enums;
pub use enums::*;

mod message;
pub use message::*;

mod message_partial;
pub use message_partial::*;

mod parser_options;
pub use parser_options::*;
