use std::ffi::{CStr, CString};

use libexif_bindings::{
    exif_content_get_entry, exif_data_free, exif_data_new_from_file, exif_entry_get_value,
    ExifData, ExifIfd, ExifTag,
};

pub struct Exif {
    exif_data: *mut ExifData,
}

#[derive(Debug)]
pub enum ExifError {
    FileNotFound,
}

impl Exif {
    pub fn from_file(file_name: &str) -> Result<Self, ExifError> {
        let fname_cstr = CString::new(file_name).unwrap();
        let data = unsafe { exif_data_new_from_file(fname_cstr.as_ptr()) };
        if data == std::ptr::null_mut() {
            return Err(ExifError::FileNotFound);
        }
        Ok(Self { exif_data: data })
    }

    pub fn get_entry_value(&self, ifd: ExifIfd, tag: ExifTag) -> Result<String, ()> {
        let mut buffer: [i8; 1024] = [0; 1024];
        let text_string_pointer = unsafe {
            let entry = exif_content_get_entry((*self.exif_data).ifd[ifd as usize], tag);
            if entry == std::ptr::null_mut() {
                return Err(());
            }
            exif_entry_get_value(entry, buffer.as_mut_ptr(), 1024)
        };

        let string = unsafe { CStr::from_ptr(text_string_pointer) }
            .to_str()
            .unwrap()
            .to_owned();
        Ok(string)
    }
}

impl Drop for Exif {
    fn drop(&mut self) {
        if self.exif_data != std::ptr::null_mut() {
            unsafe { exif_data_free(self.exif_data) }
        }
    }
}
