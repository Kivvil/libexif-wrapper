use std::ffi::{CStr, CString};

use crate::bindings::{
    exif_content_get_entry, exif_data_free, exif_data_get_mnote_data, exif_data_new_from_data,
    exif_data_new_from_file, exif_entry_get_value, exif_mnote_data_count, exif_mnote_data_get_id,
    exif_mnote_data_get_title, exif_mnote_data_get_value, ExifData,
};

use crate::{exif_tags::ExifTag, ExifIfd};

/// Acts as a safe wrapper around libexif native APIs.
///
/// # Example
/// ```
/// use libexif_wrapper::{Exif, ExifIfd, exif_tags, exif_tags::ExifTag};
/// let exif = Exif::from_jpeg_file("test_resources/DSC_5613.jpg").unwrap();
/// let datetime = exif.get_entry_value(ExifIfd::IfdExif, exif_tags::DATE_TIME_ORIGINAL).unwrap();
/// println!("The picture was taken on: {}", datetime);
/// ```
#[derive(Debug, PartialEq)]
pub struct Exif {
    exif_data: *mut ExifData,
}

unsafe impl Send for Exif {}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ExifError {
    /// An unknown error from libexif.
    ExifFailed,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MakerNoteError {
    /// An unknown error from libexif.
    ExifFailed,
    /// No makernote EXIF tag
    MakerNoteNotFound,
    /// Makernote EXIF tag was not found, but the specific maker note tag was not present in it.
    MNoteTagNotFound,
}

/// Contains data about a maker note tag.
#[derive(Debug, PartialEq, Clone)]
pub struct MakerNoteData {
    /// The tag value
    pub tag_id: u32,
    /// The human readable title
    pub title: String,
    /// The human readable value, eg. f/5.6. May be empty, if no title is found.
    pub value: String,
}

impl Exif {
    /// Create an instance from a JPEG file path.
    pub fn from_jpeg_file(file_name: &str) -> Result<Self, ExifError> {
        let fname_cstr = CString::new(file_name).unwrap();
        let data = unsafe { exif_data_new_from_file(fname_cstr.as_ptr()) };
        if data == std::ptr::null_mut() {
            return Err(ExifError::ExifFailed);
        }
        Ok(Self { exif_data: data })
    }

    /// Create an instance from raw exif data.
    pub fn from_data(exif_data: &[u8]) -> Result<Self, ExifError> {
        let data = unsafe { exif_data_new_from_data(exif_data.as_ptr(), exif_data.len() as u32) };
        if data == std::ptr::null_mut() {
            return Err(ExifError::ExifFailed);
        }
        Ok(Self { exif_data: data })
    }

    /// Get a human readable value of an EXIF tag.
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

    /// Get a maker note tag, Note that maker note tags are specific for camera manufacturers.
    pub fn get_maker_note(&self, mnote_tag: u32) -> Result<MakerNoteData, MakerNoteError> {
        let mnote_data = unsafe { exif_data_get_mnote_data(self.exif_data) };
        if mnote_data == std::ptr::null_mut() {
            return Err(MakerNoteError::MNoteTagNotFound);
        }

        let num = unsafe { exif_mnote_data_count(mnote_data) };
        for i in 0..num {
            if unsafe { exif_mnote_data_get_id(mnote_data, i) } == mnote_tag {
                let mut buffer: [i8; 1024] = [0; 1024];
                let value_string_pointer = unsafe {
                    exif_mnote_data_get_value(
                        mnote_data,
                        i,
                        buffer.as_mut_ptr(),
                        buffer.len() as u32,
                    )
                };
                let title_string_pointer = unsafe { exif_mnote_data_get_title(mnote_data, i) };
                if value_string_pointer == std::ptr::null_mut()
                    || title_string_pointer == std::ptr::null_mut()
                {
                    return Err(MakerNoteError::ExifFailed);
                }
                let value_string = unsafe { CStr::from_ptr(value_string_pointer) }
                    .to_str()
                    .unwrap()
                    .to_owned();
                let title_string = unsafe { CStr::from_ptr(title_string_pointer) }
                    .to_str()
                    .unwrap()
                    .to_owned();
                return Ok(MakerNoteData {
                    tag_id: mnote_tag,
                    title: title_string,
                    value: value_string,
                });
            }
        }
        // Maker note tag was note found.
        Err(MakerNoteError::MNoteTagNotFound)
    }
}

impl Drop for Exif {
    fn drop(&mut self) {
        if self.exif_data != std::ptr::null_mut() {
            unsafe { exif_data_free(self.exif_data) }
        }
    }
}
