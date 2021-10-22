use crate::FilterChecksum;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use libc::size_t;

pub trait FilterChecksumExtManual: 'static {
    #[doc(alias = "g_mime_filter_checksum_get_digest")]
    #[doc(alias = "get_digest")]
    fn digest(&self) -> Vec<u8>;
}

impl<O: IsA<FilterChecksum>> FilterChecksumExtManual for O {
    fn digest(&self) -> Vec<u8> {
        unsafe {
            let digest_capacity: size_t = 512 / 8;
            let mut vec = Vec::with_capacity(digest_capacity as usize);

            let digest_len = ffi::g_mime_filter_checksum_get_digest(
                self.as_ref().to_glib_none().0,
                vec.as_mut_ptr(),
                digest_capacity,
            );

            vec.set_len(digest_len);
            vec
        }
    }
}
