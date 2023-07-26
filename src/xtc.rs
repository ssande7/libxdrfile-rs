use std::ffi::{c_char, c_int, c_float};

use crate::xdrfile::*;

#[link(name="xdrfile")]
extern "C" {
    /// This function returns the number of atoms in the xtc file in `natoms`.
    pub fn read_xtc_natoms(fname: *const c_char, natoms: *mut c_int) -> XDRStatus;
    
    /// Read one frame of an open xtc file
    pub fn read_xtc(xd: *mut XDRFILE, natoms: c_int, step: *mut c_int, time: *mut c_float, r#box: *mut [c_float; 3], x: *mut rvec, prec: *mut c_float) -> XDRStatus;

    /// Write a frame to an xtc file
    // NOTE: C impl uses *mut for box and x, but doesn't mutate them through this call path.
    pub fn write_xtc(xd: *mut XDRFILE, natoms: c_int, step: c_int, time: c_float, r#box: *const [c_float; 3], x: *const rvec, prec: c_float) -> XDRStatus;

    /// Read header information of the current frame
    pub fn xtc_header(xd: *mut XDRFILE, natoms: *mut c_int, step: *mut c_int, time: *mut c_float, bRead: mybool) -> XDRStatus;
}
