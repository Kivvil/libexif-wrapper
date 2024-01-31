pub mod exif;

pub mod bindings {
    pub use libexif_bindings::*;
}

#[cfg(test)]
mod tests {
    use crate::exif::Exif;

    #[test]
    fn read_exif_test() {
        let exif = Exif::from_file("test_resources/DSC_5613.jpg").unwrap();
        let make = exif
            .get_entry_value(
                libexif_bindings::ExifIfd_EXIF_IFD_0,
                libexif_bindings::ExifTag_EXIF_TAG_MAKE,
            )
            .unwrap();
        let model = exif
            .get_entry_value(
                libexif_bindings::ExifIfd_EXIF_IFD_0,
                libexif_bindings::ExifTag_EXIF_TAG_MODEL,
            )
            .unwrap();
        let datetime_original = exif
            .get_entry_value(
                libexif_bindings::ExifIfd_EXIF_IFD_0,
                libexif_bindings::ExifTag_EXIF_TAG_DATE_TIME,
            )
            .unwrap();
        assert_eq!(&make, "NIKON CORPORATION");
        assert_eq!(&model, "NIKON D7000");
        assert_eq!(&datetime_original, "2023:07:25 05:29:51");
    }
}
