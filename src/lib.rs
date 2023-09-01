use std::{
    marker::PhantomData,
    ffi::{CStr, c_char, c_int, c_float},
    mem::MaybeUninit, ptr::addr_of_mut
};

pub mod xdr;
pub mod xtc;

use xdr::*;
use xtc::*;

pub mod prelude {
    pub use super::xdr::XDRStatus;
    pub use super::xdr::rvec;
    pub use super::xdr::matrix;
    pub use super::xdr::DIM;
    pub use super::XDRFile;
    pub use super::XTCFrame;
    pub use super::access_mode;
}

pub struct XDRFile<MODE: XDRAccessMode> {
    handle: *mut XDRFILE,
    _mode: PhantomData<MODE>,
}

impl<MODE: XDRAccessMode> XDRFile<MODE> {
    /// Open an xdr file in the specified access mode.
    pub fn open(fname: &CStr) -> Result<Self, XDRStatus> {
        let handle = unsafe {xdrfile_open(fname.as_ptr(), MODE::mode_char()) };
        if handle.is_null() { return Err(XDRStatus::exdrFILENOTFOUND) }
        Ok(Self {
            handle,
            _mode: PhantomData::default(),
        })
    }

    pub fn close(self) {
        // Return value is the return value of C fclose() or exdrCLOSE if the file pointer is null.
        // File pointer can't be null here, and I can't think of a case where the value from
        // fclose() will be useful.
        unsafe { xdrfile_close(self.handle) };
    }
}

impl<MODE: XDRAccessMode> Drop for XDRFile<MODE> {
    fn drop(&mut self) {
        unsafe { xdrfile_close(self.handle) };
    }
}

// TODO: Implement other read/write functions in the xdrfile library
impl<MODE: XDRAccessMode + access_mode::Writable> XDRFile<MODE> {
    /// Write a frame to an xtc file, including the set of atom locations (`x`) the `step`, `time`,
    /// and precision (`prec`).
    /// Returns `Err(XDRStatus::exdrUINT)` if the length of `x` is too large to be safely converted
    /// to a `c_int`.
    pub fn write_xtc(&self, step: i32, time: f32, sim_box: matrix, x: &[rvec], prec: f32) -> Result<(), XDRStatus> {
        let Ok(natoms) = x.len().try_into() else { return Err(XDRStatus::exdrUINT) };
        let result = unsafe {
            write_xtc(
                self.handle,
                natoms,
                step as c_int,
                time as c_float,
                sim_box.0.as_ptr(),
                x.as_ptr(),
                prec as c_float
            )
        };
        if result != XDRStatus::exdrOK {
            return Err(result)
        }
        Ok(())
    }
}

impl XDRFile<access_mode::Read> {
    /// Read the number of atoms from an xtc file
    pub fn read_xtc_natoms(&self) -> Result<usize, XDRStatus> {
        // Save current position
        let fpos = unsafe { xdr_tell(self.handle) };

        // Go to start of file (libxdrfile implementation takes a file name and opens it fresh)
        match unsafe { xdr_seek(self.handle, 0, libc::SEEK_SET) } {
            XDRStatus::exdrOK => (),
            e => return Err(e),
        }

        // Read header to get numatoms
        let mut natoms = MaybeUninit::<c_int>::uninit();
        let mut step = MaybeUninit::<c_int>::uninit();
        let mut time = MaybeUninit::<c_float>::uninit();
        match unsafe {
            xtc_header(
                self.handle,
                natoms.as_mut_ptr(),
                step.as_mut_ptr(),
                time.as_mut_ptr(),
                mybool::TRUE
            )
        } {
            XDRStatus::exdrOK => (),
            e => return Err(e),
        }

        // Jump back to old file position
        match unsafe { xdr_seek(self.handle, fpos, libc::SEEK_SET) } {
            XDRStatus::exdrOK => (),
            e => return Err(e),
        }

        // Make sure natoms is non-negative and return it
        let Ok(natoms) = unsafe { natoms.assume_init() }.try_into() else {
            return Err(XDRStatus::exdrUINT)
        };
        Ok(natoms)
    }

    /// Read a frame from an xtc file
    pub fn read_xtc_reuse(&self, natoms: usize, frame: &mut XTCFrame) -> Result<(), XDRStatus> {
        let Ok(num_atoms) = natoms.try_into() else { return Err(XDRStatus::exdrUINT) };
        frame.x.clear();
        frame.x.reserve(natoms);
        let result = unsafe {
            read_xtc(
                self.handle,
                num_atoms,
                &mut frame.step,
                &mut frame.time,
                frame.sim_box.0.as_mut_ptr(),
                frame.x.as_mut_ptr(),
                &mut frame.prec
            )
        };
        unsafe { frame.x.set_len(natoms); }

        if result != XDRStatus::exdrOK { return Err(result) }

        Ok(())
    }

    /// Read a frame from an xtc file
    pub fn read_xtc(&self, natoms: usize) -> Result<XTCFrame, XDRStatus> {
        let Ok(num_atoms) = natoms.try_into() else { return Err(XDRStatus::exdrUINT) };
        let mut frame = MaybeUninit::<XTCFrame>::uninit();
        let result = unsafe {
            let f = frame.as_mut_ptr();
            addr_of_mut!((*f).sim_box).write(matrix::new());
            addr_of_mut!((*f).x).write(Vec::with_capacity(natoms));
            (*f).x.set_len(num_atoms as usize);
            read_xtc(
                self.handle,
                num_atoms,
                &mut (*f).step,
                &mut (*f).time,
                (*f).sim_box.0.as_mut_ptr(),
                (*f).x.as_mut_ptr(),
                &mut (*f).prec
            )
        };

        if result != XDRStatus::exdrOK { return Err(result) }

        Ok(unsafe {frame.assume_init()})
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct XTCFrame {
    pub step: c_int,
    pub time: c_float,
    pub sim_box: matrix,
    pub prec: c_float,
    pub x: Vec<rvec>,
}

impl XTCFrame {
    pub fn empty() -> Self {
        Self {
            step: 0,
            time: 0.,
            sim_box: matrix::new(),
            prec: 1000.,
            x: Vec::new(),
        }
    }
}

pub mod access_mode {
    pub struct Read;
    pub struct Write;
    pub struct Append;

    pub trait Writable {}
    impl Writable for super::access_mode::Write {}
    impl Writable for super::access_mode::Append {}
}

pub trait XDRAccessMode: seal::Sealed {
    fn mode_char() -> &'static c_char;
}
impl XDRAccessMode for access_mode::Read {
    fn mode_char() -> &'static c_char {
        &('r' as c_char)
    }
}
impl XDRAccessMode for access_mode::Write {
    fn mode_char() -> &'static c_char {
        &('w' as c_char)
    }
}
impl XDRAccessMode for access_mode::Append {
    fn mode_char() -> &'static c_char {
        &('a' as c_char)
    }
}

mod seal {
    pub trait Sealed {}
    impl Sealed for super::access_mode::Read {}
    impl Sealed for super::access_mode::Write {}
    impl Sealed for super::access_mode::Append {}
}



#[cfg(test)]
mod tests {
    use std::ffi::{c_char, CString, c_int, c_float};
    use std::mem::MaybeUninit;
    use super::prelude::*;

    #[test]
    /// Test the safe wrapper for reading/writing xtc files
    fn test_xtc_wrapper() -> Result<(), XDRStatus> {

        let test_file = CString::new("tests/test_wrapper.xtc").unwrap();
        let nframes = 13;
        let natoms1 = 173;
        let step1 = 1993;
        let time1 = 1097.23;
        let prec1 = 1000.0;
        let toler = 1e-3;


        let mut box1 = matrix::new();
        for i in 0..DIM {
            for j in 0..DIM {
                box1.0[i][j] = (i + 1) as c_float * 3.7 as c_float + (j+1) as c_float;
            }
        }

        let mut x1 = Vec::<rvec>::with_capacity(natoms1 as usize);
        for i in 0..natoms1 {
            let mut x1i = rvec::new();
            for j in 0..DIM {
                x1i.0[j] = (i+1) as c_float * 3.7 as c_float + (j+1) as c_float;
            }
            x1.push(x1i);
        }

        {
            println!("Opening xtc file");
            let xtc_write = XDRFile::<access_mode::Write>::open(&test_file)?;
            println!("Writing xtc file");
            for k in 0..nframes {
                xtc_write.write_xtc(step1 + k, time1 + k as f32, box1, &x1[..], prec1)?;
            }
        }

        {
            println!("Reading xtc file");
            let xtc_read = XDRFile::<access_mode::Read>::open(&test_file)?;
            let mut frame = XTCFrame::empty();
            let natoms2 = xtc_read.read_xtc_natoms()?;
            assert_eq!(natoms1, natoms2, "Number of atoms incorrect when reading xtc file");

            let mut k = 0;
            while xtc_read.read_xtc_reuse(natoms2, &mut frame).is_ok() {
                assert_eq!(frame.step - step1, k, "Incorrect step on frame {}", k);
                assert!(f32::abs(frame.time - time1 - k as c_float) <= toler, "Incorrect time on frame {}", k);
                assert!(f32::abs(frame.prec - prec1) <= toler, "Incorrect precision on frame {}", k);
                for i in 0..DIM {
                    for j in 0..DIM {
                        assert!(f32::abs(frame.sim_box.0[i][j] - box1.0[i][j]) <= toler, "Incorrect box on frame {}", k);
                    }
                }
                for i in 0..natoms1 as usize {
                    for j in 0..DIM {
                        assert!(f32::abs(frame.x[i].0[j] - x1[i].0[j]) <= toler, "Incorrect x on frame {}", k);
                    }
                }

                k += 1;
            }
            assert_eq!(xtc_read.read_xtc(natoms2), Err(XDRStatus::exdrENDOFFILE));
        }

        Ok(())
    }

    #[test]
    /// Transcribed from libxdrfile/src/tests/test.c
    fn test_xtc() {
        use super::xdr::*;
        use super::xtc::*;

        let test_file = CString::new("tests/test.xtc").unwrap();
        let nframes: c_int = 13;
        let natoms1: c_int = 173;
        let step1: c_int = 1993;
        let time1: c_float = 1097.23;
        let prec1: c_float = 1000.0;
        let toler: c_float = 1e-3;

        println!("Testing xtc functionality:");

        let mut box1 = matrix::new();
        for i in 0..DIM {
            for j in 0..DIM {
                box1.0[i][j] = (i + 1) as c_float * 3.7 as c_float + (j+1) as c_float;
            }
        }

        let mut x1 = Vec::<rvec>::with_capacity(natoms1 as usize);
        for i in 0..natoms1 {
            let mut x1i = rvec::new();
            for j in 0..DIM {
                x1i.0[j] = (i+1) as c_float * 3.7 as c_float + (j+1) as c_float;
            }
            x1.push(x1i);
        }


        let writemode = 'w' as c_char;
        let xd = unsafe { xdrfile_open(test_file.as_ptr(), &writemode) };

        assert!(!xd.is_null(), "Error opening xdrfile for writing");

        for k in 0..nframes {
            let result = unsafe {write_xtc(xd, natoms1, step1 + k as c_int, time1 + k as c_float, box1.0.as_mut_ptr(), x1.as_mut_ptr(), prec1)};
            assert_eq!(result, XDRStatus::exdrOK, "Error writing xtc file: {:?}", result);
        }
        unsafe { xdrfile_close(xd) };


        let mut natoms2 = MaybeUninit::<c_int>::uninit();
        let result = unsafe {read_xtc_natoms(test_file.as_ptr(), natoms2.as_mut_ptr() as *mut c_int)};
        assert_eq!(result, XDRStatus::exdrOK, "Error reading xtc file: {:?}", result);
        let natoms2 = unsafe {natoms2.assume_init()};
        assert_eq!(natoms1, natoms2, "Number of atoms incorrect when reading xtc file");

        let mut x2 = Vec::<rvec>::with_capacity(natoms2 as usize);

        let readmode = 'r' as c_char;
        let xd = unsafe {xdrfile_open(test_file.as_ptr(), &readmode)};
        let mut k = 0;
        let mut result = XDRStatus::exdrOK;
        let mut box2 = matrix::new();
        while result == XDRStatus::exdrOK {
            let mut step2 = MaybeUninit::<c_int>::uninit();
            let mut time2 = MaybeUninit::<c_float>::uninit();
            let mut prec2 = MaybeUninit::<c_float>::uninit();
            result = unsafe {read_xtc(xd, natoms2, step2.as_mut_ptr(), time2.as_mut_ptr(), box2.0.as_mut_ptr(), x2.as_mut_ptr(), prec2.as_mut_ptr())};
            if result == XDRStatus::exdrENDOFFILE {break}
            assert_eq!(result, XDRStatus::exdrOK, "Error reading frame from xtc file: {:?}", result);
            let step2 = unsafe {step2.assume_init()};
            let time2 = unsafe {time2.assume_init()};
            let prec2 = unsafe {prec2.assume_init()};
            unsafe {x2.set_len(natoms2 as usize)};
            assert_eq!(step2 - step1, k, "Incorrect step on frame {}", k);
            assert!(f32::abs(time2 - time1 - k as c_float) <= toler, "Incorrect time on frame {}", k);
            assert!(f32::abs(prec2 - prec1) <= toler, "Incorrect precision on frame {}", k);
            for i in 0..DIM {
                for j in 0..DIM {
                    assert!(f32::abs(box2.0[i][j] - box1.0[i][j]) <= toler, "Incorrect box on frame {}", k);
                }
            }
            for i in 0..natoms1 as usize {
                for j in 0..DIM {
                    assert!(f32::abs(x2[i].0[j] - x1[i].0[j]) <= toler, "Incorrect x on frame {}", k);
                }
            }

            k += 1;
        }
        unsafe {xdrfile_close(xd)};
    }
}
