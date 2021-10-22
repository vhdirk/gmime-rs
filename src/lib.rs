#![allow(deprecated)]
#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;

// Re-export gtk dependencies
pub use gio;
pub use glib;

pub use glib::{
    Bytes, ChecksumType, DateTime, Error, IOCondition, Priority, SeekType, Source, Variant,
    VariantType,
};

pub use gio::File;

#[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
#[cfg_attr(feature = "cargo-clippy", allow(useless_transmute))]
#[macro_use]
mod rt;

#[allow(unused_imports)]
mod auto;

mod crypto_context;
mod filter_checksum;
mod internet_address;
mod internet_address_group;
mod internet_address_list;
mod internet_address_mailbox;
mod message;
mod message_partial;
mod parser_options;

pub use crypto_context::*;
pub use filter_checksum::*;
pub use internet_address::*;
pub use internet_address_group::*;
pub use internet_address_list::*;
pub use internet_address_mailbox::*;
pub use message::*;
pub use message_partial::*;
pub use parser_options::*;

pub use crate::auto::*;

pub mod prelude;
