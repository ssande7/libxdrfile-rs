mod xdrfile;
mod xtc;

#[cfg(test)]
mod tests {
    use std::ffi::{c_char, CString, c_int, c_float};
    use std::mem::MaybeUninit;

    use super::xdrfile::*;
    use super::xtc::*;

    #[test]
    /// Transcribed from libxdrfile/src/tests/test.c
    fn test_xtc() {
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
