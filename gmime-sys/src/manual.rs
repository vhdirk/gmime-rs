#[allow(unused_imports)]
use libc::{c_int, c_ushort, c_void};

pub type _GMimeFilterPrivate = c_void;
pub type _GMimeFilterGZipPrivate = c_void;
pub type _GMimeParserPrivate = c_void;
pub type _GMimeStreamFilterPrivate = c_void;
pub type _UrlScanner = c_void;
pub type _cat_node = c_void;

pub type iconv_t = *mut c_void;
