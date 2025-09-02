use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Write;
use std::os::raw::{c_char, c_int, c_void};
use std::slice;

use crate::CompoundFile;

// We use a type alias to make it clear that this is an opaque pointer.
pub type OpaqueCompoundFile = c_void;

#[repr(C)]
pub struct CfbString {
    ptr: *mut c_char,
}

impl Drop for CfbString {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let _ = CString::from_raw(self.ptr);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn cfb_create_compound_file(path: *const c_char) -> c_int {
    let path_str = unsafe {
        if path.is_null() {
            return -1;
        }
        CStr::from_ptr(path)
    };

    let path = match path_str.to_str() {
        Ok(s) => s,
        Err(_) => return -2,
    };

    // Use the crate's top-level create function which takes a path.
    match crate::create(path) {
        Ok(_) => 0,
        Err(_) => -3,
    }
}

#[no_mangle]
pub extern "C" fn cfb_open_compound_file(path: *const c_char) -> *mut OpaqueCompoundFile {
    let path_str = unsafe {
        if path.is_null() {
            return std::ptr::null_mut();
        }
        CStr::from_ptr(path)
    };

    let path = match path_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    // Use the crate's top-level open function which takes a path.
    match crate::open_rw(path) {
        Ok(cf) => Box::into_raw(Box::new(cf)) as *mut OpaqueCompoundFile,
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn cfb_close_compound_file(cf: *mut OpaqueCompoundFile) {
    if !cf.is_null() {
        unsafe {
            let _ = Box::from_raw(cf as *mut CompoundFile<File>);
        }
    }
}

#[no_mangle]
pub extern "C" fn cfb_create_storage(cf: *mut OpaqueCompoundFile, path: *const c_char) -> c_int {
    let cf = unsafe {
        if cf.is_null() {
            return -1;
        }
        &mut *(cf as *mut CompoundFile<File>)
    };

    let path_str = unsafe {
        if path.is_null() {
            return -2;
        }
        CStr::from_ptr(path)
    };

    let path = match path_str.to_str() {
        Ok(s) => s,
        Err(_) => return -3,
    };

    match cf.create_storage(path) {
        Ok(_) => 0,
        Err(_) => -4,
    }
}

#[no_mangle]
pub extern "C" fn cfb_create_stream(
    cf: *mut OpaqueCompoundFile,
    path: *const c_char,
    data: *const u8,
    len: usize,
) -> c_int {
    let cf = unsafe {
        if cf.is_null() {
            return -1;
        }
        &mut *(cf as *mut CompoundFile<File>)
    };

    let path_str = unsafe {
        if path.is_null() {
            return -2;
        }
        CStr::from_ptr(path)
    };

    let path = match path_str.to_str() {
        Ok(s) => s,
        Err(_) => return -3,
    };

    let data_slice = unsafe {
        if data.is_null() {
            return -4;
        }
        slice::from_raw_parts(data, len)
    };

    match cf.create_stream(path) {
        Ok(mut stream) => match stream.write_all(data_slice) {
            Ok(_) => 0,
            Err(_) => -5,
        },
        Err(_) => -6,
    }
}
