<!-- file * -->
<!-- enum AddressType -->
An address type.
<!-- enum AddressType::variant Sender -->
Represents the addresses in the Sender header.
<!-- enum AddressType::variant From -->
Represents the addresses in the From header.
<!-- enum AddressType::variant ReplyTo -->
Represents the addresses in the Reply-To header.
<!-- enum AddressType::variant To -->
Represents the recipients in the To header.
<!-- enum AddressType::variant Cc -->
Represents the recipients in the Cc header.
<!-- enum AddressType::variant Bcc -->
Represents the recipients in the Bcc header.
<!-- struct ApplicationPkcs7Mime -->
An application/pkcs7-mime MIME part.

# Implements

[`ApplicationPkcs7MimeExt`](trait.ApplicationPkcs7MimeExt.html), [`PartExt`](trait.PartExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait ApplicationPkcs7MimeExt -->
Trait containing all `ApplicationPkcs7Mime` methods.

# Implementors

[`ApplicationPkcs7Mime`](struct.ApplicationPkcs7Mime.html)
<!-- impl ApplicationPkcs7Mime::fn new -->
Creates a new application/pkcs7-mime object.
## `type_`
The type of S/MIME data contained within the part.

# Returns

an empty application/pkcs7-mime object.
<!-- impl ApplicationPkcs7Mime::fn encrypt -->
Attempts to encrypt the `entity` MIME part to the public keys of `recipients`
using S/MIME. If successful, a new application/pkcs7-mime object is returned.
## `entity`
a `Object` to encrypt
## `flags`
a `EncryptFlags`
## `recipients`
an array of recipients to encrypt to

# Returns

a new `ApplicationPkcs7Mime` object on success
or `None` on fail. If encrypting fails, an exception will be set on `err` to provide
information as to why the failure occurred.
<!-- impl ApplicationPkcs7Mime::fn sign -->
Attempts to sign the `entity` MIME part with `userid`'s private key using
S/MIME. If successful, a new application/pkcs7-mime object is returned.
## `entity`
a `Object`
## `userid`
the user id to sign with

# Returns

a new `ApplicationPkcs7Mime` object on success
or `None` on fail. If signing fails, an exception will be set on `err` to provide
information as to why the failure occurred.
<!-- trait ApplicationPkcs7MimeExt::fn decrypt -->
Attempts to decrypt the encrypted application/pkcs7-mime part.

When non-`None`, `session_key` should be a `None`-terminated string,
such as the one returned by `DecryptResultExt::get_session_key`
from a previous decryption. If the `session_key` is not valid, decryption
will fail.

If `result` is non-`None`, then on a successful decrypt operation, it will be
updated to point to a newly-allocated `DecryptResult` with signature
status information as well as a list of recipients that the part was
encrypted to.
## `flags`
a `DecryptFlags`
## `session_key`
session key to use or `None`
## `result`
the decryption result

# Returns

the decrypted MIME part on success or
`None` on fail. If the decryption fails, an exception will be set on
`err` to provide information as to why the failure occurred.
<!-- trait ApplicationPkcs7MimeExt::fn get_smime_type -->
Gets the smime-type value of the Content-Type header.

# Returns

the smime-type value.
<!-- trait ApplicationPkcs7MimeExt::fn verify -->
Attempts to verify the signed `self` part and extract the original
MIME entity.
## `flags`
a `VerifyFlags`
## `entity`
the extracted entity

# Returns

a new `SignatureList` object on
success or `None` on fail. If the verification fails, an exception
will be set on `err` to provide information as to why the failure
occurred.
<!-- struct AutocryptHeader -->
An object containing Autocrypt information about a given e-mail
address, as derived from a message header.

See https://autocrypt.org/ for details and motivation.

# Implements

[`AutocryptHeaderExt`](trait.AutocryptHeaderExt.html)
<!-- trait AutocryptHeaderExt -->
Trait containing all `AutocryptHeader` methods.

# Implementors

[`AutocryptHeader`](struct.AutocryptHeader.html)
<!-- impl AutocryptHeader::fn new -->
Creates a new `AutocryptHeader` object.

# Returns

a new `AutocryptHeader` object.
<!-- impl AutocryptHeader::fn new_from_string -->
Creates a new `AutocryptHeader` object based on the value of an
Autocrypt: header.

Note that this will not have an `effective_date` set, since the
`effective_date` is derived from the Date: line in the same block of
e-mail headers, but cannot be extracted from the raw Autocrypt:
header itself.
## `string`
The raw string value of an Autocrypt header

# Returns

a new `AutocryptHeader` object, or
`None` on error.
<!-- trait AutocryptHeaderExt::fn clone -->
If address and type already match between `src` and `self`, copy
keydata, prefer_encrypt, effective_date from `src` to `self`.
## `src`
a `AutocryptHeader` object
<!-- trait AutocryptHeaderExt::fn compare -->
Compare two Autocrypt Headers. This is useful for comparison, as well as for
sorting headers by:

 - address
 - effective_date
 - keydata
 - prefer_encrypt
## `ah2`
a `AutocryptHeader` object

# Returns

-1, 0, or 1 when `self` is less than, equal to, or greater than `ah2`.
<!-- trait AutocryptHeaderExt::fn get_address -->
Gets the internal address of the Autocrypt header, or `None` if not set.

# Returns

the address associated with the Autocrypt header
<!-- trait AutocryptHeaderExt::fn get_address_as_string -->
Gets the internal address of the Autocrypt header as a C string, or `None` if not set.

# Returns

the address associated with the Autocrypt header
<!-- trait AutocryptHeaderExt::fn get_effective_date -->
Gets the effective date of the Autocrypt header, or `None` if not set.

# Returns

the effective date associated with the Autocrypt header
<!-- trait AutocryptHeaderExt::fn get_keydata -->
Gets the raw keydata of the Autocrypt header, or `None` if not set.

# Returns

the raw key data associated with the Autocrypt header
<!-- trait AutocryptHeaderExt::fn get_prefer_encrypt -->
Gets the encryption preference stated by the Autocrypt header.

# Returns

the encryption preference associated with the Autocrypt header
<!-- trait AutocryptHeaderExt::fn is_complete -->
When dealing with Autocrypt headers derived from a message, some
sender addresses will not have a legitimate/complete header
associated with them. When a given sender address has no complete
header of a specific type, it should "reset" the state of the
associated address.

# Returns

`true` if the header is complete, or `false` if it is incomplete.
<!-- trait AutocryptHeaderExt::fn set_address -->
Set the address associated with the autocrypt_header.
## `address`
a `InternetAddressMailbox` value
<!-- trait AutocryptHeaderExt::fn set_address_from_string -->
Set the address associated with the autocrypt_header.
## `address`
a `None`-terminated string that is a raw e-mail address
<!-- trait AutocryptHeaderExt::fn set_effective_date -->
Set the effective date associated with the Autocrypt header.
## `effective_date`
a `glib::DateTime` object
<!-- trait AutocryptHeaderExt::fn set_keydata -->
Set the raw key data associated with the Autocrypt header.
<!-- trait AutocryptHeaderExt::fn set_prefer_encrypt -->
Set the encryption preference associated with the Autocrypt header.
## `pref`
a `AutocryptPreferEncrypt` value
<!-- trait AutocryptHeaderExt::fn to_string -->
Gets the string representation of the Autocrypt header, or `None` on
error. For example, it might return:

 prefer-encrypt=mutual; addr=bob\@example.com; keydata=AAAB15BE...

If you are using this object to populate an Autocrypt-Gossip
header, you should set `gossip` to `true` (this will suppress
inclusion of prefer-encrypt).
## `gossip`
a `gboolean`, indicating whether this header is for use with gossip

# Returns

the string representation of the
Autocrypt header.
<!-- struct AutocryptHeaderList -->
A list of Autocrypt headers, typically extracted from a GMimeMessage.

# Implements

[`AutocryptHeaderListExt`](trait.AutocryptHeaderListExt.html)
<!-- trait AutocryptHeaderListExt -->
Trait containing all `AutocryptHeaderList` methods.

# Implementors

[`AutocryptHeaderList`](struct.AutocryptHeaderList.html)
<!-- impl AutocryptHeaderList::fn new -->
Creates a new `AutocryptHeaderList` object.

# Returns

a new `AutocryptHeaderList` object.
<!-- trait AutocryptHeaderListExt::fn add -->
Adds a the passed `AutocryptHeader` to the list.
## `header`
a `AutocryptHeader` object
<!-- trait AutocryptHeaderListExt::fn add_missing_addresses -->
Adds a new incomplete `AutocryptHeader` object for each
InternetAddressMailbox found in `addresses`.
## `addresses`
an `InternetAddressList` object

# Returns

the number of addresses added
<!-- trait AutocryptHeaderListExt::fn get_count -->
See how many Autocrypt headers are in the list.

# Returns

the number of available Autocrypt headers
<!-- trait AutocryptHeaderListExt::fn get_header_at -->
Get the Nth header in the list (returns `None` on error, or if `index` is out of bounds)
## `index`
an index into the list

# Returns

a pointer to the Nth header in the list.
<!-- trait AutocryptHeaderListExt::fn get_header_for_address -->
Gets the Autocrypt header corresponding to the given `mailbox`.
## `mailbox`
an `InternetAddressMailbox` object

# Returns

a pointer to the header in the list which
matches the requested address, or `None` if no such header exists in
the list.
<!-- trait AutocryptHeaderListExt::fn remove_incomplete -->
Remove all incomplete Autocrypt headers from the list.
<!-- enum AutocryptPreferEncrypt -->
A description of the user's preference for encrypted messaging.
<!-- enum AutocryptPreferEncrypt::variant None -->
No preference stated.
<!-- enum AutocryptPreferEncrypt::variant Mutual -->
Please encrypt, if you also have this preference
<!-- struct Certificate -->
An object containing useful information about a certificate.

# Implements

[`CertificateExt`](trait.CertificateExt.html)
<!-- trait CertificateExt -->
Trait containing all `Certificate` methods.

# Implementors

[`Certificate`](struct.Certificate.html)
<!-- impl Certificate::fn new -->
Creates a new `Certificate` object.

# Returns

a new `Certificate` object.
<!-- trait CertificateExt::fn get_created -->
Get the creation date of the certificate's key.

# Returns

the creation date of the certificate's key or %-1 if unknown.
<!-- trait CertificateExt::fn get_digest_algo -->
Get the digest algorithm used by the certificate.

# Returns

the digest algorithm used by the certificate or
`DigestAlgo::Default` if unspecified.
<!-- trait CertificateExt::fn get_email -->
Get the email address associated with the certificate. If the
certificate contains more than one email address with different
validities, the email address with the highest validity is
returned. If more than one email address appears in the
certificate with the same (highest) validity, the first such email
address will be returned.

# Returns

the relevant e-mail address, or `None` if unspecified.
<!-- trait CertificateExt::fn get_expires -->
Get the expiration date of the certificate's key.

# Returns

the expiration date of the certificate's key or %-1 if unknown.
<!-- trait CertificateExt::fn get_fingerprint -->
Get the certificate's key fingerprint.

# Returns

the certificate's key fingerprint or `None` if unspecified.
<!-- trait CertificateExt::fn get_id_validity -->
Get the validity of the certificate's identity information. This
validity applies to the name, email, and user_id fields associated
with the certificate.

# Returns

the identity validity of the certificate.
<!-- trait CertificateExt::fn get_issuer_name -->
Get the certificate's issuer name.

# Returns

the certificate's issuer name or `None` if unspecified.
<!-- trait CertificateExt::fn get_issuer_serial -->
Get the certificate's issuer serial.

# Returns

the certificate's issuer serial or `None` if unspecified.
<!-- trait CertificateExt::fn get_key_id -->
Get the certificate's key id.

# Returns

the certificate's key id or `None` if unspecified.
<!-- trait CertificateExt::fn get_name -->
Get the name associated with the certificate. For email
certificates, this is usually the name of the person who controls
the certificate (encoded in UTF-8). If the certificate contains
more than one name with different validities, the name with the
highest validity is returned. If more than one name appears in the
certificate with the same (highest) validity, the first such name
will be returned.

# Returns

the the relevant name or `None` if unspecified.
<!-- trait CertificateExt::fn get_pubkey_algo -->
Get the public-key algorithm used by the certificate.

# Returns

the public-key algorithm used by the certificate or
`PubKeyAlgo::Default` if unspecified.
<!-- trait CertificateExt::fn get_trust -->
Get the certificate trust.

# Returns

the certificate trust.
<!-- trait CertificateExt::fn get_user_id -->
Get the certificate's full User ID. If the certificate contains
more than one User ID with different validities, the User ID with
the highest validity is returned. If more than one User ID appears
in the certificate with the same (highest) validity, the first such
User ID will be returned.

# Returns

the relevant User ID or `None` if unspecified.
<!-- trait CertificateExt::fn set_created -->
Set the creation date of the certificate's key.
## `created`
creation date
<!-- trait CertificateExt::fn set_digest_algo -->
Set the digest algorithm used by the certificate.
## `algo`
a `DigestAlgo`
<!-- trait CertificateExt::fn set_email -->
Set the email address associated with the
certificate. (e.g. "jane\@example.org")
## `email`
certificate's email
<!-- trait CertificateExt::fn set_expires -->
Set the expiration date of the certificate's key.
## `expires`
expiration date
<!-- trait CertificateExt::fn set_fingerprint -->
Set the certificate's key fingerprint.
## `fingerprint`
fingerprint string
<!-- trait CertificateExt::fn set_id_validity -->
Set the validity associated with the certificate's name, email, and user_id.
## `validity`
a `Validity` representing the validity of the certificate's identity information.
<!-- trait CertificateExt::fn set_issuer_name -->
Set the certificate's issuer name.
## `issuer_name`
certificate's issuer name
<!-- trait CertificateExt::fn set_issuer_serial -->
Set the certificate's issuer serial.
## `issuer_serial`
certificate's issuer serial
<!-- trait CertificateExt::fn set_key_id -->
Set the certificate's key id.
## `key_id`
key id
<!-- trait CertificateExt::fn set_name -->
Set the name associated with the certificate. For email
certificates, this is usually the name of the person who controls
the certificate (encoded in UTF-8). (e.g. "Jane Doe")
## `name`
certificate's name
<!-- trait CertificateExt::fn set_pubkey_algo -->
Set the public-key algorithm used by the certificate.
## `algo`
a `PubKeyAlgo`
<!-- trait CertificateExt::fn set_trust -->
Set the certificate trust.
## `trust`
a `Trust` value
<!-- trait CertificateExt::fn set_user_id -->
Set the certificate's full User ID. By convention, this is usually
a mail name-addr as described in RFC 5322. (e.g. "Jane Doe
&lt;jane\@example.org&gt;")
## `user_id`
the full User ID for a certificate
<!-- struct CertificateList -->
A collection of `Certificate` objects.

# Implements

[`CertificateListExt`](trait.CertificateListExt.html)
<!-- trait CertificateListExt -->
Trait containing all `CertificateList` methods.

# Implementors

[`CertificateList`](struct.CertificateList.html)
<!-- impl CertificateList::fn new -->
Creates a new `CertificateList`.

# Returns

a new `CertificateList`.
<!-- trait CertificateListExt::fn add -->
Adds a `Certificate` to the `CertificateList`.
## `cert`
a `Certificate`

# Returns

the index of the added `Certificate`.
<!-- trait CertificateListExt::fn clear -->
Clears the list of certificates.
<!-- trait CertificateListExt::fn contains -->
Checks whether or not the specified `Certificate` is contained within
the `CertificateList`.
## `cert`
a `Certificate`

# Returns

`true` if the specified `Certificate` is contained within the
specified `CertificateList` or `false` otherwise.
<!-- trait CertificateListExt::fn get_certificate -->
Gets the `Certificate` at the specified index.
## `index`
index of `Certificate` to get

# Returns

the `Certificate` at the specified
index or `None` if the index is out of range.
<!-- trait CertificateListExt::fn index_of -->
Gets the index of the specified `Certificate` inside the
`CertificateList`.
## `cert`
a `Certificate`

# Returns

the index of the requested `Certificate` within the
`CertificateList` or %-1 if it is not contained within the
`CertificateList`.
<!-- trait CertificateListExt::fn insert -->
Inserts a `Certificate` into the `CertificateList` at the specified
index.
## `index`
index to insert at
## `cert`
a `Certificate`
<!-- trait CertificateListExt::fn length -->
Gets the length of the list.

# Returns

the number of `Certificate` objects in the list.
<!-- trait CertificateListExt::fn remove -->
Removes a `Certificate` from the `CertificateList`.
## `cert`
a `Certificate`

# Returns

`true` if the specified `Certificate` was removed or `false`
otherwise.
<!-- trait CertificateListExt::fn remove_at -->
Removes a `Certificate` from the `CertificateList` at the specified
index.
## `index`
index of the certificate to remove

# Returns

`true` if a `Certificate` was removed or `false` otherwise.
<!-- trait CertificateListExt::fn set_certificate -->
Sets the `Certificate` at the specified index to `cert`.
## `index`
index of `Certificate` to set
## `cert`
a `Certificate`
<!-- struct ContentDisposition -->
A data structure representing a Content-Disposition.

# Implements

[`ContentDispositionExt`](trait.ContentDispositionExt.html)
<!-- trait ContentDispositionExt -->
Trait containing all `ContentDisposition` methods.

# Implementors

[`ContentDisposition`](struct.ContentDisposition.html)
<!-- impl ContentDisposition::fn new -->
Creates a new `ContentDisposition` object.

# Returns

a new `ContentDisposition` object.
<!-- impl ContentDisposition::fn parse -->
Parses the input string into a `ContentDisposition` object.
## `options`
a `ParserOptions` or `None`
## `str`
Content-Disposition field value

# Returns

a new `ContentDisposition` object.
<!-- trait ContentDispositionExt::fn encode -->
Encodes the Content-Disposition header.
## `options`
a `FormatOptions` or `None`

# Returns

a new string containing the encoded header value.
<!-- trait ContentDispositionExt::fn get_disposition -->
Gets the disposition or `None` on fail.

# Returns

the disposition string which is probably one of
`GMIME_DISPOSITION_ATTACHMENT` or `GMIME_DISPOSITION_INLINE`.
<!-- trait ContentDispositionExt::fn get_parameter -->
Gets the parameter value specified by `name` if it's available.
## `name`
parameter name

# Returns

the value of the requested parameter or `None` if the
parameter is not set. If the parameter is set, the returned string
will be in UTF-8.
<!-- trait ContentDispositionExt::fn get_parameters -->
Gets the Content-Disposition parameter list.

# Returns

the Content-Disposition's parameter list.
<!-- trait ContentDispositionExt::fn is_attachment -->
Determines if a Content-Disposition has a value of "attachment".

# Returns

`true` if the value matches "attachment", otherwise `false`.
<!-- trait ContentDispositionExt::fn set_disposition -->
Sets the disposition to `value` which may be one of
`GMIME_DISPOSITION_ATTACHMENT` or `GMIME_DISPOSITION_INLINE` or, by
your choice, any other string which would indicate how the MIME
part should be displayed by the MUA.
## `value`
disposition value
<!-- trait ContentDispositionExt::fn set_parameter -->
Sets a parameter on the Content-Disposition.

Note: The `name` should be in US-ASCII while the `value` should be in
UTF-8.
## `name`
parameter name
## `value`
parameter value
<!-- enum ContentEncoding -->
A Content-Transfer-Encoding enumeration.
<!-- enum ContentEncoding::variant Default -->
Default transfer encoding.
<!-- enum ContentEncoding::variant 7bit -->
7bit text transfer encoding.
<!-- enum ContentEncoding::variant 8bit -->
8bit text transfer encoding.
<!-- enum ContentEncoding::variant Binary -->
Binary transfer encoding.
<!-- enum ContentEncoding::variant Base64 -->
Base64 transfer encoding.
<!-- enum ContentEncoding::variant Quotedprintable -->
Quoted-printable transfer encoding.
<!-- enum ContentEncoding::variant Uuencode -->
Uuencode transfer encoding.
<!-- struct ContentType -->
A data structure representing a Content-Type.

# Implements

[`ContentTypeExt`](trait.ContentTypeExt.html)
<!-- trait ContentTypeExt -->
Trait containing all `ContentType` methods.

# Implementors

[`ContentType`](struct.ContentType.html)
<!-- impl ContentType::fn new -->
Creates a Content-Type object with type `type_` and subtype `subtype`.
## `type_`
the MIME type or `None` for the default value
## `subtype`
the MIME subtype or `None` for the default value

# Returns

a new `ContentType` object.
<!-- impl ContentType::fn parse -->
Parses the input string into a `ContentType` object.
## `options`
a `ParserOptions` or `None`
## `str`
input string containing a content-type (and params)

# Returns

a new `ContentType` object.
<!-- trait ContentTypeExt::fn encode -->
Encodes the Content-Disposition header.
## `options`
a `FormatOptions` or `None`

# Returns

a new string containing the encoded header value.
<!-- trait ContentTypeExt::fn get_media_subtype -->
Gets the Content-Type's media sub-type.

# Returns

the Content-Type's media sub-type.
<!-- trait ContentTypeExt::fn get_media_type -->
Gets the Content-Type's media type.

# Returns

the Content-Type's media type.
<!-- trait ContentTypeExt::fn get_mime_type -->
Allocates a string buffer containing the type and subtype defined
by the `self`.

# Returns

an allocated string containing the type and subtype of the
content-type in the format: type/subtype.
<!-- trait ContentTypeExt::fn get_parameter -->
Gets the parameter value specified by `name` if it's available.
## `name`
parameter name (aka attribute)

# Returns

the value of the requested parameter or `None` if the
parameter is not set. If the parameter is set, the returned string
will be in UTF-8.
<!-- trait ContentTypeExt::fn get_parameters -->
Gets the Content-Type's parameter list.

# Returns

the Content-Type's parameter list.
<!-- trait ContentTypeExt::fn is_type -->
Compares the given type and subtype with that of the given mime
type object.
## `type_`
MIME type to compare against
## `subtype`
MIME subtype to compare against

# Returns

`true` if the MIME types match or `false` otherwise. You may
use "*" in place of `type_` and/or `subtype` as a wilcard.
<!-- trait ContentTypeExt::fn set_media_subtype -->
Sets the Content-Type's media subtype.
## `subtype`
media subtype
<!-- trait ContentTypeExt::fn set_media_type -->
Sets the Content-Type's media type.
## `type_`
media type
<!-- trait ContentTypeExt::fn set_parameter -->
Sets a parameter on the Content-Type.

Note: The `name` should be in US-ASCII while the `value` should be in
UTF-8.
## `name`
parameter name (aka attribute)
## `value`
parameter value
<!-- struct CryptoContext -->
A crypto context for use with MIME.

# Implements

[`CryptoContextExt`](trait.CryptoContextExt.html)
<!-- trait CryptoContextExt -->
Trait containing all `CryptoContext` methods.

# Implementors

[`CryptoContext`](struct.CryptoContext.html), [`GpgContext`](struct.GpgContext.html), [`Pkcs7Context`](struct.Pkcs7Context.html)
<!-- impl CryptoContext::fn new -->
Creates a new crypto context for the specified `protocol`.
## `protocol`
the crypto protocol

# Returns

a newly allocated `CryptoContext`.
<!-- impl CryptoContext::fn register -->
Registers the callback for the specified `protocol`.
## `protocol`
crypto protocol
## `callback`
a `GMimeCryptoContextNewFunc`
<!-- trait CryptoContextExt::fn decrypt -->
Decrypts the ciphertext input stream and writes the resulting cleartext
to the output stream.

When non-`None`, `session_key` should be a `None`-terminated string,
such as the one returned by `DecryptResultExt::get_session_key`
from a previous decryption. If the `session_key` is not valid, decryption
will fail.

If the encrypted input stream was also signed, the returned
`DecryptResult` will have a non-`None` list of signatures, each with a
`SignatureStatus` (among other details about each signature).

On success, the returned `DecryptResult` will contain a list of
certificates, one for each recipient, that the original encrypted stream
was encrypted to.

Note: It *may* be possible to maliciously design an encrypted stream such
that recursively decrypting it will result in an endless loop, causing
a denial of service attack on your application.
## `flags`
a set of `DecryptFlags`
## `session_key`
the session key to use or `None`
## `istream`
input/ciphertext stream
## `ostream`
output/cleartext stream

# Returns

a `DecryptResult` on success or `None`
on error.
<!-- trait CryptoContextExt::fn digest_id -->
Gets the digest id based on the digest name.
## `name`
digest name

# Returns

the equivalent digest id or `DigestAlgo::Default` on fail.
<!-- trait CryptoContextExt::fn digest_name -->
Gets the digest name based on the digest id `digest`.
## `digest`
digest id

# Returns

the equivalent digest name or `None` on fail.
<!-- trait CryptoContextExt::fn encrypt -->
Encrypts (and optionally signs) the cleartext input stream and
writes the resulting ciphertext to the output stream.
## `sign`
sign as well as encrypt
## `userid`
the key id (or email address) to use when signing (assuming `sign` is `true`)
## `flags`
a set of `EncryptFlags`
## `recipients`
an array of recipient key ids and/or email addresses
## `istream`
cleartext input stream
## `ostream`
ciphertext output stream

# Returns

`0` on success or %-1 on fail.
<!-- trait CryptoContextExt::fn export_keys -->
Exports the keys/certificates in `keys` to the stream `ostream` from
the key/certificate database controlled by `self`.

If `keys` is `None` or contains only a `None` element, then all keys
will be exported.
## `keys`
an array of key ids, terminated by a `None` element
## `ostream`
output stream

# Returns

`0` on success or %-1 on fail.
<!-- trait CryptoContextExt::fn get_encryption_protocol -->
Gets the encryption protocol for the crypto context.

# Returns

the encryption protocol or `None` if not supported.
<!-- trait CryptoContextExt::fn get_key_exchange_protocol -->
Gets the key exchange protocol for the crypto context.

# Returns

the key exchange protocol or `None` if not supported.
<!-- trait CryptoContextExt::fn get_signature_protocol -->
Gets the signature protocol for the crypto context.

# Returns

the signature protocol or `None` if not supported.
<!-- trait CryptoContextExt::fn import_keys -->
Imports a stream of keys/certificates contained within `istream`
into the key/certificate database controlled by `self`.
## `istream`
input stream (containing keys)

# Returns

the total number of keys imported on success or %-1 on fail.
<!-- trait CryptoContextExt::fn set_request_password -->
Sets the function used by the `self` for requesting a password from
the user.
## `request_passwd`
a callback function for requesting a password
<!-- trait CryptoContextExt::fn sign -->
Signs the input stream and writes the resulting signature to the output stream.
## `detach`
`true` if `ostream` should be the detached signature; otherwise, `false`
## `userid`
private key to use to sign the stream
## `istream`
input stream
## `ostream`
output stream

# Returns

the `DigestAlgo` used on success or %-1 on fail.
<!-- trait CryptoContextExt::fn verify -->
Verifies the signature. If `istream` is a clearsigned stream, you
should pass `None` as the `sigstream` parameter and may wish to
provide an `ostream` argument for GMime to output the original
plaintext into. Otherwise `sigstream` is assumed to be the signature
stream and is used to verify the integirity of the `istream`.
## `flags`
a `VerifyFlags`
## `istream`
input stream
## `sigstream`
detached-signature stream
## `ostream`
output stream for use with encapsulated signature input streams

# Returns

a `SignatureList` object containing
the status of each signature or `None` on error.
<!-- struct DataWrapper -->
A wrapper for a stream which may be encoded.

# Implements

[`DataWrapperExt`](trait.DataWrapperExt.html)
<!-- trait DataWrapperExt -->
Trait containing all `DataWrapper` methods.

# Implementors

[`DataWrapper`](struct.DataWrapper.html)
<!-- impl DataWrapper::fn new -->
Creates a new `DataWrapper` object.

# Returns

a new data wrapper object.
<!-- impl DataWrapper::fn new_with_stream -->
Creates a new `DataWrapper` object around `stream`.
## `stream`
a `Stream`
## `encoding`
stream's encoding

# Returns

a data wrapper around `stream`. Since the wrapper owns its
own reference on the stream, caller is responsible for unrefing
its own copy.
<!-- trait DataWrapperExt::fn get_encoding -->
Gets the encoding type of the stream wrapped by `self`.

# Returns

the encoding type of the internal stream.
<!-- trait DataWrapperExt::fn get_stream -->
Gets a reference to the stream wrapped by `self`.

# Returns

a reference to the internal stream.
<!-- trait DataWrapperExt::fn set_encoding -->
Sets the encoding type of the internal stream.
## `encoding`
encoding
<!-- trait DataWrapperExt::fn set_stream -->
Replaces the wrapper's internal stream with `stream`. Don't forget,
if `stream` is not of the same encoding as the old stream, you'll
want to call `DataWrapperExt::set_encoding` as well.

Note: caller is responsible for its own reference on
`stream`.
## `stream`
a `Stream`
<!-- trait DataWrapperExt::fn write_to_stream -->
Writes the raw (decoded) data to the output stream.
## `stream`
output stream

# Returns

the number of bytes written or %-1 on failure.
<!-- struct DecryptResult -->
An object containing the results from decrypting an encrypted stream.

# Implements

[`DecryptResultExt`](trait.DecryptResultExt.html)
<!-- trait DecryptResultExt -->
Trait containing all `DecryptResult` methods.

# Implementors

[`DecryptResult`](struct.DecryptResult.html)
<!-- impl DecryptResult::fn new -->
Creates a new `DecryptResult` object.

# Returns

a new `DecryptResult` object.
<!-- trait DecryptResultExt::fn get_cipher -->
Get the cipher algorithm used.

# Returns

the cipher algorithm used.
<!-- trait DecryptResultExt::fn get_mdc -->
Get the mdc digest algorithm used.

# Returns

the mdc digest algorithm used.
<!-- trait DecryptResultExt::fn get_recipients -->
Gets the list of certificates that the stream had been encrypted to.

# Returns

a `CertificateList`.
<!-- trait DecryptResultExt::fn get_session_key -->
Get the session key used for this decryption.

# Returns

the session key digest algorithm used, or `None` if no
session key was requested or found.
<!-- trait DecryptResultExt::fn get_signatures -->
Gets a list of signatures if the encrypted stream had also been signed.

# Returns

a `SignatureList` or `None` if the
stream was not signed.
<!-- trait DecryptResultExt::fn set_cipher -->
Set the cipher algorithm used.
## `cipher`
a `CipherAlgo`
<!-- trait DecryptResultExt::fn set_mdc -->
Set the mdc digest algorithm used.
## `mdc`
a `DigestAlgo`
<!-- trait DecryptResultExt::fn set_recipients -->
Sets the list of certificates that the stream had been encrypted to.
## `recipients`
A `CertificateList`
<!-- trait DecryptResultExt::fn set_session_key -->
Set the session key to be returned by this decryption result.
## `session_key`
a string representing the session key or `None` to unset the key
<!-- trait DecryptResultExt::fn set_signatures -->
Sets the list of signatures.
## `signatures`
A `SignatureList`
<!-- enum DigestAlgo -->
A hash algorithm.
<!-- enum DigestAlgo::variant Default -->
The default hash algorithm.
<!-- enum DigestAlgo::variant Md5 -->
The MD5 hash algorithm.
<!-- enum DigestAlgo::variant Sha1 -->
The SHA-1 hash algorithm.
<!-- enum DigestAlgo::variant Ripemd160 -->
The RIPEMD-160 hash algorithm.
<!-- enum DigestAlgo::variant Md2 -->
The MD2 hash algorithm.
<!-- enum DigestAlgo::variant Tiger192 -->
The TIGER-192 hash algorithm.
<!-- enum DigestAlgo::variant Haval5160 -->
The HAVAL-5-160 hash algorithm.
<!-- enum DigestAlgo::variant Sha256 -->
The SHA-256 hash algorithm.
<!-- enum DigestAlgo::variant Sha384 -->
The SHA-384 hash algorithm.
<!-- enum DigestAlgo::variant Sha512 -->
The SHA-512 hash algorithm.
<!-- enum DigestAlgo::variant Sha224 -->
The SHA-224 hash algorithm.
<!-- enum DigestAlgo::variant Md4 -->
The MD4 hash algorithm.
<!-- enum DigestAlgo::variant Crc32 -->
The CRC32 hash algorithm.
<!-- enum DigestAlgo::variant Crc32Rfc1510 -->
The rfc1510 CRC32 hash algorithm.
<!-- enum DigestAlgo::variant Crc32Rfc2440 -->
The rfc2440 CRC32 hash algorithm.
<!-- enum EncodingConstraint -->
Used with functions like `FilterBestExt::encoding` and
`ObjectExt::encode` as the 'constraint' argument. These values
provide a means of letting the filter know what the encoding
constraints are for the stream.
<!-- enum EncodingConstraint::variant 7bit -->
The stream data must fit within the 7bit ASCII range.
<!-- enum EncodingConstraint::variant 8bit -->
The stream data may have bytes with the high bit set, but no null bytes.
<!-- enum EncodingConstraint::variant Binary -->
The stream may contain any binary data.
<!-- enum EncryptFlags -->
Encryption flags.
<!-- enum EncryptFlags::variant None -->
No flags specified.
<!-- enum EncryptFlags::variant AlwaysTrust -->
Always trust the specified keys.
<!-- enum EncryptFlags::variant NoCompress -->
Don't compress the plaintext before encrypting.
<!-- enum EncryptFlags::variant Symmetric -->
Encrypt symmetrically.
<!-- enum EncryptFlags::variant ThrowKeyids -->
Do not include the key ids in the ciphertext.
<!-- struct Filter -->
Base class for filters used by `StreamFilter`.

# Implements

[`FilterExt`](trait.FilterExt.html)
<!-- trait FilterExt -->
Trait containing all `Filter` methods.

# Implementors

[`FilterBasic`](struct.FilterBasic.html), [`FilterBest`](struct.FilterBest.html), [`FilterCharset`](struct.FilterCharset.html), [`FilterDos2Unix`](struct.FilterDos2Unix.html), [`FilterEnriched`](struct.FilterEnriched.html), [`FilterFrom`](struct.FilterFrom.html), [`FilterGZip`](struct.FilterGZip.html), [`FilterHTML`](struct.FilterHTML.html), [`FilterOpenPGP`](struct.FilterOpenPGP.html), [`FilterSmtpData`](struct.FilterSmtpData.html), [`FilterStrip`](struct.FilterStrip.html), [`FilterUnix2Dos`](struct.FilterUnix2Dos.html), [`FilterWindows`](struct.FilterWindows.html), [`FilterYenc`](struct.FilterYenc.html), [`Filter`](struct.Filter.html)
<!-- trait FilterExt::fn backup -->
Sets number of bytes backed up on the input, new calls replace
previous ones
## `data`
data to backup
## `length`
length of `data`
<!-- trait FilterExt::fn complete -->
Completes the filtering.
## `inbuf`
input buffer
## `inlen`
input buffer length
## `prespace`
prespace buffer length
## `outbuf`

 pointer to output buffer
## `outlen`
pointer to output length
## `outprespace`
pointer to output prespace buffer length
<!-- trait FilterExt::fn copy -->
Copies `self` into a new `Filter` object.

# Returns

a duplicate of `self`.
<!-- trait FilterExt::fn filter -->
Filters the input data and writes it to `out`.
## `inbuf`
input buffer
## `inlen`
input buffer length
## `prespace`
prespace buffer length
## `outbuf`

 pointer to output buffer
## `outlen`
pointer to output length
## `outprespace`
pointer to output prespace buffer length
<!-- trait FilterExt::fn reset -->
Resets the filter.
<!-- trait FilterExt::fn set_size -->
Ensure this much size is available for filter output (if required)
## `size`
requested size for the output buffer
## `keep`
`true` if existing data in the output buffer should be kept
<!-- struct FilterBasic -->
A basic encoder/decoder filter for the MIME encodings.

# Implements

[`FilterExt`](trait.FilterExt.html)
<!-- impl FilterBasic::fn new -->
Creates a new basic filter for `encoding`.
## `encoding`
a `ContentEncoding`
## `encode`
`true` to encode or `false` to decode

# Returns

a new basic encoder filter.
<!-- struct FilterBest -->
A filter for calculating the best encoding and/or charset to encode
the data passed through it.

# Implements

[`FilterBestExt`](trait.FilterBestExt.html), [`FilterExt`](trait.FilterExt.html)
<!-- trait FilterBestExt -->
Trait containing all `FilterBest` methods.

# Implementors

[`FilterBest`](struct.FilterBest.html)
<!-- impl FilterBest::fn new -->
Creates a new GMimeFilterBest filter. `flags` are used to determine
which information to keep statistics of. If the
`FilterBestFlags::Charset` bit is set, the filter will be able to
compute the best charset for encoding the stream of data
filtered. If the `FilterBestFlags::Encoding` bit is set, the filter
will be able to compute the best Content-Transfer-Encoding for use
with the stream being filtered.

Note: In order for the `FilterBestExt::charset` function to
work, the stream being filtered MUST already be encoded in UTF-8.
## `flags`
filter flags

# Returns

a new best filter with flags `flags`.
<!-- trait FilterBestExt::fn charset -->
Calculates the best charset for encoding the stream filtered
through the `self` filter.

# Returns

a pointer to a string containing the name of the charset
best suited for the text filtered through `self`.
<!-- trait FilterBestExt::fn encoding -->
Calculates the most efficient Content-Transfer-Encoding for the
stream filtered through `self` that fits within the encoding
`constraint`.
## `constraint`
a `EncodingConstraint`

# Returns

the best encoding for the stream filtered by `self`.
<!-- struct FilterCharset -->
A filter to convert between charsets.

# Implements

[`FilterExt`](trait.FilterExt.html)
<!-- impl FilterCharset::fn new -->
Creates a new `FilterCharset` filter.
## `from_charset`
charset to convert from
## `to_charset`
charset to convert to

# Returns

a new charset filter or `None` if the charset conversion is
not possible.
<!-- struct FilterDos2Unix -->
A filter to convert a stream from Windows/DOS line endings to Unix line endings.

# Implements

[`FilterExt`](trait.FilterExt.html)
<!-- impl FilterDos2Unix::fn new -->
Creates a new `FilterDos2Unix` filter.
## `ensure_newline`
`true` if the filter should ensure that the stream ends in a new line

# Returns

a new `FilterDos2Unix` filter.
<!-- struct FilterEnriched -->
A filter for converting text/enriched or text/richtext textual
streams into text/html.

# Implements

[`FilterExt`](trait.FilterExt.html)
<!-- impl FilterEnriched::fn new -->
Creates a new GMimeFilterEnriched object.
## `flags`
flags

# Returns

a new GMimeFilter object.
<!-- struct FilterFrom -->
A filter for armoring or escaping lines beginning with "From ".

# Implements

[`FilterExt`](trait.FilterExt.html)
<!-- impl FilterFrom::fn new -->
Creates a new GMimeFilterFrom filter. If `mode` is
`FilterFromMode::Armor`, the from-filter will encode from
lines using the quoted-printable encoding resulting in "=46rom ".
Using the `FilterFromMode::Default` or
`FilterFromMode::Escape` mode (they are the same), from lines
will be escaped to ">From ".

Note: If you plan on using a from-filter in mode ARMOR, you should
remember to also use a `FilterBasic` filter with an encoding of
`ContentEncoding::Quotedprintable`.
## `mode`
filter mode

# Returns

a new from filter with mode `mode`.
<!-- enum FilterFromMode -->
The mode for a `FilterFrom` filter.
<!-- enum FilterFromMode::variant Default -->
Default mode.
<!-- enum FilterFromMode::variant Escape -->
Escape 'From ' lines with a '>'
<!-- enum FilterFromMode::variant Armor -->
QP-Encode 'From ' lines
<!-- struct FilterGZip -->
A filter for compresing or decompressing a gzip stream.

# Implements

[`FilterGZipExt`](trait.FilterGZipExt.html), [`FilterExt`](trait.FilterExt.html)
<!-- trait FilterGZipExt -->
Trait containing all `FilterGZip` methods.

# Implementors

[`FilterGZip`](struct.FilterGZip.html)
<!-- impl FilterGZip::fn new -->
Creates a new gzip (or gunzip) filter.
## `mode`
zip or unzip
## `level`
compression level

# Returns

a new gzip (or gunzip) filter.
<!-- trait FilterGZipExt::fn get_comment -->
Gets the comment that was either previously set or retrieved when decoding a gzip stream.

Feature: `v3_2`


# Returns

a string containing the comment.
<!-- trait FilterGZipExt::fn get_filename -->
Gets the filename that was either previously set or retrieved when decoding a gzip stream.

Feature: `v3_2`


# Returns

a string containing th ename of the file.
<!-- trait FilterGZipExt::fn set_comment -->
Sets the comment that should be used when generating the gzip header.

Feature: `v3_2`

## `comment`
The comment
<!-- trait FilterGZipExt::fn set_filename -->
Sets the filename that should be used when generating the gzip header.

Feature: `v3_2`

## `filename`
The name of the file
<!-- enum FilterGZipMode -->
The mode for the `FilterGZip` filter.
<!-- enum FilterGZipMode::variant Zip -->
Compress (zip) mode.
<!-- enum FilterGZipMode::variant Unzip -->
Uncompress (unzip) mode.
<!-- struct FilterHTML -->
A filter for converting text/plain into text/html.

# Implements

[`FilterExt`](trait.FilterExt.html)
<!-- impl FilterHTML::fn new -->
Creates a new GMimeFilterHTML filter which can be used to convert a
plain UTF-8 text stream into an html stream.
## `flags`
html flags
## `colour`
citation colour

# Returns

a new html filter.
<!-- struct FilterOpenPGP -->
A filter to detect OpenPGP markers.

Feature: `v3_2`

# Implements

[`FilterOpenPGPExt`](trait.FilterOpenPGPExt.html), [`FilterExt`](trait.FilterExt.html)
<!-- trait FilterOpenPGPExt -->
Trait containing all `FilterOpenPGP` methods.

Feature: `v3_2`

# Implementors

[`FilterOpenPGP`](struct.FilterOpenPGP.html)
<!-- impl FilterOpenPGP::fn new -->
Creates a new `FilterOpenPGP` filter.

Feature: `v3_2`


# Returns

a new `FilterOpenPGP` filter.
<!-- trait FilterOpenPGPExt::fn get_begin_offset -->
Gets the stream offset of the beginning of the OpenPGP data block, if any have been found.

Feature: `v3_2`


# Returns

The stream offset or %-1 if no OpenPGP block was found.
<!-- trait FilterOpenPGPExt::fn get_data_type -->
Gets the type of OpenPGP data that has been detected.

Feature: `v3_2`


# Returns

a `OpenPGPData` value.
<!-- trait FilterOpenPGPExt::fn get_end_offset -->
Gets the stream offset of the end of the OpenPGP data block, if any have been found.

Feature: `v3_2`


# Returns

The stream offset or %-1 if no OpenPGP block was found.
<!-- struct FilterSmtpData -->
A filter to byte-stuff SMTP DATA.

# Implements

[`FilterExt`](trait.FilterExt.html)
<!-- impl FilterSmtpData::fn new -->
Creates a new `FilterSmtpData` filter.

# Returns

a new `FilterSmtpData` filter.
<!-- struct FilterStrip -->
A filter for stripping whitespace from the end of lines.

# Implements

[`FilterExt`](trait.FilterExt.html)
<!-- impl FilterStrip::fn new -->
Creates a new `FilterStrip` filter which will strip trailing
whitespace from every line of input passed through the filter.

# Returns

a new strip filter.
<!-- struct FilterUnix2Dos -->
A filter to convert a stream from Windows/DOS line endings to Unix line endings.

# Implements

[`FilterExt`](trait.FilterExt.html)
<!-- impl FilterUnix2Dos::fn new -->
Creates a new `FilterUnix2Dos` filter.
## `ensure_newline`
`true` if the filter should ensure that the stream ends in a new line

# Returns

a new `FilterUnix2Dos` filter.
<!-- struct FilterWindows -->
A filter for detecting whether or not a text stream claimed to be
iso-8859-X is really that charset or if it is really a
Windows-CP125x charset.

# Implements

[`FilterWindowsExt`](trait.FilterWindowsExt.html), [`FilterExt`](trait.FilterExt.html)
<!-- trait FilterWindowsExt -->
Trait containing all `FilterWindows` methods.

# Implementors

[`FilterWindows`](struct.FilterWindows.html)
<!-- impl FilterWindows::fn new -->
Creates a new GMimeFilterWindows filter. When a stream of text has
been filtered, it can be determined whether or not said text stream
was in `claimed_charset` or the equivalent Windows-CP125# charset.
## `claimed_charset`
charset that a text stream claims to be

# Returns

a new windows filter.
<!-- trait FilterWindowsExt::fn is_windows_charset -->
Determines whether or not a Windows-CP125# charset has been
detected so far.

# Returns

`true` if the filtered stream has been detected to contain
Windows-CP125# characters or `false` otherwise.
<!-- trait FilterWindowsExt::fn real_charset -->
Figures out the real charset that the text is encoded in based on whether or not Windows-CP125# characters were found.

# Returns

a const string pointer to the claimed charset if filtered
text stream was found not to contain any Windows-CP125# characters
or the proper Windows-CP125# charset.
<!-- struct FilterYenc -->
A filter for yEncoding or yDecoding a stream.

# Implements

[`FilterYencExt`](trait.FilterYencExt.html), [`FilterExt`](trait.FilterExt.html)
<!-- trait FilterYencExt -->
Trait containing all `FilterYenc` methods.

# Implementors

[`FilterYenc`](struct.FilterYenc.html)
<!-- impl FilterYenc::fn new -->
Creates a new yEnc filter.
## `encode`
encode vs decode

# Returns

a new yEnc filter.
<!-- trait FilterYencExt::fn get_crc -->
Get the computed crc or (guint32) -1 on fail.

# Returns

the computed crc or (guint32) -1 on fail.
<!-- trait FilterYencExt::fn get_pcrc -->
Get the computed part crc or (guint32) -1 on fail.

# Returns

the computed part crc or (guint32) -1 on fail.
<!-- trait FilterYencExt::fn set_crc -->
Sets the current crc32 value on the yEnc filter `self` to `crc`.
## `crc`
crc32
<!-- trait FilterYencExt::fn set_state -->
Sets the current state of the yencoder/ydecoder
## `state`
encode/decode state
<!-- enum Format -->
An enum of formats.
<!-- enum Format::variant Message -->
The stream contains a single message.
<!-- enum Format::variant Mbox -->
The stream is in the UNIX mbox format.
<!-- enum Format::variant Mmdf -->
The stream is in the MMDF format.
<!-- struct FormatOptions -->
Format options for serializing various GMime objects.
<!-- impl FormatOptions::fn new -->
Creates a new set of `FormatOptions`.

# Returns

a newly allocated set of `FormatOptions` with the default values.
<!-- impl FormatOptions::fn add_hidden_header -->
Adds the given header to the list of headers that should be hidden.
## `header`
a header name
<!-- impl FormatOptions::fn clear_hidden_headers -->
Clears the list of headers that should be hidden.
<!-- impl FormatOptions::fn clone -->
Clones a `FormatOptions`.

# Returns

a newly allocated `FormatOptions`.
<!-- impl FormatOptions::fn create_newline_filter -->
Creates a `Filter` suitable for converting line-endings to the
currently set new-line format.
## `ensure_newline`
`true` if the output must *always* end with a new line

# Returns

a `Filter` to convert to the specified new-line format.
<!-- impl FormatOptions::fn free -->
Frees a set of `FormatOptions`.
<!-- impl FormatOptions::fn get_newline -->
Gets a string representing the currently set new-line format.

# Returns

a new-line character sequence.
<!-- impl FormatOptions::fn get_newline_format -->
Gets the new-line format to use when writing out messages and headers.

# Returns

the new-line format that is currently set.
<!-- impl FormatOptions::fn get_param_encoding_method -->
Gets the parameter encoding method to use for `Param` parameters that do not
already have an encoding method specified.

# Returns

the encoding method that is currently set.
<!-- impl FormatOptions::fn is_hidden_header -->
Gets whether or not the specified header should be hidden.
## `header`
the name of a header

# Returns

`true` if the header should be hidden or `false` otherwise.
<!-- impl FormatOptions::fn remove_hidden_header -->
Removes the given header from the list of headers that should be hidden.
## `header`
a header name
<!-- impl FormatOptions::fn set_newline_format -->
Sets the new-line format that should be used when writing headers and messages.
## `newline`
a `NewLineFormat`
<!-- impl FormatOptions::fn set_param_encoding_method -->
Sets the parameter encoding method to use when encoding parameters which
do not have an encoding method specified.

Note: `ParamEncodingMethod::Default` is not allowed.
## `method`
a `ParamEncodingMethod`
<!-- impl FormatOptions::fn get_default -->
Gets the default format options.

# Returns

the default format options.
<!-- struct GpgContext -->
A GnuPG crypto context.

# Implements

[`CryptoContextExt`](trait.CryptoContextExt.html)
<!-- impl GpgContext::fn new -->
Creates a new gpg crypto context object.

# Returns

a new gpg crypto context object.
<!-- struct Header -->
A message or mime-part header.

# Implements

[`HeaderExt`](trait.HeaderExt.html)
<!-- trait HeaderExt -->
Trait containing all `Header` methods.

# Implementors

[`Header`](struct.Header.html)
<!-- trait HeaderExt::fn format_addrlist -->
Parses the `value` and then re-formats it to conform to the formatting options,
folding the value if necessary.
## `options`
a `FormatOptions` or `None`
## `value`
a Sender, From, Reply-To, To, Cc, or Bcc header value
## `charset`
a charset (note: unused)

# Returns

a newly allocated string containing the reformatted value.
<!-- trait HeaderExt::fn format_content_disposition -->
Parses the `value` and then re-formats it to conform to the formatting options,
folding the value if necessary.
## `options`
a `FormatOptions` or `None`
## `value`
a Content-Disposition header value
## `charset`
a charset (note: unused)

# Returns

a newly allocated string containing the reformatted value.
<!-- trait HeaderExt::fn format_content_type -->
Parses the `value` and then re-formats it to conform to the formatting options,
folding the value if necessary.
## `options`
a `FormatOptions` or `None`
## `value`
a Content-Type header value
## `charset`
a charset (note: unused)

# Returns

a newly allocated string containing the reformatted value.
<!-- trait HeaderExt::fn format_default -->
Parses the `value` and then re-formats it to conform to the formatting options,
folding the value if necessary.
## `options`
a `FormatOptions` or `None`
## `value`
a header value
## `charset`
a charset to use when encoding the `value`

# Returns

a newly allocated string containing the reformatted value.
<!-- trait HeaderExt::fn format_message_id -->
Parses the `value` and then re-formats it to conform to the formatting options,
folding the value if necessary.
## `options`
a `FormatOptions` or `None`
## `value`
a Message-Id or Content-Id header value
## `charset`
a charset (note: unused)

# Returns

a newly allocated string containing the reformatted value.
<!-- trait HeaderExt::fn format_received -->
Parses the `value` and then re-formats it to conform to the formatting options,
folding the value if necessary.
## `options`
a `FormatOptions` or `None`
## `value`
a Received header value
## `charset`
a charset (note: unused)

# Returns

a newly allocated string containing the reformatted value.
<!-- trait HeaderExt::fn format_references -->
Parses the `value` and then re-formats it to conform to the formatting options,
folding the value if necessary.
## `options`
a `FormatOptions` or `None`
## `value`
a References or In-Reply-To header value
## `charset`
a charset (note: unused)

# Returns

a newly allocated string containing the reformatted value.
<!-- trait HeaderExt::fn get_name -->
Gets the header's name.

# Returns

the header name or `None` if invalid.
<!-- trait HeaderExt::fn get_offset -->
Gets the header's stream offset if known.

# Returns

the header offset or %-1 if unknown.
<!-- trait HeaderExt::fn get_raw_name -->
Gets the header's raw name. The raw header name is the complete string up to
(but not including) the ':' separating the header's name from its value. This
string may be different from the value returned by `HeaderExt::get_name`
if the parsed message's header contained trailing whitespace after the header
name, such as: "Subject : this is the subject\r\n".

# Returns

the raw header name.
<!-- trait HeaderExt::fn get_raw_value -->
Gets the header's raw (folded) value.

# Returns

the header value or `None` if invalid.
<!-- trait HeaderExt::fn get_value -->
Gets the header's unfolded value.

# Returns

the header's decoded value or `None` if invalid.
<!-- trait HeaderExt::fn set_raw_value -->
Sets the header's raw value.
## `raw_value`
the raw value
<!-- trait HeaderExt::fn set_value -->
Sets the header's decoded value.
## `options`
a `FormatOptions` or `None`
## `value`
the new header value
## `charset`
a charset
<!-- trait HeaderExt::fn write_to_stream -->
Write the header to the specified stream.
## `options`
a `FormatOptions` or `None`
## `stream`
a `Stream`

# Returns

the number of bytes written, or %-1 on fail.
<!-- struct HeaderList -->
A list of message or mime-part headers.

# Implements

[`HeaderListExt`](trait.HeaderListExt.html)
<!-- trait HeaderListExt -->
Trait containing all `HeaderList` methods.

# Implementors

[`HeaderList`](struct.HeaderList.html)
<!-- impl HeaderList::fn new -->
Creates a new `HeaderList` object.
## `options`
a `ParserOptions` or `None`

# Returns

a new header list object.
<!-- trait HeaderListExt::fn append -->
Appends a header. If `value` is `None`, a space will be set aside for it
(useful for setting the order of headers before values can be
obtained for them) otherwise the header will be unset.
## `name`
header name
## `value`
header value
## `charset`
a charset
<!-- trait HeaderListExt::fn clear -->
Removes all of the headers from the `HeaderList`.
<!-- trait HeaderListExt::fn contains -->
Checks whether or not a header exists.
## `name`
header name

# Returns

`true` if the specified header exists or `false` otherwise.
<!-- trait HeaderListExt::fn get_count -->
Gets the number of headers contained within the header list.

# Returns

the number of headers in the header list.
<!-- trait HeaderListExt::fn get_header -->
Gets the first header with the specified name.
## `name`
header name

# Returns

a `Header` for the specified `name`.
<!-- trait HeaderListExt::fn get_header_at -->
Gets the header at the specified `index` within the list.
## `index`
the 0-based index of the header

# Returns

the header at position `index`.
<!-- trait HeaderListExt::fn prepend -->
Prepends a header. If `value` is `None`, a space will be set aside
for it (useful for setting the order of headers before values can
be obtained for them) otherwise the header will be unset.
## `name`
header name
## `value`
header value
## `charset`
a charset
<!-- trait HeaderListExt::fn remove -->
Remove the first instance of the specified header.
## `name`
header name

# Returns

`true` if the header was successfully removed or `false` if
the specified header could not be found.
<!-- trait HeaderListExt::fn remove_at -->
Removes the header at the specified `index` from `self`.
## `index`
the 0-based index of the header to remove
<!-- trait HeaderListExt::fn set -->
Set the value of the specified header. If `value` is `None` and the
header, `name`, had not been previously set, a space will be set
aside for it (useful for setting the order of headers before values
can be obtained for them) otherwise the header will be unset.

Note: If there are multiple headers with the specified field name,
the first instance of the header will be replaced and further
instances will be removed.
## `name`
header name
## `value`
header value
## `charset`
a charset
<!-- trait HeaderListExt::fn to_string -->
Allocates a string buffer containing the raw rfc822 headers
contained in `self`.
## `options`
a `FormatOptions` or `None`

# Returns

a string containing the header block.
<!-- trait HeaderListExt::fn write_to_stream -->
Write the headers to a stream.
## `options`
a `FormatOptions` or `None`
## `stream`
output stream

# Returns

the number of bytes written or %-1 on fail.
<!-- struct InternetAddress -->
An RFC 2822 Address object.

# Implements

[`InternetAddressExt`](trait.InternetAddressExt.html)
<!-- trait InternetAddressExt -->
Trait containing all `InternetAddress` methods.

# Implementors

[`InternetAddressMailbox`](struct.InternetAddressMailbox.html), [`InternetAddress`](struct.InternetAddress.html)
<!-- trait InternetAddressExt::fn get_charset -->
Gets the charset to be used when encoding the name of the mailbox or group.

# Returns

the charset to be used when encoding the name of the
mailbox or group if available or `None` otherwise.
<!-- trait InternetAddressExt::fn get_name -->
Gets the display name of the `InternetAddress`.

# Returns

the name of the mailbox or group in a form suitable
for display if available or `None` otherwise. If the name is available,
the returned string will be in UTF-8.
<!-- trait InternetAddressExt::fn set_charset -->
Set the charset to use for encoding the name of the mailbox or group.
## `charset`
the charset to use when encoding the name or `None` to use the defaults
<!-- trait InternetAddressExt::fn set_name -->
Set the display name of the `InternetAddress`.

Note: The `name` string should be in UTF-8.
## `name`
the display name for the address group or mailbox
<!-- trait InternetAddressExt::fn to_string -->
Allocates a string containing the contents of the `InternetAddress`
object.
## `options`
a `FormatOptions` or `None`
## `encode`
`true` if the address should be rfc2047 encoded

# Returns

the `InternetAddress` object as an allocated string in
rfc822 format.
<!-- struct InternetAddressList -->
A collection of `InternetAddress` objects.

# Implements

[`InternetAddressListExt`](trait.InternetAddressListExt.html)
<!-- trait InternetAddressListExt -->
Trait containing all `InternetAddressList` methods.

# Implementors

[`InternetAddressList`](struct.InternetAddressList.html)
<!-- impl InternetAddressList::fn new -->
Creates a new `InternetAddressList`.

# Returns

a new `InternetAddressList`.
<!-- impl InternetAddressList::fn parse -->
Construct a list of internet addresses from the given string.
## `options`
a `ParserOptions` or `None`
## `str`
a string containing internet addresses

# Returns

a `InternetAddressList` or `None` if the
input string does not contain any addresses.
<!-- trait InternetAddressListExt::fn add -->
Adds an `InternetAddress` to the `InternetAddressList`.
## `ia`
a `InternetAddress`

# Returns

the index of the added `InternetAddress`.
<!-- trait InternetAddressListExt::fn append -->
Adds all of the addresses in `append` to `self`.
## `append`
a `InternetAddressList`
<!-- trait InternetAddressListExt::fn clear -->
Clears the list of addresses.
<!-- trait InternetAddressListExt::fn contains -->
Checks whether or not the specified `InternetAddress` is contained
within the `InternetAddressList`.
## `ia`
a `InternetAddress`

# Returns

`true` if the specified `InternetAddress` is contained
within the specified `InternetAddressList` or `false` otherwise.
<!-- trait InternetAddressListExt::fn encode -->
Writes the rfc2047-encoded rfc822 formatted addresses in `self` to
`str`, folding appropriately.
## `options`
a `FormatOptions` or `None`
## `str`
string to write to
<!-- trait InternetAddressListExt::fn get_address -->
Gets the `InternetAddress` at the specified index.
## `index`
index of `InternetAddress` to get

# Returns

the `InternetAddress` at the specified
index or `None` if the index is out of range.
<!-- trait InternetAddressListExt::fn index_of -->
Gets the index of the specified `InternetAddress` inside the
`InternetAddressList`.
## `ia`
a `InternetAddress`

# Returns

the index of the requested `InternetAddress` within the
`InternetAddressList` or %-1 if it is not contained within the
`InternetAddressList`.
<!-- trait InternetAddressListExt::fn insert -->
Inserts an `InternetAddress` into the `InternetAddressList` at the
specified index.
## `index`
index to insert at
## `ia`
a `InternetAddress`
<!-- trait InternetAddressListExt::fn length -->
Gets the length of the list.

# Returns

the number of `InternetAddress` objects in the list.
<!-- trait InternetAddressListExt::fn prepend -->
Inserts all of the addresses in `prepend` to the beginning of `self`.
## `prepend`
a `InternetAddressList`
<!-- trait InternetAddressListExt::fn remove -->
Removes an `InternetAddress` from the `InternetAddressList`.
## `ia`
a `InternetAddress`

# Returns

`true` if the specified `InternetAddress` was removed or
`false` otherwise.
<!-- trait InternetAddressListExt::fn remove_at -->
Removes an `InternetAddress` from the `InternetAddressList` at the
specified index.
## `index`
index to remove

# Returns

`true` if an `InternetAddress` was removed or `false`
otherwise.
<!-- trait InternetAddressListExt::fn set_address -->
Sets the `InternetAddress` at the specified index to `ia`.
## `index`
index of `InternetAddress` to set
## `ia`
a `InternetAddress`
<!-- trait InternetAddressListExt::fn to_string -->
Allocates a string buffer containing the rfc822 formatted addresses
in `self`.
## `options`
a `FormatOptions` or `None`
## `encode`
`true` if the address should be rfc2047 encoded

# Returns

a string containing the list of addresses in rfc822
format or `None` if no addresses are contained in the list.
<!-- struct InternetAddressMailbox -->
An RFC 2822 Mailbox address.

# Implements

[`InternetAddressMailboxExt`](trait.InternetAddressMailboxExt.html), [`InternetAddressExt`](trait.InternetAddressExt.html)
<!-- trait InternetAddressMailboxExt -->
Trait containing all `InternetAddressMailbox` methods.

# Implementors

[`InternetAddressMailbox`](struct.InternetAddressMailbox.html)
<!-- impl InternetAddressMailbox::fn new -->
Creates a new `InternetAddress` object with the specified `name` and
`addr`.
## `name`
person's name
## `addr`
person's address

# Returns

a new `InternetAddressMailbox` object.

Note: The `name` string should be in UTF-8.
<!-- trait InternetAddressMailboxExt::fn get_addr -->
Gets the addr-spec of the internet address mailbox.

# Returns

the addr-spec string.
<!-- trait InternetAddressMailboxExt::fn get_idn_addr -->
Gets the IDN ascii-encoded addr-spec.

# Returns

the encoded addr-spec string.
<!-- trait InternetAddressMailboxExt::fn set_addr -->
Set the mailbox address.
## `addr`
contact's email address
<!-- struct Message -->
A MIME Message object.

# Implements

[`MessageExt`](trait.MessageExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait MessageExt -->
Trait containing all `Message` methods.

# Implementors

[`Message`](struct.Message.html)
<!-- impl Message::fn new -->
If `pretty_headers` is `true`, then the standard rfc822 headers are
initialized so as to put headers in a nice friendly order. This is
strictly a cosmetic thing, so if you are unsure, it is safe to say
no (`false`).
## `pretty_headers`
make pretty headers

# Returns

an empty `Message` object.
<!-- trait MessageExt::fn add_mailbox -->
Add a mailbox of a chosen type to the MIME message.

Note: The `name` (and `addr`) strings should be in UTF-8.
## `type_`
A `AddressType`
## `name`
The name of the mailbox (or `None`)
## `addr`
The address of the mailbox
<!-- trait MessageExt::fn foreach -->
Recursively calls `callback` on each of the mime parts in the mime message.
## `callback`
function to call on each of the mime parts
 contained by the mime message
## `user_data`
user-supplied callback data
<!-- trait MessageExt::fn get_addresses -->
Gets a list of addresses of the specified `type_` from the `self`.
## `type_`
A `AddressType`

# Returns

a list of addresses of the specified
`type_` from the `self`.
<!-- trait MessageExt::fn get_all_recipients -->
Gets the complete list of recipients for `self`.

# Returns

a newly allocated `InternetAddressList`
containing all recipients of the message or `None` if no recipients
are set.
<!-- trait MessageExt::fn get_autocrypt_gossip_headers -->
Creates a new `AutocryptHeaderList` of relevant headers of the
given type based on the recipient(s) of an e-mail message.

Returns the same object as
`g_mime_message_get_autocrypt_gossip_headers_with_inner_part` , but
handles decryption and cleanup automatically.

`flags` and `session_key` are passed through to
`MultipartEncryptedExt::decrypt`, as needed.

If the message is not actually an encrypted message, returns `None`:
it should be ignored for purposes of evaluating gossip.

If decryption fails, returns `None`. In this case, an exception
will be set on `err` to provide information about the decryption
failure.
## `now`
a `glib::DateTime` object, or `None`
## `flags`
a `DecryptFlags`, to be used during decryption
## `session_key`
session key to use or `None`

# Returns

a new `AutocryptHeaderList` object,
or `None` on error.
<!-- trait MessageExt::fn get_autocrypt_gossip_headers_from_inner_part -->
Creates a new `AutocryptHeaderList` of relevant headers of the
given type based on the recipient(s) of an e-mail message.

You must pass the decrypted inner part of the message to this
function, since Autocrypt-Gossip headers are only stored within the
encrypted layer.

If you don't already have the decrypted inner part available to
you, you probably want to use
`MessageExt::get_autocrypt_gossip_headers` instead.

Each header in the returned list will:

 - have a valid address
 - be of the type requested
 - be complete

If no Autocrypt header is found for a recipient, no
`AutocryptHeader` will be in the list associated with that e-mail address.

Note that the following types of Autocrypt headers will not be
returned by this function:

 - headers of an unrequested type
 - headers that do not match an address in "From:"
 - unparseable headers
 - headers with unknown critical attributes
 - duplicate valid headers for a given address

On error (e.g. if this version of GMime cannot handle the requested
Autocrypt type, or if a parameter is missing or malformed), returns
`None`

The returned Autocrypt headers will have their effective_date set
to the earliest of either:

- the Date: header of the message or
- `now` (or the current time, if `now` is `None`)
## `now`
a `glib::DateTime` object, or `None`
## `inner_part`
a `Object` which is the cleartext part of the inner message

# Returns

a new `AutocryptHeaderList` object, or `None` on error.
<!-- trait MessageExt::fn get_autocrypt_header -->
Creates a new `AutocryptHeader` base on the relevant Autocrypt
header associated with the sender of an e-mail message.

If the message has no sender in the From: field, or has more than
one sender, then this function will return `None`. Autocrypt should
ignore the message entirely.

If there is one sender, but no single Autocrypt header is found
that matches that e-mail address, a `AutocryptHeader` will be
returned for the sender, but it will be incomplete (see
`AutocryptHeaderExt::is_complete`).

Note that the following types of Autocrypt headers will not be
returned by this function:

 - headers that do not match an address in "From:"
 - unparseable headers
 - headers with unknown critical attributes
 - duplicate valid headers for the sender's address

The returned Autocrypt headers will have their effective_date set
to the earliest of either:

- the Date: header of the message or
- `now` (or the current time, if `now` is `None`)
## `now`
a `glib::DateTime` object, or `None`

# Returns

a new `AutocryptHeaderList` object,
or `None` if the message should be ignored for purposes of
Autocrypt.
<!-- trait MessageExt::fn get_bcc -->
Gets combined list of parsed addresses in the Bcc header(s).

# Returns

the parsed list of addresses in the Bcc header(s).
<!-- trait MessageExt::fn get_body -->
Attempts to identify the MIME part containing the body of the
message.

# Returns

a `Object` containing the textual
content that appears to be the main body of the message.

Note: This function is NOT guaranteed to always work as it
makes some assumptions that are not necessarily true. It is
recommended that you traverse the MIME structure yourself.
<!-- trait MessageExt::fn get_cc -->
Gets combined list of parsed addresses in the Cc header(s).

# Returns

the parsed list of addresses in the Cc header(s).
<!-- trait MessageExt::fn get_date -->
Gets the parsed date and time value from the Date header.

# Returns

a `glib::DateTime` on success or `None` if the date could not be parsed.
<!-- trait MessageExt::fn get_from -->
Gets the parsed list of addresses in the From header.

# Returns

the parsed list of addresses in the From header.
<!-- trait MessageExt::fn get_message_id -->
Gets the Message-Id header of `self`.

# Returns

the Message-Id of a message.
<!-- trait MessageExt::fn get_mime_part -->
Gets the toplevel MIME part contained within `self`.

# Returns

the toplevel MIME part of `self`.
<!-- trait MessageExt::fn get_reply_to -->
Gets the parsed list of addresses in the Reply-To header.

# Returns

the parsed list of addresses in the Reply-To header.
<!-- trait MessageExt::fn get_sender -->
Gets the parsed list of addresses in the Sender header.

# Returns

the parsed list of addresses in the Sender header.
<!-- trait MessageExt::fn get_subject -->
Gets the subject of the `self`.

# Returns

the subject of the `self` in a form suitable for display
or `None` if no subject is set. If not `None`, the returned string
will be in UTF-8.
<!-- trait MessageExt::fn get_to -->
Gets combined list of parsed addresses in the To header(s).

# Returns

the parsed list of addresses in the To header(s).
<!-- trait MessageExt::fn partial_split_message -->
Splits `self` into an array of `Message` objects each
containing a single `MessagePartial` object containing
`max_size` bytes or fewer. `nparts` is set to the number of
`MessagePartial` objects created.
## `max_size`
max size
## `nparts`
number of parts

# Returns

an array of `Message` objects and
sets `nparts` to the number of messages returned or `None` on fail.
<!-- trait MessageExt::fn set_date -->
Sets the Date header on a MIME Message.
## `date`
a date to be used in the Date header
<!-- trait MessageExt::fn set_message_id -->
Set the Message-Id on a message.
## `message_id`
message-id (addr-spec portion)
<!-- trait MessageExt::fn set_mime_part -->
Set the root-level MIME part of the message.
## `mime_part`
The root-level MIME Part
<!-- trait MessageExt::fn set_subject -->
Set the subject of a `self`.

Note: The `subject` string should be in UTF-8.
## `subject`
Subject string
## `charset`
The charset to use for encoding the subject or `None` to use the default
<!-- struct MessagePart -->
A message/rfc822 or message/news MIME part.

# Implements

[`MessagePartExt`](trait.MessagePartExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait MessagePartExt -->
Trait containing all `MessagePart` methods.

# Implementors

[`MessagePart`](struct.MessagePart.html)
<!-- impl MessagePart::fn new -->
Creates a new MIME message part object with a default content-type
of message/`subtype`.
## `subtype`
message subtype or `None` for "rfc822"

# Returns

an empty MIME message part object with a default
content-type of message/`subtype`.
<!-- impl MessagePart::fn new_with_message -->
Creates a new MIME message part object with a default content-type
of message/`subtype` containing `message`.
## `subtype`
message subtype or `None` for "rfc822"
## `message`
message

# Returns

a MIME message part object with a default content-type of
message/`subtype` containing `message`.
<!-- trait MessagePartExt::fn get_message -->
Gets the message object on the message part object `self`.

# Returns

the message part contained within `self`.
<!-- trait MessagePartExt::fn set_message -->
Sets the `message` object on the message part object `self`.
## `message`
message
<!-- struct MessagePartial -->
A message/partial MIME part.

# Implements

[`MessagePartialExt`](trait.MessagePartialExt.html), [`PartExt`](trait.PartExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait MessagePartialExt -->
Trait containing all `MessagePartial` methods.

# Implementors

[`MessagePartial`](struct.MessagePartial.html)
<!-- impl MessagePartial::fn new -->
Creates a new MIME message/partial object.
## `id`
message/partial part id
## `number`
message/partial part number
## `total`
total number of message/partial parts

# Returns

an empty MIME message/partial object.
<!-- impl MessagePartial::fn reconstruct_message -->
Reconstructs the GMimeMessage from the given message/partial parts
in `partials`.
## `partials`
an array of message/partial mime parts
## `num`
the number of elements in `partials`

# Returns

a GMimeMessage object on success or `None`
on fail.
<!-- trait MessagePartialExt::fn get_id -->
Gets the message/partial id parameter value.

# Returns

the message/partial id or `None` on fail.
<!-- trait MessagePartialExt::fn get_number -->
Gets the message/partial part number.

# Returns

the message/partial part number or %-1 on fail.
<!-- trait MessagePartialExt::fn get_total -->
Gets the total number of message/partial parts needed to
reconstruct the original message.

# Returns

the total number of message/partial parts needed to
reconstruct the original message or -1 on fail.
<!-- struct Multipart -->
A base MIME multipart object.

# Implements

[`MultipartExt`](trait.MultipartExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait MultipartExt -->
Trait containing all `Multipart` methods.

# Implementors

[`MultipartEncrypted`](struct.MultipartEncrypted.html), [`MultipartSigned`](struct.MultipartSigned.html), [`Multipart`](struct.Multipart.html)
<!-- impl Multipart::fn new -->
Creates a new MIME multipart object with a default content-type of
multipart/mixed.

# Returns

an empty MIME multipart object with a default content-type of
multipart/mixed.
<!-- impl Multipart::fn new_with_subtype -->
Creates a new MIME multipart object with a content-type of
multipart/`subtype`.
## `subtype`
content-type subtype

# Returns

an empty MIME multipart object with a content-type of
multipart/`subtype`.
<!-- trait MultipartExt::fn add -->
Appends a mime part to `self`.
## `part`
a `Object`
<!-- trait MultipartExt::fn clear -->
Removes all subparts from `self`.
<!-- trait MultipartExt::fn contains -->
Checks if `part` is contained within `self`.
## `part`
a `Object`

# Returns

`true` if `part` is a subpart of `self` or `false`
otherwise.
<!-- trait MultipartExt::fn foreach -->
Recursively calls `callback` on each of `self`'s subparts.
## `callback`
function to call for each of `self`'s
 subparts.
## `user_data`
user-supplied callback data
<!-- trait MultipartExt::fn get_boundary -->
Gets the boundary on the multipart. If the internal boundary is
`None`, then an auto-generated boundary will be set on the multipart
and returned.

# Returns

the boundary on the multipart.
<!-- trait MultipartExt::fn get_count -->
Gets the number of parts contained within `self`.

# Returns

the number of parts contained within `self`.
<!-- trait MultipartExt::fn get_epilogue -->
Gets the epilogue on the multipart.

# Returns

a pointer to the epilogue string on the multipart.
<!-- trait MultipartExt::fn get_part -->
Gets the part at the specified `index` within the multipart.
## `index`
the 0-based index of the part

# Returns

the part at position `index`.
<!-- trait MultipartExt::fn get_prologue -->
Gets the prologue on the multipart.

# Returns

a pointer to the prologue string on the multipart.
<!-- trait MultipartExt::fn get_subpart_from_content_id -->
Gets the mime part with the content-id `content_id` from the
multipart `self`.
## `content_id`
the content id of the part to look for

# Returns

the `Object` whose content-id matches
the search string, or `None` if a match cannot be found.
<!-- trait MultipartExt::fn index_of -->
Gets the 0-based index of `part` within `self`.
## `part`
a `Object`

# Returns

the 0-based index of `part` within `self` or %-1 if not found.
<!-- trait MultipartExt::fn insert -->
Inserts `part` into `self` at the specified `index`.
## `index`
the 0-based index to insert the part
## `part`
a `Object`
<!-- trait MultipartExt::fn remove -->
Removes the specified `part` from `self`.
## `part`
a `Object`

# Returns

`true` if the part was removed or `false` otherwise.
<!-- trait MultipartExt::fn remove_at -->
Removes the part at the specified `index` from `self`.
## `index`
the 0-based index of the part to remove

# Returns

the mime part that was removed or `None`
if the part was not contained within the multipart.
<!-- trait MultipartExt::fn replace -->
Replaces the part at the specified `index` within `self` with
`replacement`.
## `index`
the 0-based index of the part to replace
## `replacement`
a `Object` to use as the replacement

# Returns

the part that was replaced or `None`
if the part was not contained within the multipart.
<!-- trait MultipartExt::fn set_boundary -->
Sets `boundary` as the boundary on the multipart. If `boundary` is
`None`, then a boundary will be auto-generated for you.
## `boundary`
boundary or `None` to autogenerate one
<!-- trait MultipartExt::fn set_epilogue -->
Sets the epilogue on the multipart.
## `epilogue`
epilogue
<!-- trait MultipartExt::fn set_prologue -->
Sets the prologue on the multipart.
## `prologue`
prologue
<!-- struct MultipartEncrypted -->
A multipart/encrypted MIME part.

# Implements

[`MultipartEncryptedExt`](trait.MultipartEncryptedExt.html), [`MultipartExt`](trait.MultipartExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait MultipartEncryptedExt -->
Trait containing all `MultipartEncrypted` methods.

# Implementors

[`MultipartEncrypted`](struct.MultipartEncrypted.html)
<!-- impl MultipartEncrypted::fn new -->
Creates a new MIME multipart/encrypted object.

# Returns

an empty MIME multipart/encrypted object.
<!-- impl MultipartEncrypted::fn encrypt -->
Attempts to encrypt (and conditionally sign) the `entity` MIME part
to the public keys of `recipients` using the `ctx` encryption
context. If successful, a new multipart/encrypted object is returned.
## `ctx`
a `CryptoContext`
## `entity`
MIME part to encrypt
## `sign`
`true` if the content should also be signed or `false` otherwise
## `userid`
user id to use for signing (only used if `sign` is `true`)
## `flags`
a `EncryptFlags`
## `recipients`
an array of recipients to encrypt to

# Returns

a new `MultipartEncrypted` object on success
or `None` on fail. If encrypting fails, an exception will be set on `err` to provide
information as to why the failure occurred.
<!-- trait MultipartEncryptedExt::fn decrypt -->
Attempts to decrypt the encrypted MIME part contained within the
multipart/encrypted object `self`.

When non-`None`, `session_key` should be a `None`-terminated string,
such as the one returned by `DecryptResultExt::get_session_key`
from a previous decryption. If the `session_key` is not valid, decryption
will fail.

If `result` is non-`None`, then on a successful decrypt operation, it will be
updated to point to a newly-allocated `DecryptResult` with signature
status information as well as a list of recipients that the part was
encrypted to.
## `flags`
a `DecryptFlags`
## `session_key`
session key to use or `None`
## `result`
a `DecryptResult`

# Returns

the decrypted MIME part on success or
`None` on fail. If the decryption fails, an exception will be set on
`err` to provide information as to why the failure occurred.
<!-- struct MultipartSigned -->
A multipart/signed MIME part.

# Implements

[`MultipartSignedExt`](trait.MultipartSignedExt.html), [`MultipartExt`](trait.MultipartExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait MultipartSignedExt -->
Trait containing all `MultipartSigned` methods.

# Implementors

[`MultipartSigned`](struct.MultipartSigned.html)
<!-- impl MultipartSigned::fn new -->
Creates a new MIME multipart/signed object.

# Returns

an empty MIME multipart/signed object.
<!-- impl MultipartSigned::fn sign -->
Attempts to sign the `content` MIME part with `userid`'s private key
using the `ctx` signing context. If successful, a new multipart/signed
object is returned.
## `ctx`
a `CryptoContext`
## `entity`
MIME part to sign
## `userid`
user id to sign with

# Returns

a new `MultipartSigned` object on success
or `None` on fail. If signing fails, an exception will be set on `err` to provide
information as to why the failure occurred.
<!-- trait MultipartSignedExt::fn verify -->
Attempts to verify the signed MIME part contained within the
multipart/signed object `self`.
## `flags`
a `VerifyFlags`

# Returns

a new `SignatureList` object on
success or `None` on fail. If the verification fails, an exception
will be set on `err` to provide information as to why the failure
occurred.
<!-- enum NewLineFormat -->
There are two commonly used line-endings used by modern Operating Systems.
Unix-based systems such as Linux and Mac OS use a single character ('\n' aka LF)
to represent the end of line where-as Windows (or DOS) uses a sequence of two
characters ("\r\n" aka CRLF). Most text-based network protocols such as SMTP,
POP3, and IMAP use the CRLF sequence as well.
<!-- enum NewLineFormat::variant Unix -->
The Unix New-Line format ("\n").
<!-- enum NewLineFormat::variant Dos -->
The DOS New-Line format ("\r\n").
<!-- struct Object -->
Base class for all MIME parts.

# Implements

[`ObjectExt`](trait.ObjectExt.html)
<!-- trait ObjectExt -->
Trait containing all `Object` methods.

# Implementors

[`MessagePart`](struct.MessagePart.html), [`Message`](struct.Message.html), [`Multipart`](struct.Multipart.html), [`Object`](struct.Object.html), [`Part`](struct.Part.html)
<!-- impl Object::fn new -->
Performs a lookup of registered `Object` subclasses, registered
using `Object::register_type`, to find an appropriate class
capable of handling MIME parts of the specified Content-Type. If no
class has been registered to handle that type, it looks for a
registered class that can handle `content_type`'s media type. If
that also fails, then it will use the generic part class,
`Part`.
## `options`
a `ParserOptions` or `None`
## `content_type`
a `ContentType` object

# Returns

an appropriate `Object` registered to handle MIME
parts appropriate for `content_type`.
<!-- impl Object::fn new_type -->
Performs a lookup of registered `Object` subclasses, registered
using `Object::register_type`, to find an appropriate class
capable of handling MIME parts of type `type_`/`subtype`. If no class
has been registered to handle that type, it looks for a registered
class that can handle `type_`. If that also fails, then it will use
the generic part class, `Part`.
## `options`
a `ParserOptions` or `None`
## `type_`
mime type
## `subtype`
mime subtype

# Returns

an appropriate `Object` registered to handle mime-types
of `type_`/`subtype`.
<!-- impl Object::fn register_type -->
Registers the object type `object_type` for use with the
`Object::new_type` convenience function.

Note: You may use the wildcard "*" to match any type and/or
subtype.
## `type_`
mime type
## `subtype`
mime subtype
## `object_type`
object type
<!-- trait ObjectExt::fn append_header -->
Appends a new header to the header list.
## `header`
header name
## `value`
header value
## `charset`
a charset
<!-- trait ObjectExt::fn encode -->
Calculates and sets the most efficient Content-Transfer-Encoding
for this `Object` and all child parts based on the `constraint`
provided.
## `constraint`
a `EncodingConstraint`
<!-- trait ObjectExt::fn get_content_disposition -->
Gets the `ContentDisposition` for the specified MIME object.

# Returns

the `ContentDisposition` set on the
MIME object.
<!-- trait ObjectExt::fn get_content_disposition_parameter -->
Gets the value of the Content-Disposition parameter specified by
`name`, or `None` if the parameter does not exist.
## `name`
parameter name

# Returns

the value of the requested content-disposition param or
`None` if the param doesn't exist. If the param is set, the returned
string will be in UTF-8.
<!-- trait ObjectExt::fn get_content_id -->
Gets the Content-Id of the MIME object or NULL if one is not set.

# Returns

a const pointer to the Content-Id header.
<!-- trait ObjectExt::fn get_content_type -->
Gets the `ContentType` object for the given MIME object or
`None` on fail.

# Returns

the content-type object for the specified
MIME object.
<!-- trait ObjectExt::fn get_content_type_parameter -->
Gets the value of the content-type param `name` set on the MIME part
`self`.
## `name`
param name

# Returns

the value of the requested content-type param or `None` if
the param doesn't exist. If the param is set, the returned string
will be in UTF-8.
<!-- trait ObjectExt::fn get_disposition -->
Gets the MIME object's disposition if set or `None` otherwise.

# Returns

the disposition string which is probably one of
`GMIME_DISPOSITION_ATTACHMENT` or `GMIME_DISPOSITION_INLINE`.
<!-- trait ObjectExt::fn get_header -->
Gets the value of the first header with the specified name.
## `header`
header name

# Returns

the value of the requested header if it
exists or `None` otherwise.
<!-- trait ObjectExt::fn get_header_list -->
Get the header list for `self`.

# Returns

the `HeaderList` for `self`. Do not
free this pointer when you are done with it.
<!-- trait ObjectExt::fn get_headers -->
Allocates a string buffer containing all of the MIME object's raw
headers.
## `options`
a `FormatOptions` or `None`

# Returns

an allocated string containing all of the raw MIME headers.

Note: The returned string will not be suitable for display.
<!-- trait ObjectExt::fn prepend_header -->
Prepends a new header to the header list.
## `header`
header name
## `value`
header value
## `charset`
a charset
<!-- trait ObjectExt::fn remove_header -->
Removed the specified header if it exists.
## `header`
header name

# Returns

`true` if the header was removed or `false` if it could not
be found.
<!-- trait ObjectExt::fn set_content_disposition -->
Set the content disposition for the specified mime part and then
serializes it to the Content-Disposition header field.
## `disposition`
a `ContentDisposition` object
<!-- trait ObjectExt::fn set_content_disposition_parameter -->
Add a content-disposition parameter to the specified mime part.

Note: The `name` string should be in US-ASCII while the `value`
string should be in UTF-8.
## `name`
parameter name
## `value`
parameter value
<!-- trait ObjectExt::fn set_content_id -->
Sets the Content-Id of the MIME object.
## `content_id`
content-id (addr-spec portion)
<!-- trait ObjectExt::fn set_content_type -->
Sets the content-type for the specified MIME object and then
serializes it to the Content-Type header field.
## `content_type`
a `ContentType` object
<!-- trait ObjectExt::fn set_content_type_parameter -->
Sets the content-type param `name` to the value `value`.

Note: The `name` string should be in US-ASCII while the `value`
string should be in UTF-8.
## `name`
param name
## `value`
param value
<!-- trait ObjectExt::fn set_disposition -->
Sets the disposition to `disposition` which may be one of
`GMIME_DISPOSITION_ATTACHMENT` or `GMIME_DISPOSITION_INLINE` or, by
your choice, any other string which would indicate how the MIME
part should be displayed by the MUA.
## `disposition`
disposition ("attachment" or "inline")
<!-- trait ObjectExt::fn set_header -->
Sets a header to the specified value.
## `header`
header name
## `value`
header value
## `charset`
a charset
<!-- trait ObjectExt::fn to_string -->
Allocates a string buffer containing the contents of `self`.
## `options`
a `FormatOptions` or `None`

# Returns

an allocated string containing the contents of the mime
object.
<!-- trait ObjectExt::fn write_to_stream -->
Write the contents of the MIME object to `stream`.
## `options`
a `FormatOptions` or `None`
## `stream`
stream

# Returns

the number of bytes written or %-1 on fail.
<!-- enum OpenPGPData -->
The type of OpenPGP data found, if any.
<!-- enum OpenPGPData::variant None -->
No OpenPGP data found.
<!-- enum OpenPGPData::variant Encrypted -->
The content contains OpenPGP encrypted data.
<!-- enum OpenPGPData::variant Signed -->
The content contains OpenPGP signed data.
<!-- enum OpenPGPData::variant PublicKey -->
The content contains OpenPGP public key data.
<!-- enum OpenPGPData::variant PrivateKey -->
The content contains OpenPGP private key data.
<!-- struct Param -->
A parameter name/value pair as used in the Content-Type and Content-Disposition headers.

# Implements

[`ParamExt`](trait.ParamExt.html)
<!-- trait ParamExt -->
Trait containing all `Param` methods.

# Implementors

[`Param`](struct.Param.html)
<!-- trait ParamExt::fn get_charset -->
Gets the charset used for encoding the parameter.

# Returns

the charset used for encoding the parameter.
<!-- trait ParamExt::fn get_encoding_method -->
Gets the encoding method used for encoding the parameter.

# Returns

the encoding method used for encoding the parameter.
<!-- trait ParamExt::fn get_lang -->
Gets the language specifier used for encoding the parameter.

# Returns

the language specifier used for encoding the parameter.
<!-- trait ParamExt::fn get_name -->
Gets the name of the parameter.

# Returns

the name of the parameter.
<!-- trait ParamExt::fn get_value -->
Gets the value of the parameter.

# Returns

the value of the parameter.
<!-- trait ParamExt::fn set_charset -->
Sets the parameter charset used for encoding the value.
## `charset`
the charset or `None` to use the default
<!-- trait ParamExt::fn set_encoding_method -->
Sets the encoding method used for encoding the value.
## `method`
a `ParamEncodingMethod`
<!-- trait ParamExt::fn set_lang -->
Sets the parameter language specifier used for encoding the value.
## `lang`
the language specifier
<!-- trait ParamExt::fn set_value -->
Sets the parameter value to `value`.
## `value`
the new parameter value
<!-- enum ParamEncodingMethod -->
The MIME specifications specify that the proper method for encoding Content-Type and
Content-Disposition parameter values is the method described in
<a href="https://tools.ietf.org/html/rfc2231">rfc2231`</a>`. However, it is common for
some older email clients to improperly encode using the method described in
<a href="https://tools.ietf.org/html/rfc2047">rfc2047`</a>` instead.
<!-- enum ParamEncodingMethod::variant Default -->
Use the default encoding method set on the `FormatOptions`.
<!-- enum ParamEncodingMethod::variant Rfc2231 -->
Use the encoding method described in rfc2231.
<!-- enum ParamEncodingMethod::variant Rfc2047 -->
Use the encoding method described in rfc2047.
<!-- struct ParamList -->
A list of Content-Type or Content-Disposition parameters.

# Implements

[`ParamListExt`](trait.ParamListExt.html)
<!-- trait ParamListExt -->
Trait containing all `ParamList` methods.

# Implementors

[`ParamList`](struct.ParamList.html)
<!-- impl ParamList::fn new -->
Creates a new Content-Type or Content-Disposition parameter list.

# Returns

a new `ParamList`.
<!-- impl ParamList::fn parse -->
Parses the input string into a parameter list.
## `options`
a `ParserOptions` or `None`
## `str`
a string to parse

# Returns

a new `ParamList`.
<!-- trait ParamListExt::fn clear -->
Clears the list of parameters.
<!-- trait ParamListExt::fn encode -->
Encodes the parameter list into `str`, folding lines if required.
## `options`
a `FormatOptions` or `None`
## `fold`
`true` if the parameter list should be folded; otherwise, `false`
## `str`
the output string buffer
<!-- trait ParamListExt::fn get_parameter -->
Gets the `Param` with the given `name`.
## `name`
the name of the parameter

# Returns

the requested `Param`.
<!-- trait ParamListExt::fn get_parameter_at -->
Gets the `Param` at the specified `index`.
## `index`
the index of the requested parameter

# Returns

the `Param` at the specified index.
<!-- trait ParamListExt::fn length -->
Gets the length of the list.

# Returns

the number of `Param` items in the list.
<!-- trait ParamListExt::fn remove -->
Removes a parameter from the `ParamList`.
## `name`
the name of the parameter

# Returns

`true` if the specified parameter was removed or `false` otherwise.
<!-- trait ParamListExt::fn remove_at -->
Removes a `Param` from the `ParamList` at the specified index.
## `index`
index of the param to remove

# Returns

`true` if a `Param` was removed or `false` otherwise.
<!-- trait ParamListExt::fn set_parameter -->
Sets the specified parameter to `value`.
## `name`
The name of the parameter
## `value`
The parameter value
<!-- struct Parser -->
A MIME parser context.

# Implements

[`ParserExt`](trait.ParserExt.html)
<!-- trait ParserExt -->
Trait containing all `Parser` methods.

# Implementors

[`Parser`](struct.Parser.html)
<!-- impl Parser::fn new -->
Creates a new parser object.

# Returns

a new parser object.
<!-- impl Parser::fn new_with_stream -->
Creates a new parser object preset to parse `stream`.
## `stream`
raw message or part stream

# Returns

a new parser object.
<!-- trait ParserExt::fn construct_message -->
Constructs a MIME message from `self`.
## `options`
a `ParserOptions` or `None`

# Returns

a MIME message or `None` on fail.
<!-- trait ParserExt::fn construct_part -->
Constructs a MIME part from `self`.
## `options`
a `ParserOptions` or `None`

# Returns

a MIME part based on `self` or `None` on
fail.
<!-- trait ParserExt::fn eos -->
Tests the end-of-stream indicator for `self`'s internal stream.

# Returns

`true` on EOS or `false` otherwise.
<!-- trait ParserExt::fn get_format -->
Gets the format that the parser is set to parse.

# Returns

the format that the parser is set to parse.
<!-- trait ParserExt::fn get_headers_begin -->
Gets the stream offset of the beginning of the headers of the most
recently parsed message.

# Returns

the offset of the beginning of the headers of the most
recently parsed message or %-1 on error.
<!-- trait ParserExt::fn get_headers_end -->
Gets the stream offset of the end of the headers of the most
recently parsed message.

# Returns

the offset of the end of the headers of the most recently
parsed message or %-1 on error.
<!-- trait ParserExt::fn get_mbox_marker -->
Gets the mbox-style From-line of the most recently parsed message
(gotten from `ParserExt::construct_message`).

# Returns

the mbox-style From-line of the most recently parsed
message or `None` on error.
<!-- trait ParserExt::fn get_mbox_marker_offset -->
Gets the offset of the most recently parsed mbox-style From-line
(gotten from `ParserExt::construct_message`).

# Returns

the offset of the most recently parsed mbox-style From-line
or %-1 on error.
<!-- trait ParserExt::fn get_persist_stream -->
Gets whether or not the underlying stream is persistent.

# Returns

`true` if the `self` will leave the content on disk or
`false` if it will load the content into memory.
<!-- trait ParserExt::fn get_respect_content_length -->
Gets whether or not `self` is set to use Content-Length for
determining the offset of the end of the message.

# Returns

whether or not `self` is set to use Content-Length for
determining the offset of the end of the message.
<!-- trait ParserExt::fn init_with_stream -->
Initializes `self` to use `stream`.

WARNING: Initializing a parser with a stream is comparable to
selling your soul (`stream`) to the devil (`self`). You are
basically giving the parser complete control of the stream, this
means that you had better not touch the stream so long as the
parser is still using it. This means no reading, writing, seeking,
or resetting of the stream. Anything that will/could change the
current stream's offset is PROHIBITED.

It is also recommended that you not use `StreamExt::tell`
because it will not necessarily give you the current `self` offset
since `self` handles its own internal read-ahead buffer. Instead,
it is recommended that you use `ParserExt::tell` if you have a
reason to need the current offset of the `self`.
## `stream`
raw message or part stream
<!-- trait ParserExt::fn set_format -->
Sets the format that the parser should expect the stream to be in.
## `format`
a `Format`
<!-- trait ParserExt::fn set_header_regex -->
Sets the regular expression pattern `regex` on `self`. Whenever a
header matching the pattern `regex` is parsed, `header_cb` is called
with `user_data` as the user_data argument.

If `regex` is `None`, then the previously registered regex callback
is unregistered and no new callback is set.
## `regex`
regular expression
## `header_cb`
callback function
## `user_data`
user data
<!-- trait ParserExt::fn set_persist_stream -->
Sets whether or not the `self`'s underlying stream is persistent.

If `persist` is `true`, the `self` will attempt to construct
messages/parts whose content will remain on disk rather than being
loaded into memory so as to reduce memory usage. This is the default.

If `persist` is `false`, the `self` will always load message content
into memory.

Note: This attribute only serves as a hint to the `self`. If the
underlying stream does not support seeking, then this attribute
will be ignored.

By default, this feature is enabled if the underlying stream is seekable.
## `persist`
persist attribute
<!-- trait ParserExt::fn set_respect_content_length -->
Sets whether or not `self` should respect Content-Length headers
when deciding where to look for the start of the next message. Only
used when the parser is also set to scan for From-lines.

Most notably useful when parsing broken Solaris mbox files (See
http://www.jwz.org/doc/content-length.html for details).

By default, this feature is disabled.
## `respect_content_length`
`true` if the parser should use Content-Length headers or `false` otherwise.
<!-- trait ParserExt::fn tell -->
Gets the current stream offset from the parser's internal stream.

# Returns

the current stream offset from the parser's internal stream
or %-1 on error.
<!-- struct ParserOptions -->
A set of parser options used by `Parser` and various other parsing functions.
<!-- impl ParserOptions::fn new -->
Creates a new set of `ParserOptions`.

# Returns

a newly allocated set of `ParserOptions` with the default values.
<!-- impl ParserOptions::fn clone -->
Clones a `ParserOptions`.

# Returns

a newly allocated `ParserOptions`.
<!-- impl ParserOptions::fn free -->
Frees a set of `ParserOptions`.
<!-- impl ParserOptions::fn get_address_compliance_mode -->
Gets the compliance mode that should be used when parsing rfc822 addresses.

Note: Even in `RfcComplianceMode::Strict` mode, the address parser is fairly liberal in
what it accepts. Setting it to `RfcComplianceMode::Loose` just makes it try harder to
deal with garbage input.

# Returns

the compliance mode that is currently set.
<!-- impl ParserOptions::fn get_allow_addresses_without_domain -->
Gets whether or not the rfc822 address parser should allow addresses without a domain.

In general, you'll probably want this value to be `false` (the default) as it allows
maximum interoperability with existing (broken) mail clients and other mail software
such as sloppily written perl scripts (aka spambots) that do not properly quote the
name when it contains a comma.

This option exists in order to allow parsing of mailbox addresses that do not have a
domain component. These types of addresses are rare and were typically only used when
sending mail to other users on the same UNIX system.

# Returns

`true` if the address parser should allow addresses without a domain.
<!-- impl ParserOptions::fn get_fallback_charsets -->
Gets the fallback charsets to try when decoding 8-bit headers.

# Returns

a `None`-terminated list of charsets to try when
decoding 8-bit headers.
<!-- impl ParserOptions::fn get_parameter_compliance_mode -->
Gets the compliance mode that should be used when parsing Content-Type and
Content-Disposition parameters.

Note: Even in `RfcComplianceMode::Strict` mode, the parameter parser is fairly liberal
in what it accepts. Setting it to `RfcComplianceMode::Loose` just makes it try harder
to deal with garbage input.

# Returns

the compliance mode that is currently set.
<!-- impl ParserOptions::fn get_rfc2047_compliance_mode -->
Gets the compliance mode that should be used when parsing rfc2047 encoded words.

Note: Even in `RfcComplianceMode::Strict` mode, the rfc2047 parser is fairly liberal
in what it accepts. Setting it to `RfcComplianceMode::Loose` just makes it try harder
to deal with garbage input.

# Returns

the compliance mode that is currently set.
<!-- impl ParserOptions::fn get_warning_callback -->
Gets callback function which is called if the parser detects any issues.

# Returns

the currently registered warning callback function
<!-- impl ParserOptions::fn set_address_compliance_mode -->
Sets the compliance mode that should be used when parsing rfc822 addresses.

In general, you'll probably want this value to be `RfcComplianceMode::Loose`
(the default) as it allows maximum interoperability with existing (broken) mail clients
and other mail software such as sloppily written perl scripts (aka spambots).

Note: Even in `RfcComplianceMode::Strict` mode, the address parser is fairly liberal in
what it accepts. Setting it to `RfcComplianceMode::Loose` just makes it try harder to
deal with garbage input.
## `mode`
a `RfcComplianceMode`
<!-- impl ParserOptions::fn set_allow_addresses_without_domain -->
Sets whether the rfc822 address parser should allow addresses without a domain.

In general, you'll probably want this value to be `false` (the default) as it allows
maximum interoperability with existing (broken) mail clients and other mail software
such as sloppily written perl scripts (aka spambots) that do not properly quote the
name when it contains a comma.

This option exists in order to allow parsing of mailbox addresses that do not have a
domain component. These types of addresses are rare and were typically only used when
sending mail to other users on the same UNIX system.
## `allow`
`true` if the parser should allow addresses without a domain or `false` otherwise
<!-- impl ParserOptions::fn set_fallback_charsets -->
Sets the fallback charsets to try when decoding 8-bit headers.

Note: It is recommended that the list of charsets start with utf-8
and end with iso-8859-1.
## `charsets`
a `None`-terminated list of charsets or `None` for the default list
<!-- impl ParserOptions::fn set_parameter_compliance_mode -->
Sets the compliance mode that should be used when parsing Content-Type and
Content-Disposition parameters.

In general, you'll probably want this value to be `RfcComplianceMode::Loose`
(the default) as it allows maximum interoperability with existing (broken) mail clients
and other mail software such as sloppily written perl scripts (aka spambots).

Note: Even in `RfcComplianceMode::Strict` mode, the parameter parser is fairly liberal
in what it accepts. Setting it to `RfcComplianceMode::Loose` just makes it try harder
to deal with garbage input.
## `mode`
a `RfcComplianceMode`
<!-- impl ParserOptions::fn set_rfc2047_compliance_mode -->
Sets the compliance mode that should be used when parsing rfc2047 encoded words.

In general, you'll probably want this value to be `RfcComplianceMode::Loose`
(the default) as it allows maximum interoperability with existing (broken) mail clients
and other mail software such as sloppily written perl scripts (aka spambots).

Note: Even in `RfcComplianceMode::Strict` mode, the parameter parser is fairly liberal
in what it accepts. Setting it to `RfcComplianceMode::Loose` just makes it try harder
to deal with garbage input.
## `mode`
a `RfcComplianceMode`
<!-- impl ParserOptions::fn set_warning_callback -->
Registers the callback function being called if the parser detects any issues.
## `warning_cb`
a `GMimeParserWarningFunc` or `None` to clear the callback
## `user_data`
data passed to the warning callback function
<!-- impl ParserOptions::fn get_default -->
Gets the default parser options.

# Returns

the default parser options.
<!-- struct Part -->
A leaf-node MIME part object.

# Implements

[`PartExt`](trait.PartExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait PartExt -->
Trait containing all `Part` methods.

# Implementors

[`ApplicationPkcs7Mime`](struct.ApplicationPkcs7Mime.html), [`MessagePartial`](struct.MessagePartial.html), [`Part`](struct.Part.html), [`TextPart`](struct.TextPart.html)
<!-- impl Part::fn new -->
Creates a new MIME Part object with a default content-type of
application/octet-stream.

# Returns

an empty MIME Part object with a default content-type of
application/octet-stream.
<!-- impl Part::fn new_with_type -->
Creates a new MIME Part with a sepcified type.
## `type_`
content-type string
## `subtype`
content-subtype string

# Returns

an empty MIME Part object with the specified content-type.
<!-- trait PartExt::fn get_best_content_encoding -->
Calculates the most efficient content encoding for the `self`
given the `constraint`.
## `constraint`
a `EncodingConstraint`

# Returns

the best content encoding for the specified mime part.
<!-- trait PartExt::fn get_content -->
Gets the internal data-wrapper of the specified mime part, or `None`
on error.

# Returns

the data-wrapper for the mime part's
contents.
<!-- trait PartExt::fn get_content_description -->
Gets the value of the Content-Description for the specified mime
part if it exists or `None` otherwise.

# Returns

the content description for the specified mime part.
<!-- trait PartExt::fn get_content_encoding -->
Gets the content encoding of the mime part.

# Returns

the content encoding for the specified mime part.
<!-- trait PartExt::fn get_content_id -->
Gets the content-id of the specified mime part if it exists, or
`None` otherwise.

# Returns

the content id for the specified mime part.
<!-- trait PartExt::fn get_content_location -->
Gets the value of the Content-Location header if it exists, or
`None` otherwise.

# Returns

the content location for the specified mime part.
<!-- trait PartExt::fn get_content_md5 -->
Gets the md5sum contained in the Content-Md5 header of the
specified mime part if it exists, or `None` otherwise.

# Returns

the content md5 for the specified mime part.
<!-- trait PartExt::fn get_filename -->
Gets the filename of the specificed mime part, or `None` if the
`self` does not have the filename or name parameter set.

# Returns

the filename of the specified `self` or `None` if
neither of the parameters is set. If a file name is set, the
returned string will be in UTF-8.
<!-- trait PartExt::fn get_openpgp_data -->
Gets whether or not (and what type) of OpenPGP data is contained
within the `Part`.

# Returns

a `OpenPGPData`.
<!-- trait PartExt::fn is_attachment -->
Determines whether or not the part is an attachment based on the
value of the Content-Disposition header.

# Returns

`true` if the part is an attachment, otherwise `false`.
<!-- trait PartExt::fn openpgp_decrypt -->
Decrypts the content of the `self` and then replaces the content with
the new, decrypted, content.
## `flags`
a set of `DecryptFlags`
## `session_key`
the session key to use or `None`

# Returns

a `DecryptResult` on success or `None` on error.
<!-- trait PartExt::fn openpgp_encrypt -->
Encrypts (and optionally signs) the content of the `self` and then replaces
the content with the new, encrypted, content.
## `sign`
`true` if the content should also be signed; otherwise, `false`
## `userid`
the key id (or email address) to use when signing (assuming `sign` is `true`)
## `flags`
a set of `EncryptFlags`
## `recipients`
an array of recipient key ids and/or email addresses

# Returns

`true` on success or `false` on error.
<!-- trait PartExt::fn openpgp_sign -->
Signs the content of the `self` and then replaces the content with
the new, signed, content.
## `userid`
the key id (or email address) to use for signing

# Returns

`true` on success or `false` on error.
<!-- trait PartExt::fn openpgp_verify -->
Verifies the OpenPGP signature of the `self` and then replaces the content
with the original, raw, content.
## `flags`
a set of `VerifyFlags`

# Returns

a `SignatureList` on success or `None` on error.
<!-- trait PartExt::fn set_content -->
Sets the content on the mime part.
## `content`
a `DataWrapper` content object
<!-- trait PartExt::fn set_content_description -->
Set the content description for the specified mime part.
## `description`
content description
<!-- trait PartExt::fn set_content_encoding -->
Set the content encoding for the specified mime part.
## `encoding`
a `ContentEncoding`
<!-- trait PartExt::fn set_content_id -->
Set the content id for the specified mime part.
## `content_id`
content id
<!-- trait PartExt::fn set_content_location -->
Set the content location for the specified mime part.
## `content_location`
content location
<!-- trait PartExt::fn set_content_md5 -->
Set the content md5 for the specified mime part.
## `content_md5`
content md5 or `None` to generate the md5 digest.
<!-- trait PartExt::fn set_filename -->
Sets the "filename" parameter on the Content-Disposition and also sets the
"name" parameter on the Content-Type.

Note: The `filename` string should be in UTF-8.
## `filename`
the file name
<!-- trait PartExt::fn set_openpgp_data -->
Sets whether or not (and what type) of OpenPGP data is contained
within the `Part`.
## `data`
a `OpenPGPData`
<!-- trait PartExt::fn verify_content_md5 -->
Verify the content md5 for the specified mime part.

# Returns

`true` if the md5 is valid or `false` otherwise. Note: will
return `false` if the mime part does not contain a Content-MD5.
<!-- struct PartIter -->
A MIME part iterator.
<!-- impl PartIter::fn new -->
Creates a new `PartIter` for iterating over `toplevel`'s subparts.
## `toplevel`
a `Object` to use as the toplevel

# Returns

a newly allocated `PartIter` which should be freed
using `PartIter::free` when finished with it.
<!-- impl PartIter::fn clone -->
Clones the `self`, including its current state.

# Returns

a new `PartIter` that is identical to `self`.
<!-- impl PartIter::fn free -->
Frees the memory allocated by `PartIter::new`.
<!-- impl PartIter::fn get_current -->
Gets the `Object` at the current `PartIter` position.

# Returns

the current `Object` or `None` if the
state of `self` is invalid.
<!-- impl PartIter::fn get_parent -->
Gets the parent of the `Object` at the current `PartIter`
position.

# Returns

the parent `Object` or `None` if the
state of `self` is invalid.
<!-- impl PartIter::fn get_path -->
Gets the path of the current `Object` in the MIME structure
used to initialize `self`.

# Returns

a newly allocated string representation of the path to the
`Object` at the current `PartIter` position.
<!-- impl PartIter::fn get_toplevel -->
Gets the toplevel `Object` used to initialize `self`.

# Returns

the toplevel `Object`.
<!-- impl PartIter::fn is_valid -->
Checks that the current state of `self` is valid.

# Returns

`true` if `self` is valid or `false` otherwise.
<!-- impl PartIter::fn jump_to -->
Updates the state of `self` to point to the `Object` specified
by `path`.
## `path`
a string representing the path to jump to

# Returns

`true` if the `Object` specified by `path` exists or
`false` otherwise.
<!-- impl PartIter::fn next -->
Advances to the next part in the MIME structure used to initialize
`self`.

# Returns

`true` if successful or `false` otherwise.
<!-- impl PartIter::fn prev -->
Rewinds to the previous part in the MIME structure used to
initialize `self`.

# Returns

`true` if successful or `false` otherwise.
<!-- impl PartIter::fn remove -->
Removes the `Object` at the current position from its
parent. If successful, `self` is advanced to the next position
(since the current position will become invalid).

# Returns

`true` if the part at the current position was removed or
`false` otherwise.
<!-- impl PartIter::fn replace -->
Replaces the `Object` at the current position with `replacement`.
## `replacement`
a `Object`

# Returns

`true` if the part at the current position was replaced or
`false` otherwise.
<!-- impl PartIter::fn reset -->
Resets the state of `self` to its initial state.
<!-- struct Pkcs7Context -->
A PKCS7 crypto context.

# Implements

[`CryptoContextExt`](trait.CryptoContextExt.html)
<!-- impl Pkcs7Context::fn new -->
Creates a new pkcs7 crypto context object.

# Returns

a new pkcs7 crypto context object.
<!-- enum PubKeyAlgo -->
A public-key algorithm.
<!-- enum PubKeyAlgo::variant Default -->
The default public-key algorithm.
<!-- enum PubKeyAlgo::variant Rsa -->
The RSA algorithm.
<!-- enum PubKeyAlgo::variant RsaE -->
An encryption-only RSA algorithm.
<!-- enum PubKeyAlgo::variant RsaS -->
A signature-only RSA algorithm.
<!-- enum PubKeyAlgo::variant ElgE -->
An encryption-only ElGamal algorithm.
<!-- enum PubKeyAlgo::variant Dsa -->
The DSA algorithm.
<!-- enum PubKeyAlgo::variant Ecc -->
The Eliptic Curve algorithm.
<!-- enum PubKeyAlgo::variant Elg -->
The ElGamal algorithm.
<!-- enum PubKeyAlgo::variant Ecdsa -->
The Eliptic Curve + DSA algorithm.
<!-- enum PubKeyAlgo::variant Ecdh -->
The Eliptic Curve + Diffie Helman algorithm.
<!-- enum PubKeyAlgo::variant Eddsa -->
The Eliptic Curve + DSA algorithm.
<!-- enum RfcComplianceMode -->
An RFC compliance mode.
<!-- enum RfcComplianceMode::variant Loose -->
Attempt to be much more liberal accepting broken and/or invalid formatting.
<!-- enum RfcComplianceMode::variant Strict -->
Do not attempt to be overly liberal in accepting broken and/or invalid formatting.
<!-- enum SecureMimeType -->
The S/MIME data type.
<!-- enum SecureMimeType::variant CompressedData -->
The S/MIME content contains compressed data.
<!-- enum SecureMimeType::variant EnvelopedData -->
The S/MIME content contains enveloped data.
<!-- enum SecureMimeType::variant SignedData -->
The S/MIME content contains signed data.
<!-- enum SecureMimeType::variant CertsOnly -->
The S/MIME content contains only certificates.
<!-- enum SecureMimeType::variant Unknown -->
The S/MIME content is unknown.
<!-- enum SeekWhence -->
Relative seek position.
<!-- enum SeekWhence::variant Set -->
Seek relative to the beginning of the stream.
<!-- enum SeekWhence::variant Cur -->
Seek relative to the current position in the stream.
<!-- enum SeekWhence::variant End -->
Seek relative to the end of the stream.
<!-- struct Signature -->
An object containing useful information about a signature.

# Implements

[`SignatureExt`](trait.SignatureExt.html)
<!-- trait SignatureExt -->
Trait containing all `Signature` methods.

# Implementors

[`Signature`](struct.Signature.html)
<!-- impl Signature::fn new -->
Creates a new `Signature` object.

# Returns

a new `Signature` object.
<!-- trait SignatureExt::fn get_certificate -->
Get the signature's certificate.

# Returns

the signature's certificate.
<!-- trait SignatureExt::fn get_created -->
Get the creation date of the signature.

# Returns

the creation date of the signature or %-1 if unknown.
<!-- trait SignatureExt::fn get_expires -->
Get the expiration date of the signature.

# Returns

the expiration date of the signature or %-1 if unknown.
<!-- trait SignatureExt::fn get_status -->
Get the signature status as a bitfield of `SignatureStatus` flags.

# Returns

the signature status.
<!-- trait SignatureExt::fn set_certificate -->
Set the signature's certificate.
## `cert`
a `Certificate`
<!-- trait SignatureExt::fn set_created -->
Set the creation date of the signature.
## `created`
creation date
<!-- trait SignatureExt::fn set_expires -->
Set the expiration date of the signature.
## `expires`
expiration date
<!-- trait SignatureExt::fn set_status -->
Set the status on the signature.
## `status`
a `SignatureStatus`
<!-- struct SignatureList -->
A collection of `Signature` objects.

# Implements

[`SignatureListExt`](trait.SignatureListExt.html)
<!-- trait SignatureListExt -->
Trait containing all `SignatureList` methods.

# Implementors

[`SignatureList`](struct.SignatureList.html)
<!-- impl SignatureList::fn new -->
Creates a new `SignatureList`.

# Returns

a new `SignatureList`.
<!-- trait SignatureListExt::fn add -->
Adds a `Signature` to the `SignatureList`.
## `sig`
a `Signature`

# Returns

the index of the added `Signature`.
<!-- trait SignatureListExt::fn clear -->
Clears the list of addresses.
<!-- trait SignatureListExt::fn contains -->
Checks whether or not the specified `Signature` is contained within
the `SignatureList`.
## `sig`
a `Signature`

# Returns

`true` if the specified `Signature` is contained within the
specified `SignatureList` or `false` otherwise.
<!-- trait SignatureListExt::fn get_signature -->
Gets the `Signature` at the specified index.
## `index`
index of `Signature` to get

# Returns

the `Signature` at the specified
index or `None` if the index is out of range.
<!-- trait SignatureListExt::fn index_of -->
Gets the index of the specified `Signature` inside the
`SignatureList`.
## `sig`
a `Signature`

# Returns

the index of the requested `Signature` within the
`SignatureList` or %-1 if it is not contained within the
`SignatureList`.
<!-- trait SignatureListExt::fn insert -->
Inserts a `Signature` into the `SignatureList` at the specified
index.
## `index`
index to insert at
## `sig`
a `Signature`
<!-- trait SignatureListExt::fn length -->
Gets the length of the list.

# Returns

the number of `Signature` objects in the list.
<!-- trait SignatureListExt::fn remove -->
Removes a `Signature` from the `SignatureList`.
## `sig`
a `Signature`

# Returns

`true` if the specified `Signature` was removed or `false`
otherwise.
<!-- trait SignatureListExt::fn remove_at -->
Removes a `Signature` from the `SignatureList` at the specified
index.
## `index`
index to remove

# Returns

`true` if an `Signature` was removed or `false` otherwise.
<!-- trait SignatureListExt::fn set_signature -->
Sets the `Signature` at the specified index to `sig`.
## `index`
index of `Signature` to set
## `sig`
a `Signature`
<!-- enum SignatureStatus -->
A value representing the signature status bit flags for a particular
`Signature`.
<!-- enum SignatureStatus::variant Valid -->
The signature is fully valid.
<!-- enum SignatureStatus::variant Green -->
The signature is good.
<!-- enum SignatureStatus::variant Red -->
The signature is bad.
<!-- enum SignatureStatus::variant KeyRevoked -->
The key has been revoked.
<!-- enum SignatureStatus::variant KeyExpired -->
The key has expired.
<!-- enum SignatureStatus::variant SigExpired -->
The signature has expired.
<!-- enum SignatureStatus::variant KeyMissing -->
Can't verify due to missing key.
<!-- enum SignatureStatus::variant CrlMissing -->
CRL not available.
<!-- enum SignatureStatus::variant CrlTooOld -->
Available CRL is too old.
<!-- enum SignatureStatus::variant BadPolicy -->
A policy was not met.
<!-- enum SignatureStatus::variant SysError -->
A system error occurred.
<!-- enum SignatureStatus::variant TofuConflict -->
Tofu conflict detected.
<!-- struct Stream -->
Abstract I/O stream class.

# Implements

[`StreamExt`](trait.StreamExt.html)
<!-- trait StreamExt -->
Trait containing all `Stream` methods.

# Implementors

[`StreamBuffer`](struct.StreamBuffer.html), [`StreamCat`](struct.StreamCat.html), [`StreamFile`](struct.StreamFile.html), [`StreamFilter`](struct.StreamFilter.html), [`StreamFs`](struct.StreamFs.html), [`StreamGIO`](struct.StreamGIO.html), [`StreamMem`](struct.StreamMem.html), [`StreamMmap`](struct.StreamMmap.html), [`StreamNull`](struct.StreamNull.html), [`StreamPipe`](struct.StreamPipe.html), [`Stream`](struct.Stream.html)
<!-- trait StreamExt::fn buffer_gets -->
Reads in at most one less than `max` characters from `self` and
stores them into the buffer pointed to by `buf`. Reading stops after
an EOS or newline ('\n'). If a newline is read, it is stored into
the buffer. A '\0' is stored after the last character in the
buffer.
## `buf`
line buffer
## `max`
max length of a line

# Returns

the number of characters read into `buf` on success or %-1
on fail.
<!-- trait StreamExt::fn buffer_readln -->
Reads a single line into `buffer`.
## `buffer`
output buffer
<!-- trait StreamExt::fn close -->
Closes the stream.

# Returns

`0` on success or %-1 on fail.
<!-- trait StreamExt::fn construct -->
Initializes a new stream with bounds `start` and `end`.
## `start`
start boundary
## `end`
end boundary
<!-- trait StreamExt::fn eos -->
Tests the end-of-stream indicator for `self`.

# Returns

`true` on EOS or `false` otherwise.
<!-- trait StreamExt::fn flush -->
Sync's the stream to disk.

# Returns

`0` on success or %-1 on fail.
<!-- trait StreamExt::fn length -->
Gets the length of the stream.

# Returns

the length of the stream or %-1 if unknown.
<!-- trait StreamExt::fn printf -->
Write formatted output to a stream.
## `fmt`
format

# Returns

the number of bytes written or %-1 on fail.
<!-- trait StreamExt::fn read -->
Attempts to read up to `len` bytes from `self` into `buf`.
## `buf`
buffer
## `len`
buffer length

# Returns

the number of bytes read or %-1 on fail.
<!-- trait StreamExt::fn reset -->
Resets the stream.

# Returns

`0` on success or %-1 on fail.
<!-- trait StreamExt::fn seek -->
Repositions the offset of the stream `self` to
the argument `offset` according to the
directive `whence` as follows:

 `SeekWhence::Set`: Seek `offset` bytes relative to
 the beginning (bound_start) of the stream.

 `SeekWhence::Cur`: Seek `offset` bytes relative to the
 current offset of the stream.

 `SeekWhence::End`: Seek `offset` bytes relative to the
 end of the stream (bound_end if non-negative).
## `offset`
positional offset
## `whence`
seek directive

# Returns

the resultant position on success or %-1 on fail.
<!-- trait StreamExt::fn set_bounds -->
Set the bounds on a stream.
## `start`
start boundary
## `end`
end boundary
<!-- trait StreamExt::fn substream -->
Creates a new substream of `self` with bounds `start` and `end`.
## `start`
start boundary
## `end`
end boundary

# Returns

a substream of `self` with bounds `start`
and `end`.
<!-- trait StreamExt::fn tell -->
Gets the current offset within the stream.

# Returns

the current position within the stream or %-1 on fail.
<!-- trait StreamExt::fn write -->
Attempts to write up to `len` bytes of `buf` to `self`.
## `buf`
buffer
## `len`
buffer length

# Returns

the number of bytes written or %-1 on fail.
<!-- trait StreamExt::fn write_string -->
Writes `string` to `self`.
## `str`
string to write

# Returns

the number of bytes written or %-1 on fail.
<!-- trait StreamExt::fn write_to_stream -->
Attempts to write the source stream to the destination stream.
## `dest`
destination stream

# Returns

the number of bytes written or %-1 on fail.
<!-- trait StreamExt::fn writev -->
Writes at most `count` blocks described by `vector` to `self`.
## `vector`
a `StreamIOVector`
## `count`
number of vector elements

# Returns

the number of bytes written or %-1 on fail.
<!-- struct StreamBuffer -->
A buffered stream wrapper around any `Stream` object.

# Implements

[`StreamExt`](trait.StreamExt.html)
<!-- impl StreamBuffer::fn new -->
Creates a new GMimeStreamBuffer object.
## `source`
source stream
## `mode`
buffering mode

# Returns

a new buffer stream with source `source` and mode `mode`.
<!-- enum StreamBufferMode -->
The buffering mode for a `StreamBuffer` stream.
<!-- enum StreamBufferMode::variant Read -->
Read in 4k blocks.
<!-- enum StreamBufferMode::variant Write -->
Write in 4k blocks.
<!-- struct StreamCat -->
A concatenation of other `Stream` objects.

# Implements

[`StreamCatExt`](trait.StreamCatExt.html), [`StreamExt`](trait.StreamExt.html)
<!-- trait StreamCatExt -->
Trait containing all `StreamCat` methods.

# Implementors

[`StreamCat`](struct.StreamCat.html)
<!-- impl StreamCat::fn new -->
Creates a new `StreamCat` object.

# Returns

a new `StreamCat` stream.
<!-- trait StreamCatExt::fn add_source -->
Adds the `source` stream to the `self`.
## `source`
a source stream

# Returns

`0` on success or %-1 on fail.
<!-- struct StreamFile -->
A `Stream` wrapper around standard-c FILE pointers.

# Implements

[`StreamFileExt`](trait.StreamFileExt.html), [`StreamExt`](trait.StreamExt.html)
<!-- trait StreamFileExt -->
Trait containing all `StreamFile` methods.

# Implementors

[`StreamFile`](struct.StreamFile.html)
<!-- impl StreamFile::fn new -->
Creates a new `StreamFile` object around `fp`.

Note: The created `StreamFile` object will own the FILE pointer
passed in.
## `fp`
a FILE pointer

# Returns

a stream using `fp`.
<!-- impl StreamFile::fn new_with_bounds -->
Creates a new `StreamFile` object around `fp` with bounds `start`
and `end`.

Note: The created `StreamFile` object will own the FILE pointer
passed in.
## `fp`
a FILE pointer
## `start`
start boundary
## `end`
end boundary

# Returns

a stream using `fp` with bounds `start` and `end`.
<!-- impl StreamFile::fn open -->
Creates a new `StreamFile` object for the specified `path`.
## `path`
the path to a file
## `mode`
as in fopen(3)

# Returns

a stream using for reading and/or writing to the
specified file path or `None` on error.
<!-- trait StreamFileExt::fn get_owner -->
Gets whether or not `self` owns the backend FILE pointer.

# Returns

`true` if `self` owns the backend FILE pointer or `false`
otherwise.
<!-- trait StreamFileExt::fn set_owner -->
Sets whether or not `self` owns the backend FILE pointer.

Note: `owner` should be `true` if the stream should `fclose` the
backend FILE pointer when destroyed or `false` otherwise.
## `owner`
`true` if this stream should own the FILE pointer or `false` otherwise
<!-- struct StreamFilter -->
A `Stream` which passes data through any `Filter` objects.

# Implements

[`StreamFilterExt`](trait.StreamFilterExt.html), [`StreamExt`](trait.StreamExt.html)
<!-- trait StreamFilterExt -->
Trait containing all `StreamFilter` methods.

# Implementors

[`StreamFilter`](struct.StreamFilter.html)
<!-- impl StreamFilter::fn new -->
Creates a new `StreamFilter` object using `stream` as the source
stream.
## `stream`
source stream

# Returns

a new filter stream with `stream` as its source.
<!-- trait StreamFilterExt::fn add -->
Adds `filter` to `self`. Filters are applied in the same order in
which they are added.
## `filter`
a `Filter`

# Returns

an id for the filter.
<!-- trait StreamFilterExt::fn get_owner -->
Gets whether or not `self` owns the source stream.

# Returns

`true` if `self` owns the source stream or `false`
otherwise.
<!-- trait StreamFilterExt::fn remove -->
Removed a filter from the stream based on the id (as returned from
filter_add).
## `id`
filter id
<!-- trait StreamFilterExt::fn set_owner -->
Sets whether or not `self` owns the source stream..

Note: `owner` should be `true` if the stream should `close` the
backend source stream when destroyed or `false` otherwise.
## `owner`
`true` if this stream should own the source stream or `false` otherwise
<!-- struct StreamFs -->
A `Stream` wrapper around POSIX file descriptors.

# Implements

[`StreamFsExt`](trait.StreamFsExt.html), [`StreamExt`](trait.StreamExt.html)
<!-- trait StreamFsExt -->
Trait containing all `StreamFs` methods.

# Implementors

[`StreamFs`](struct.StreamFs.html)
<!-- impl StreamFs::fn new -->
Creates a new `StreamFs` object around `fd`.
## `fd`
a file descriptor

# Returns

a stream using `fd`.
<!-- impl StreamFs::fn new_with_bounds -->
Creates a new `StreamFs` object around `fd` with bounds `start`
and `end`.
## `fd`
a file descriptor
## `start`
start boundary
## `end`
end boundary

# Returns

a stream using `fd` with bounds `start` and `end`.
<!-- impl StreamFs::fn open -->
Creates a new `StreamFs` object for the specified `path`.
## `path`
the path to a file
## `flags`
as in open(2)
## `mode`
as in open(2)

# Returns

a stream using for reading and/or writing to the
specified file path or `None` on error.
<!-- trait StreamFsExt::fn get_owner -->
Gets whether or not `self` owns the backend file descriptor.

# Returns

`true` if `self` owns the backend file descriptor or `false`
otherwise.
<!-- trait StreamFsExt::fn set_owner -->
Sets whether or not `self` owns the backend file descriptor.

Note: `owner` should be `true` if the stream should `close` the
backend file descriptor when destroyed or `false` otherwise.
## `owner`
`true` if this stream should own the file descriptor or `false` otherwise
<!-- struct StreamGIO -->
A `Stream` wrapper around GLib's GIO streams.

# Implements

[`StreamGIOExt`](trait.StreamGIOExt.html), [`StreamExt`](trait.StreamExt.html)
<!-- trait StreamGIOExt -->
Trait containing all `StreamGIO` methods.

# Implementors

[`StreamGIO`](struct.StreamGIO.html)
<!-- impl StreamGIO::fn new -->
Creates a new `StreamGIO` wrapper around a `gio::File` object.
## `file`
a `gio::File`

# Returns

a stream using `file`.
<!-- impl StreamGIO::fn new_with_bounds -->
Creates a new `StreamGIO` stream around a `gio::File` with bounds
`start` and `end`.
## `file`
a `gio::File`
## `start`
start boundary
## `end`
end boundary

# Returns

a stream using `file` with bounds `start`
and `end`.
<!-- trait StreamGIOExt::fn get_owner -->
Gets whether or not `self` owns the backend `gio::File`.

# Returns

`true` if `self` owns the backend `gio::File` or `false`
otherwise.
<!-- trait StreamGIOExt::fn set_owner -->
Sets whether or not `self` owns the backend GIO pointer.

Note: `owner` should be `true` if the stream should `close` the
backend file descriptor when destroyed or `false` otherwise.
## `owner`
`true` if this stream should own the `gio::File` or `false` otherwise
<!-- struct StreamMem -->
A memory-backed `Stream`.

# Implements

[`StreamMemExt`](trait.StreamMemExt.html), [`StreamExt`](trait.StreamExt.html)
<!-- trait StreamMemExt -->
Trait containing all `StreamMem` methods.

# Implementors

[`StreamMem`](struct.StreamMem.html)
<!-- impl StreamMem::fn new -->
Creates a new `StreamMem` object.

# Returns

a new memory stream.
<!-- impl StreamMem::fn new_with_buffer -->
Creates a new `StreamMem` object and initializes the stream
contents with the first `len` bytes of `buffer`.
## `buffer`
stream data
## `len`
buffer length

# Returns

a new memory stream initialized with `buffer`.
<!-- impl StreamMem::fn new_with_byte_array -->
Creates a new `StreamMem` with data `array`.
## `array`
source data

# Returns

a new memory stream using `array`.
<!-- trait StreamMemExt::fn get_byte_array -->
Gets the byte array from the memory stream.

# Returns

the byte array from the memory stream.
<!-- trait StreamMemExt::fn get_owner -->
Gets whether or not `self` owns the backend memory buffer.

# Returns

`true` if `self` owns the backend memory buffer or `false`
otherwise.
<!-- trait StreamMemExt::fn set_byte_array -->
Sets the byte array on the memory stream.

Note: The memory stream is not responsible for freeing the byte
array. Use `StreamMemExt::set_owner` to change this behavior.
## `array`
stream data
<!-- trait StreamMemExt::fn set_owner -->
Sets whether or not `self` owns the backend memory buffer.

Note: `owner` should be `true` if the stream should free the backend
memory buffer when destroyed or `false` otherwise.
## `owner`
`true` if this stream should own the `glib::ByteArray` or `false` otherwise
<!-- struct StreamMmap -->
A memory-mapped `Stream`.

# Implements

[`StreamMmapExt`](trait.StreamMmapExt.html), [`StreamExt`](trait.StreamExt.html)
<!-- trait StreamMmapExt -->
Trait containing all `StreamMmap` methods.

# Implementors

[`StreamMmap`](struct.StreamMmap.html)
<!-- impl StreamMmap::fn new -->
Creates a new `StreamMmap` object around `fd`.
## `fd`
file descriptor
## `prot`
protection flags
## `flags`
map flags

# Returns

a stream using `fd`.
<!-- impl StreamMmap::fn new_with_bounds -->
Creates a new `StreamMmap` object around `fd` with bounds `start`
and `end`.
## `fd`
file descriptor
## `prot`
protection flags
## `flags`
map flags
## `start`
start boundary
## `end`
end boundary

# Returns

a stream using `fd` with bounds `start` and `end`.
<!-- trait StreamMmapExt::fn get_owner -->
Gets whether or not `self` owns the backend file descriptor.

Feature: `v3_2`


# Returns

`true` if `self` owns the backend file descriptor or `false`
otherwise.
<!-- trait StreamMmapExt::fn set_owner -->
Sets whether or not `self` owns the backend file descriptor.

Note: `owner` should be `true` if the stream should `close` the
backend file descriptor when destroyed or `false` otherwise.

Feature: `v3_2`

## `owner`
`true` if this stream should own the file descriptor or `false` otherwise
<!-- struct StreamNull -->
A `Stream` which has no backing store.

# Implements

[`StreamNullExt`](trait.StreamNullExt.html), [`StreamExt`](trait.StreamExt.html)
<!-- trait StreamNullExt -->
Trait containing all `StreamNull` methods.

# Implementors

[`StreamNull`](struct.StreamNull.html)
<!-- impl StreamNull::fn new -->
Creates a new `StreamNull` object.

# Returns

a new null stream (similar to /dev/null on Unix).
<!-- trait StreamNullExt::fn get_count_newlines -->
Gets whether or not the stream should keep track of the number of newlines
encountered.

# Returns

`true` if the stream should count the number of newlines or `false` otherwise.
<!-- trait StreamNullExt::fn set_count_newlines -->
Sets whether or not the stream should keep track of the number of newlines
encountered.
## `count`
`true` if newlines should be counted or `false` otherwise
<!-- struct StreamPipe -->
A `Stream` wrapper around pipes.

# Implements

[`StreamPipeExt`](trait.StreamPipeExt.html), [`StreamExt`](trait.StreamExt.html)
<!-- trait StreamPipeExt -->
Trait containing all `StreamPipe` methods.

# Implementors

[`StreamPipe`](struct.StreamPipe.html)
<!-- impl StreamPipe::fn new -->
Creates a new `StreamPipe` object around `fd`.
## `fd`
a pipe descriptor

# Returns

a stream using `fd`.
<!-- trait StreamPipeExt::fn get_owner -->
Gets whether or not `self` owns the backend pipe descriptor.

# Returns

`true` if `self` owns the backend pipe descriptor or `false`
otherwise.
<!-- trait StreamPipeExt::fn set_owner -->
Sets whether or not `self` owns the backend pipe descriptor.

Note: `owner` should be `true` if the stream should `close` the
backend pipe descriptor when destroyed or `false` otherwise.
## `owner`
owner
<!-- struct TextPart -->
A text MIME part object.

# Implements

[`TextPartExt`](trait.TextPartExt.html), [`PartExt`](trait.PartExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait TextPartExt -->
Trait containing all `TextPart` methods.

# Implementors

[`TextPart`](struct.TextPart.html)
<!-- impl TextPart::fn new -->
Creates a new text MIME part object with a default content-type of
text/plain.

# Returns

an empty MIME Part object with a default content-type of
text/plain.
<!-- impl TextPart::fn new_with_subtype -->
Creates a new text MIME part with a sepcified subtype.
## `subtype`
textual subtype string

# Returns

an empty text MIME part object with the specified subtype.
<!-- trait TextPartExt::fn get_charset -->
Gets the value of the charset parameter on the Content-Type header.

# Returns

the value of the charset parameter or `None` if unavailable.
<!-- trait TextPartExt::fn get_text -->
Gets the text content of the `self` as a string.

# Returns

a newly allocated string containing the utf-8 encoded text content.
<!-- trait TextPartExt::fn set_charset -->
Sets the charset parameter on the Content-Type header to the specified value.
## `charset`
the name of the charset
<!-- trait TextPartExt::fn set_text -->
Sets the specified text as the content and updates the charset parameter on the Content-Type header.
## `text`
the text in utf-8
<!-- enum Trust -->
The trust level of a certificate. Trust level tries to answer the
question: "How much is the user willing to rely on cryptographic
identity assertions made by the owner of this certificate?"

By way of comparison with web browser X.509 certificate validation
stacks, the certificate of a "Root CA" has `Trust::Ultimate`,
while the certificate of an intermediate CA has `Trust::Full`,
and an end-entity certificate (e.g., with CA:FALSE set) would have
`Trust::Never`.
<!-- enum Trust::variant Unknown -->
We do not know whether to rely on identity assertions made by the certificate.
<!-- enum Trust::variant Undefined -->
We do not have enough information to decide whether to rely on identity assertions made by the certificate.
<!-- enum Trust::variant Never -->
We should never rely on identity assertions made by the certificate.
<!-- enum Trust::variant Marginal -->
We can rely on identity assertions made by this certificate as long as they are corroborated by other marginally-trusted certificates.
<!-- enum Trust::variant Full -->
We can rely on identity assertions made by this certificate.
<!-- enum Trust::variant Ultimate -->
This certificate is an undeniable root of trust (e.g. normally, this is a certificate controlled by the user themselves).
<!-- enum Validity -->
The validity level of a certificate's User ID. Validity level
tries to answer the question: "How strongly do we believe that this
certificate belongs to the party it says it belongs to?"

Note that some OpenPGP certificates have multiple User IDs, and
each User ID may have a different validity level (e.g. depending on
which third parties have certified which User IDs, and which third
parties the local user has chosen to trust).

Similarly, an X.509 certificate can have multiple SubjectAltNames,
and each name may also have a different validity level (e.g. if the
issuing CA is bound by name constraints).

Note that the GMime API currently only exposes the highest-validty
User ID for any given certificate.
<!-- enum Validity::variant Unknown -->
The User ID of the certificate is of unknown validity.
<!-- enum Validity::variant Undefined -->
The User ID of the certificate is undefined.
<!-- enum Validity::variant Never -->
The User ID of the certificate is never to be treated as valid.
<!-- enum Validity::variant Marginal -->
The User ID of the certificate is marginally valid (e.g. it has been certified by only one marginally-trusted party).
<!-- enum Validity::variant Full -->
The User ID of the certificate is fully valid.
<!-- enum Validity::variant Ultimate -->
The User ID of the certificate is ultimately valid (i.e., usually the certificate belongs to the local user themselves).
<!-- enum VerifyFlags -->
Signature verification flags.
<!-- enum VerifyFlags::variant VerifyNone -->
No flags specified.
