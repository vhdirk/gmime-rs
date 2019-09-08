use gmime_sys;
use glib::translate::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum DigestAlgo {
    AlgoDefault,
    AlgoMd5,
    AlgoSha1,
    AlgoRipemd160,
    AlgoMd2,
    AlgoTiger192,
    AlgoHaval5160,
    AlgoSha256,
    AlgoSha384,
    AlgoSha512,
    AlgoSha224,
    AlgoMd4,
    AlgoCrc32,
    AlgoCrc32Rfc1510,
    AlgoCrc32Rfc2440,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for DigestAlgo {
    type GlibType = gmime_sys::GMimeDigestAlgo;

    fn to_glib(&self) -> gmime_sys::GMimeDigestAlgo {
        match *self {
            DigestAlgo::AlgoDefault => gmime_sys::GMIME_DIGEST_ALGO_DEFAULT,
            DigestAlgo::AlgoMd5 => gmime_sys::GMIME_DIGEST_ALGO_MD5,
            DigestAlgo::AlgoSha1 => gmime_sys::GMIME_DIGEST_ALGO_SHA1,
            DigestAlgo::AlgoRipemd160 => gmime_sys::GMIME_DIGEST_ALGO_RIPEMD160,
            DigestAlgo::AlgoMd2 => gmime_sys::GMIME_DIGEST_ALGO_MD2,
            DigestAlgo::AlgoTiger192 => gmime_sys::GMIME_DIGEST_ALGO_TIGER192,
            DigestAlgo::AlgoHaval5160 => gmime_sys::GMIME_DIGEST_ALGO_HAVAL5160,
            DigestAlgo::AlgoSha256 => gmime_sys::GMIME_DIGEST_ALGO_SHA256,
            DigestAlgo::AlgoSha384 => gmime_sys::GMIME_DIGEST_ALGO_SHA384,
            DigestAlgo::AlgoSha512 => gmime_sys::GMIME_DIGEST_ALGO_SHA512,
            DigestAlgo::AlgoSha224 => gmime_sys::GMIME_DIGEST_ALGO_SHA224,
            DigestAlgo::AlgoMd4 => gmime_sys::GMIME_DIGEST_ALGO_MD4,
            DigestAlgo::AlgoCrc32 => gmime_sys::GMIME_DIGEST_ALGO_CRC32,
            DigestAlgo::AlgoCrc32Rfc1510 => gmime_sys::GMIME_DIGEST_ALGO_CRC32_RFC1510,
            DigestAlgo::AlgoCrc32Rfc2440 => gmime_sys::GMIME_DIGEST_ALGO_CRC32_RFC2440,
            DigestAlgo::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<gmime_sys::GMimeDigestAlgo> for DigestAlgo {
    fn from_glib(value: gmime_sys::GMimeDigestAlgo) -> Self {
        match value {
            0 => DigestAlgo::AlgoDefault,
            1 => DigestAlgo::AlgoMd5,
            2 => DigestAlgo::AlgoSha1,
            3 => DigestAlgo::AlgoRipemd160,
            5 => DigestAlgo::AlgoMd2,
            6 => DigestAlgo::AlgoTiger192,
            7 => DigestAlgo::AlgoHaval5160,
            8 => DigestAlgo::AlgoSha256,
            9 => DigestAlgo::AlgoSha384,
            10 => DigestAlgo::AlgoSha512,
            11 => DigestAlgo::AlgoSha224,
            301 => DigestAlgo::AlgoMd4,
            302 => DigestAlgo::AlgoCrc32,
            303 => DigestAlgo::AlgoCrc32Rfc1510,
            304 => DigestAlgo::AlgoCrc32Rfc2440,
            value => DigestAlgo::__Unknown(value),
        }
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ContentEncoding {
    EncodeDefault,
    Encode7bit,
    Encode8bit,
    EncodeBinary,
    EncodeBase64,
    EncodeQuotedprintable,
    EncodeUuencode,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for ContentEncoding {
    type GlibType = gmime_sys::GMimeContentEncoding;

    fn to_glib(&self) -> gmime_sys::GMimeContentEncoding {
        match *self {
            ContentEncoding::EncodeDefault => gmime_sys::GMIME_CONTENT_ENCODING_DEFAULT,
            ContentEncoding::Encode7bit => gmime_sys::GMIME_CONTENT_ENCODING_7BIT,
            ContentEncoding::Encode8bit => gmime_sys::GMIME_CONTENT_ENCODING_8BIT,
            ContentEncoding::EncodeBinary => gmime_sys::GMIME_CONTENT_ENCODING_BINARY,
            ContentEncoding::EncodeBase64 => gmime_sys::GMIME_CONTENT_ENCODING_BASE64,
            ContentEncoding::EncodeQuotedprintable => gmime_sys::GMIME_CONTENT_ENCODING_QUOTEDPRINTABLE,
            ContentEncoding::EncodeUuencode => gmime_sys::GMIME_CONTENT_ENCODING_UUENCODE,
            ContentEncoding::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<gmime_sys::GMimeContentEncoding> for ContentEncoding {
    fn from_glib(value: gmime_sys::GMimeContentEncoding) -> Self {
        match value {
            0 => ContentEncoding::EncodeDefault,
            1 => ContentEncoding::Encode7bit,
            2 => ContentEncoding::Encode8bit,
            3 => ContentEncoding::EncodeBinary,
            4 => ContentEncoding::EncodeBase64,
            5 => ContentEncoding::EncodeQuotedprintable,
            6 => ContentEncoding::EncodeUuencode,
            value => ContentEncoding::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum EncodingConstraint {
    Encode7bit,
    Encode8bit,
    EncodeBinary,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for EncodingConstraint {
    type GlibType = gmime_sys::GMimeEncodingConstraint;

    fn to_glib(&self) -> gmime_sys::GMimeEncodingConstraint {
        match *self {
            EncodingConstraint::Encode7bit => gmime_sys::GMIME_ENCODING_CONSTRAINT_7BIT,
            EncodingConstraint::Encode8bit => gmime_sys::GMIME_ENCODING_CONSTRAINT_8BIT,
            EncodingConstraint::EncodeBinary => gmime_sys::GMIME_ENCODING_CONSTRAINT_BINARY,
            EncodingConstraint::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<gmime_sys::GMimeEncodingConstraint> for EncodingConstraint {
    fn from_glib(value: gmime_sys::GMimeEncodingConstraint) -> Self {
        match value {
            0 => EncodingConstraint::Encode7bit,
            1 => EncodingConstraint::Encode8bit,
            2 => EncodingConstraint::EncodeBinary,
            value => EncodingConstraint::__Unknown(value),
        }
    }
}
