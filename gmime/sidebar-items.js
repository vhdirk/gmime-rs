initSidebarItems({"enum":[["AddressType","An address type."],["AutocryptPreferEncrypt","A description of the user's preference for encrypted messaging."],["ChecksumType",""],["ContentEncoding","A Content-Transfer-Encoding enumeration."],["DigestAlgo","A hash algorithm."],["EncodingConstraint","Used with functions like `FilterBestExt::encoding` and `ObjectExt::encode` as the 'constraint' argument. These values provide a means of letting the filter know what the encoding constraints are for the stream."],["EncryptFlags","Encryption flags."],["FilterFromMode","The mode for a `FilterFrom` filter."],["FilterGZipMode","The mode for the `FilterGZip` filter."],["Format","An enum of formats."],["NewLineFormat","There are two commonly used line-endings used by modern Operating Systems. Unix-based systems such as Linux and Mac OS use a single character ('\\n' aka LF) to represent the end of line where-as Windows (or DOS) uses a sequence of two characters (\"\\r\\n\" aka CRLF). Most text-based network protocols such as SMTP, POP3, and IMAP use the CRLF sequence as well."],["OpenPGPData","The type of OpenPGP data found, if any."],["ParamEncodingMethod","The MIME specifications specify that the proper method for encoding Content-Type and Content-Disposition parameter values is the method described in rfc2231`</a>`. However, it is common for some older email clients to improperly encode using the method described in rfc2047`</a>` instead."],["PubKeyAlgo","A public-key algorithm."],["RfcComplianceMode","An RFC compliance mode."],["SecureMimeType","The S/MIME data type."],["SeekType",""],["SeekWhence","Relative seek position."],["SignatureStatus","A value representing the signature status bit flags for a particular `Signature`."],["StreamBufferMode","The buffering mode for a `StreamBuffer` stream."],["Trust","The trust level of a certificate. Trust level tries to answer the question: \"How much is the user willing to rely on cryptographic identity assertions made by the owner of this certificate?\""],["Validity","The validity level of a certificate's User ID. Validity level tries to answer the question: \"How strongly do we believe that this certificate belongs to the party it says it belongs to?\""],["VerifyFlags","Signature verification flags."]],"struct":[["ApplicationPkcs7Mime","An application/pkcs7-mime MIME part."],["AutocryptHeader","An object containing Autocrypt information about a given e-mail address, as derived from a message header."],["AutocryptHeaderList","A list of Autocrypt headers, typically extracted from a GMimeMessage."],["Bytes","A shared immutable byte slice (the equivalent of `Rc<[u8]>`)."],["Certificate","An object containing useful information about a certificate."],["CertificateList","A collection of `Certificate` objects."],["ContentDisposition","A data structure representing a Content-Disposition."],["ContentType","A data structure representing a Content-Type."],["CryptoContext","A crypto context for use with MIME."],["DataWrapper","A wrapper for a stream which may be encoded."],["DateTime",""],["DecryptFlags",""],["DecryptResult","An object containing the results from decrypting an encrypted stream."],["Error","A generic error capable of representing various error domains (types)."],["File",""],["Filter","Base class for filters used by `StreamFilter`."],["FilterBasic","A basic encoder/decoder filter for the MIME encodings."],["FilterBest","A filter for calculating the best encoding and/or charset to encode the data passed through it."],["FilterBestFlags",""],["FilterCharset","A filter to convert between charsets."],["FilterDos2Unix","A filter to convert a stream from Windows/DOS line endings to Unix line endings."],["FilterEnriched","A filter for converting text/enriched or text/richtext textual streams into text/html."],["FilterFrom","A filter for armoring or escaping lines beginning with \"From \"."],["FilterGZip","A filter for compresing or decompressing a gzip stream."],["FilterHTML","A filter for converting text/plain into text/html."],["FilterSmtpData","A filter to byte-stuff SMTP DATA."],["FilterStrip","A filter for stripping whitespace from the end of lines."],["FilterUnix2Dos","A filter to convert a stream from Windows/DOS line endings to Unix line endings."],["FilterWindows","A filter for detecting whether or not a text stream claimed to be iso-8859-X is really that charset or if it is really a Windows-CP125x charset."],["FilterYenc","A filter for yEncoding or yDecoding a stream."],["FormatOptions","Format options for serializing various GMime objects."],["GpgContext","A GnuPG crypto context."],["Header","A message or mime-part header."],["HeaderList","A list of message or mime-part headers."],["IOCondition",""],["InternetAddress","An RFC 2822 Address object."],["InternetAddressList","A collection of `InternetAddress` objects."],["InternetAddressMailbox","An RFC 2822 Mailbox address."],["Message","A MIME Message object."],["MessagePart","A message/rfc822 or message/news MIME part."],["MessagePartial","A message/partial MIME part."],["Multipart","A base MIME multipart object."],["MultipartEncrypted","A multipart/encrypted MIME part."],["MultipartSigned","A multipart/signed MIME part."],["Object","Base class for all MIME parts."],["Param","A parameter name/value pair as used in the Content-Type and Content-Disposition headers."],["ParamList","A list of Content-Type or Content-Disposition parameters."],["Parser","A MIME parser context."],["ParserOptions","A set of parser options used by `Parser` and various other parsing functions."],["Part","A leaf-node MIME part object."],["PartIter","A MIME part iterator."],["Pkcs7Context","A PKCS7 crypto context."],["Priority","The priority of sources"],["Signature","An object containing useful information about a signature."],["SignatureList","A collection of `Signature` objects."],["Source",""],["Stream","Abstract I/O stream class."],["StreamBuffer","A buffered stream wrapper around any `Stream` object."],["StreamCat","A concatenation of other `Stream` objects."],["StreamFile","A `Stream` wrapper around standard-c FILE pointers."],["StreamFilter","A `Stream` which passes data through any `Filter` objects."],["StreamFs","A `Stream` wrapper around POSIX file descriptors."],["StreamGIO","A `Stream` wrapper around GLib's GIO streams."],["StreamMem","A memory-backed `Stream`."],["StreamMmap","A memory-mapped `Stream`."],["StreamNull","A `Stream` which has no backing store."],["StreamPipe","A `Stream` wrapper around pipes."],["TextPart","A text MIME part object."],["Variant","A generic immutable value capable of carrying various types."],["VariantType","Describes `Variant` types."]],"trait":[["ApplicationPkcs7MimeExt","Trait containing all `ApplicationPkcs7Mime` methods."],["AutocryptHeaderExt","Trait containing all `AutocryptHeader` methods."],["AutocryptHeaderListExt","Trait containing all `AutocryptHeaderList` methods."],["CertificateExt","Trait containing all `Certificate` methods."],["CertificateListExt","Trait containing all `CertificateList` methods."],["ContentDispositionExt","Trait containing all `ContentDisposition` methods."],["ContentTypeExt","Trait containing all `ContentType` methods."],["CryptoContextExt","Trait containing all `CryptoContext` methods."],["DataWrapperExt","Trait containing all `DataWrapper` methods."],["DecryptResultExt","Trait containing all `DecryptResult` methods."],["FilterBestExt","Trait containing all `FilterBest` methods."],["FilterExt","Trait containing all `Filter` methods."],["FilterGZipExt","Trait containing all `FilterGZip` methods."],["FilterWindowsExt","Trait containing all `FilterWindows` methods."],["FilterYencExt","Trait containing all `FilterYenc` methods."],["HeaderExt","Trait containing all `Header` methods."],["HeaderListExt","Trait containing all `HeaderList` methods."],["InternetAddressExt","Trait containing all `InternetAddress` methods."],["InternetAddressListExt","Trait containing all `InternetAddressList` methods."],["InternetAddressMailboxExt","Trait containing all `InternetAddressMailbox` methods."],["MessageExt","Trait containing all `Message` methods."],["MessagePartExt","Trait containing all `MessagePart` methods."],["MessagePartialExt","Trait containing all `MessagePartial` methods."],["MultipartEncryptedExt","Trait containing all `MultipartEncrypted` methods."],["MultipartExt","Trait containing all `Multipart` methods."],["MultipartSignedExt","Trait containing all `MultipartSigned` methods."],["ObjectExt","Trait containing all `Object` methods."],["ParamExt","Trait containing all `Param` methods."],["ParamListExt","Trait containing all `ParamList` methods."],["ParserExt","Trait containing all `Parser` methods."],["PartExt","Trait containing all `Part` methods."],["SignatureExt","Trait containing all `Signature` methods."],["SignatureListExt","Trait containing all `SignatureList` methods."],["StreamCatExt","Trait containing all `StreamCat` methods."],["StreamExt","Trait containing all `Stream` methods."],["StreamFileExt","Trait containing all `StreamFile` methods."],["StreamFilterExt","Trait containing all `StreamFilter` methods."],["StreamFsExt","Trait containing all `StreamFs` methods."],["StreamGIOExt","Trait containing all `StreamGIO` methods."],["StreamMemExt","Trait containing all `StreamMem` methods."],["StreamMmapExt","Trait containing all `StreamMmap` methods."],["StreamNullExt","Trait containing all `StreamNull` methods."],["StreamPipeExt","Trait containing all `StreamPipe` methods."],["TextPartExt","Trait containing all `TextPart` methods."]]});