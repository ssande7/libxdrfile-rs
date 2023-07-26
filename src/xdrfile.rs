use std::ffi::{c_char, c_int, c_uint, c_uchar, c_short, c_ushort, c_float, c_double};


// NOTE: once `extern_types` feature is enabled, this could be `extern "C" type XDRFILE`
#[repr(C)]
pub struct XDRFILE {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum XDRStatus {
    exdrOK,
    exdrHEADER,
    exdrSTRING,
    exdrDOUBLE,
    exdrINT,
    exdrFLOAT,
    exdrUINT,
    exdr3DX,
    exdrCLOSE,
    exdrMAGIC,
    exdrNOMEM,
    exdrENDOFFILE,
    exdrFILENOTFOUND,
    exdrNR,
}

pub const DIM: usize = 3;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct rvec(pub [c_float; DIM]);

impl rvec {
    pub fn new() -> Self {
        rvec([0 as c_float; DIM])
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct matrix(pub [[c_float; DIM]; DIM]);

impl matrix {
    pub fn new() -> Self {
        matrix([[0 as c_float; DIM]; DIM])
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct mybool(pub c_int);

#[link(name="xdrfile")]
extern "C" {

    /// Open a portable binary file, just like C fopen()
    /// `mode` should be `"r"` for reading, `"w"` for writing, or `"a"` for appending.
    /// Returns a `NULL` pointer if an error occurred
    pub fn xdrfile_open(path: *const c_char, mode: *const c_char) -> *mut XDRFILE;

    /// Close a previously opened portable binary file, just like C fclose()
    /// Returns 0 on success, non-zero on error
    pub fn xdrfile_close(xfp: *mut XDRFILE) -> c_int;

    /// Read one or more `c_char` type variable(s)
    /// `ptr` is a pointer to memory where data should be written.
    /// `ndata` is the number of characters to read.
    /// `xfp` is a handle to the portable binary file created with `xdrfile_open()`
    /// Returns the number of characters read
    pub fn xdrfile_read_char(ptr: *mut c_char, ndata: c_int, xfp: *mut XDRFILE) -> c_int;

    /// Write one or more `c_char` type variable(s)
    /// `ptr` is a pointer to memory where the data should be read
    /// `ndata` is the number of characters to write
    /// `xfp` is a handle to the portable binary file created with `xdrfile_open()`
    /// Returns the number of characters written
    pub fn xdrfile_write_char(ptr: *mut c_char, ndata: c_int, xfp: *mut XDRFILE) -> c_int;

    /// Read one or more `c_uchar` type variable(s)
    /// `ptr` is a pointer to memory where data should be written.
    /// `ndata` is the number of characters to read.
    /// `xfp` is a handle to the portable binary file created with `xdrfile_open()`
    /// Returns the number of unsigned characters read
    pub fn xdrfile_read_uchar(ptr: *mut c_uchar, ndata: c_int, xfp: *mut XDRFILE) -> c_int;

    /// Write one or more `c_uchar` type variable(s)
    /// `ptr` is a pointer to memory where the data should be read
    /// `ndata` is the number of characters to write
    /// `xfp` is a handle to the portable binary file created with `xdrfile_open()`
    /// Returns the number of unsigned characters written
    pub fn xdrfile_write_uchar(ptr: *mut c_uchar, ndata: c_int, xfp: *mut XDRFILE) -> c_int;

    /// Read one or more `c_short` type variable(s)
    /// `ptr` is a pointer to memory where data should be written.
    /// `ndata` is the number of shorts to read.
    /// `xfp` is a handle to the portable binary file created with `xdrfile_open()`
    /// Returns the number of shorts read
    pub fn xdrfile_read_short(ptr: *mut c_short, ndata: c_int, xfp: *mut XDRFILE) -> c_int;

    /// Write one or more `c_short` type variable(s)
    /// `ptr` is a pointer to memory where the data should be read
    /// `ndata` is the number of shorts to write
    /// `xfp` is a handle to the portable binary file created with `xdrfile_open()`
    /// Returns the number of shorts written
    pub fn xdrfile_write_short(ptr: *mut c_short, ndata: c_int, xfp: *mut XDRFILE) -> c_int;

    /// Read one or more `c_ushort` type variable(s)
    /// `ptr` is a pointer to memory where data should be written.
    /// `ndata` is the number of unsigned shorts to read.
    /// `xfp` is a handle to the portable binary file created with `xdrfile_open()`
    /// Returns the number of unsigned shorts read
    pub fn xdrfile_read_ushort(ptr: *mut c_ushort, ndata: c_int, xfp: *mut XDRFILE) -> c_int;

    /// Write one or more `c_ushort` type variable(s)
    /// `ptr` is a pointer to memory where the data should be read
    /// `ndata` is the number of unsigned shorts to write
    /// `xfp` is a handle to the portable binary file created with `xdrfile_open()`
    /// Returns the number of unsigned shorts written
    pub fn xdrfile_write_ushort(ptr: *mut c_ushort, ndata: c_int, xfp: *mut XDRFILE) -> c_int;

    /// Read one or more `c_int` type variable(s)
    /// `ptr` is a pointer to memory where data should be written.
    /// `ndata` is the number of ints to read.
    /// `xfp` is a handle to the portable binary file created with `xdrfile_open()`
    /// Returns the number of integers read
    ///
    /// NOTE: No routines are provided for reading/writing 64-bit integers, since:
    ///     - Not all XDR implementations support it
    ///     - Not all machines have 64-bit integers
    pub fn xdrfile_read_int(ptr: *mut c_int, ndata: c_int, xfp: *mut XDRFILE) -> c_int;

    /// Write one or more `c_int` type variable(s)
    /// `ptr` is a pointer to memory where the data should be read
    /// `ndata` is the number of ints to write
    /// `xfp` is a handle to the portable binary file created with `xdrfile_open()`
    /// Returns the number of integers written
    ///
    /// NOTE: No routines are provided for reading/writing 64-bit integers, since:
    ///     - Not all XDR implementations support it
    ///     - Not all machines have 64-bit integers
    pub fn xdrfile_write_int(ptr: *mut c_int, ndata: c_int, xfp: *mut XDRFILE) -> c_int;

    /// Read one or more `c_uint` type variable(s)
    /// `ptr` is a pointer to memory where data should be written.
    /// `ndata` is the number of unsigned ints to read.
    /// `xfp` is a handle to the portable binary file created with `xdrfile_open()`
    /// Returns the number of unsigned integers read
    ///
    /// NOTE: No routines are provided for reading/writing 64-bit integers, since:
    ///     - Not all XDR implementations support it
    ///     - Not all machines have 64-bit integers
    pub fn xdrfile_read_uint(ptr: *mut c_uint, ndata: c_int, xfp: *mut XDRFILE) -> c_int;

    /// Write one or more `c_uint` type variable(s)
    /// `ptr` is a pointer to memory where the data should be read
    /// `ndata` is the number of unsigned ints to write
    /// `xfp` is a handle to the portable binary file created with `xdrfile_open()`
    /// Returns the number of unsigned integers written
    ///
    /// NOTE: No routines are provided for reading/writing 64-bit integers, since:
    ///     - Not all XDR implementations support it
    ///     - Not all machines have 64-bit integers
    pub fn xdrfile_write_uint(ptr: *mut c_uint, ndata: c_int, xfp: *mut XDRFILE) -> c_int;

    /// Read one or more `c_float` type variable(s)
    /// `ptr` is a pointer to memory where data should be written.
    /// `ndata` is the number of floats to read.
    /// `xfp` is a handle to the portable binary file created with `xdrfile_open()`
    /// Returns the number of floats read
    pub fn xdrfile_read_float(ptr: *mut c_float, ndata: c_int, xfp: *mut XDRFILE) -> c_int;

    /// Write one or more `c_float` type variable(s)
    /// `ptr` is a pointer to memory where the data should be read
    /// `ndata` is the number of floats to write
    /// `xfp` is a handle to the portable binary file created with `xdrfile_open()`
    /// Returns the number of floats written
    pub fn xdrfile_write_float(ptr: *mut c_float, ndata: c_int, xfp: *mut XDRFILE) -> c_int;

    /// Read one or more `c_double` type variable(s)
    /// `ptr` is a pointer to memory where data should be written.
    /// `ndata` is the number of doubles to read.
    /// `xfp` is a handle to the portable binary file created with `xdrfile_open()`
    /// Returns the number of floats read
    pub fn xdrfile_read_double(ptr: *mut c_double, ndata: c_int, xfp: *mut XDRFILE) -> c_int;

    /// Write one or more `c_doubles` type variable(s)
    /// `ptr` is a pointer to memory where the data should be read
    /// `ndata` is the number of doubles to write
    /// `xfp` is a handle to the portable binary file created with `xdrfile_open()`
    /// Returns the number of doubles written
    pub fn xdrfile_write_double(ptr: *mut c_double, ndata: c_int, xfp: *mut XDRFILE) -> c_int;

    /// Read a C style string (array of `c_char`)
    /// `ptr` is a pointer to memory where data should be written.
    /// `maxlen` is the maximum length of the string. If no end-of-string is encountered,
    /// one byte less than this is read and end-of-string is appended.
    /// `xfp` is a handle to the portable binary file created with `xdrfile_open()`
    /// Returns the number of characters read, including end-of-string.
    pub fn xdrfile_read_string(ptr: *mut c_char, maxlen: c_int, xfp: *mut XDRFILE) -> c_int;

    /// Write a C style string (array of characters)
    /// `ptr` is a pointer to memory where the data should be read
    /// `xfp` is a handle to the portable binary file created with `xdrfile_open()`
    /// Returns the number of characters written, including end-of-string
    pub fn xdrfile_write_string(ptr: *mut c_char, xfp: *mut XDRFILE) -> c_int;

    /// Read raw bytes from the file (unknown datatype)
    /// `ptr` is a pointer to memory where data should be written.
    /// `nbytes` is the number of bytes to read. No conversion whatsoever is done.
    /// `xfp` is a handle to the portable binary file created with `xdrfile_open()`
    /// Returns the number of bytes read
    pub fn xdrfile_read_opaque(ptr: *mut c_char, nbytes: c_int, xfp: *mut XDRFILE) -> c_int;

    /// Write raw bytes to the file (unknown datatype)
    /// `ptr` is a pointer to memory where the data should be read
    /// `nbytes` is the number of bytes to write. No conversion whatsoever is done.
    /// `xfp` is a handle to the portable binary file created with `xdrfile_open()`
    /// Returns the number of doubles written
    pub fn xdrfile_write_opaque(ptr: *mut c_char, nbytes: c_int, xfp: *mut XDRFILE) -> c_int;

	/// Compress coordiates in a `c_float` array to XDR file
	///
	/// This routine will perform *lossy* compression on the three-dimensional
	/// coordinate data data specified and store it in the XDR file.
	///
	/// The lossy part of the compression consists of multiplying each
	/// coordinate with the precision argument and then rounding to integers.
	/// We suggest a default value of 1000.0, which means you are guaranteed
	/// three decimals of accuracy. The only limitation is that scaled coordinates
	/// must still fit in an integer variable, so if the precision is 1000.0 the
	/// coordinate magnitudes must be less than +-2e6.
	///
	/// `ptr` is a pointer to coordinates to compress (length 3*ncoord)
	/// `ncoord` is the number of coordinate triplets in data
	/// `precision` is the scaling factor for lossy compression. If it is <=0,
	/// the default value of 1000.0 is used.
	/// `xfp` is a handle to a portably binary file
	///
	/// Returns the number of coordinate triplets written.
	/// IMPORTANT: Check that this is equal to ncoord - if it is
	///            negative, an error occured. This should not happen with
	///    	       normal data, but if your coordinates are NaN or very
	///            large (>1e6) it is not possible to use the compression.
	///
	/// WARNING: The compression algorithm is not part of the XDR standard,
	///          and very complicated, so you will need this xdrfile module
	///          to read it later.
    pub fn xdrfile_compress_coord_float(ptr: *mut c_float, ncoord: c_int, precision: c_float, xfp: *mut XDRFILE) -> c_int;

	/// Decompress coordiates from XDR file to array of `c_float`s
	///
	/// This routine will decompress three-dimensional coordinate data previously
	/// stored in an XDR file and store it in the specified array of floats.
	///
	/// The precision used during the earlier compression is read from the file
	/// and returned - you cannot adjust the accuracy at this stage.
	///
	/// `ptr` is a pointer to memory where the decompressed coordinates will be stored (length>= 3*ncoord)
	/// `ncoord` is the maximum number of coordinate triplets to read on input, and is modified to
    /// the actual number of coordinate triplets read on return. If this is smaller than the number
    /// of coordinates in the frame an error will occur.
    ///
	/// The precision used in the previous compression will be written to `precision` on return.
    ///
	/// `xfp` is a handle to a portable binary file
	///
	/// Returns the number of coordinate triplets read. If this is negative, an error occured.
	///
	/// WARNING: Since we cannot count on being able to set/get the
	///          position of large files (>2Gb), it is not possible to
	///          recover from errors by re-reading the frame if the
	///          storage area you provided was too small. To avoid this
	///          from happening, we recommend that you store the number of
	///          coordinates triplet as an integer either in a header or
	///          just before the compressed coordinate data, so you can
	///          read it first and allocated enough memory.
	///
    pub fn xdrfile_decompress_coord_float(ptr: *mut c_float, ncoord: *mut c_int, precision: *mut c_float, xfp: *mut XDRFILE) -> c_int;

	/// Compress coordiates in a `c_double` array to XDR file
	///
	/// This routine will perform *lossy* compression on the three-dimensional
	/// coordinate data data specified and store it in the XDR file. This will
    /// NOT give you any extra precision since the coordinates are compressed.
    /// This routine just avoids allocating a temporary array of `c_float`s.
	///
	/// The lossy part of the compression consists of multiplying each
	/// coordinate with the precision argument and then rounding to integers.
	/// We suggest a default value of 1000.0, which means you are guaranteed
	/// three decimals of accuracy. The only limitation is that scaled coordinates
	/// must still fit in an integer variable, so if the precision is 1000.0 the
	/// coordinate magnitudes must be less than +-2e6.
	///
	/// `ptr` is a pointer to coordinates to compress (length 3*ncoord)
	/// `ncoord` is the number of coordinate triplets in data
	/// `precision` is the scaling factor for lossy compression. If it is <=0,
	/// the default value of 1000.0 is used.
	/// `xfp` is a handle to a portably binary file
	///
	/// Returns the number of coordinate triplets written.
	/// IMPORTANT: Check that this is equal to ncoord - if it is
	///            negative, an error occured. This should not happen with
	///    	       normal data, but if your coordinates are NaN or very
	///            large (>1e6) it is not possible to use the compression.
	///
	/// WARNING: The compression algorithm is not part of the XDR standard,
	///          and very complicated, so you will need this xdrfile module
	///          to read it later.
    pub fn xdrfile_compress_coord_double(ptr: *mut c_double, ncoord: c_int, precision: c_double, xfp: *mut XDRFILE) -> c_int;

	/// Decompress coordiates from XDR file to array of `c_float`s. This will
    /// NOT give you any extra precision since the coordinates are compressed.
    /// This routine just avoids allocating a temporary array of `c_float`s.
	///
	/// This routine will decompress three-dimensional coordinate data previously
	/// stored in an XDR file and store it in the specified array of floats.
	///
	/// The precision used during the earlier compression is read from the file
	/// and returned - you cannot adjust the accuracy at this stage.
	///
	/// `ptr` is a pointer to memory where the decompressed coordinates will be stored (length>= 3*ncoord)
	/// `ncoord` is the maximum number of coordinate triplets to read on input, and is modified to
    /// the actual number of coordinate triplets read on return. If this is smaller than the number
    /// of coordinates in the frame an error will occur.
    ///
	/// The precision used in the previous compression will be written to `precision` on return.
    ///
	/// `xfp` is a handle to a portable binary file
	///
	/// Returns the number of coordinate triplets read. If this is negative, an error occured.
	///
	/// WARNING: Since we cannot count on being able to set/get the
	///          position of large files (>2Gb), it is not possible to
	///          recover from errors by re-reading the frame if the
	///          storage area you provided was too small. To avoid this
	///          from happening, we recommend that you store the number of
	///          coordinates triplet as an integer either in a header or
	///          just before the compressed coordinate data, so you can
	///          read it first and allocated enough memory.
	///
    pub fn xdrfile_decompress_coord_double(ptr: *mut c_double, ncoord: *mut c_int, precision: *mut c_double, xfp: *mut XDRFILE) -> c_int;

    pub fn xdr_tell(xd: *mut XDRFILE) -> i64;
    pub fn xdr_seek(xd: *mut XDRFILE, pos: i64, whence: c_int) -> XDRStatus;
}
