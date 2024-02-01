# A safe Rust wrapper aroung libexif.  

Libexif is a C library for extracting EXIF data from picture files. This crate contains a safe  
wrapper around libexif. This crate also exposes unsafe rust bindings for libexif through the [`bindings`] module.  
  
The libexif static library will be linked statically during build time. Libexif needs to be  
installed to build the bindings. The bindings are generated using bindgen. By default, the  
libexif static library is searched using pkg-config. To manually give a file path to the  
libexif.a file, set the `LIBEXIF_STATIC_LIBRARY_PATH` environment variable during build time.  
  
Run 
```bash
cargo add libexif-wrapper 
```
 to add this crate as a dependency.  
  
## Example  
```rust  
use libexif_wrapper::{Exif, ExifIfd, ExifTag};  
  
fn main() {  
    let exif = Exif::from_jpeg_file("test_resources/DSC_5613.jpg").unwrap();  
    let datetime = exif.get_entry_value(ExifIfd::IfdExif, ExifTag::DateTimeOriginal).unwrap();  
    println!("The picture was taken on: {}", datetime);  
}  
```  
  
## Extracting maker notes  
In addition to standardized EXIF tags, most camera manufacturers use the maker note exif tag to store useful information about the  
photo. Maker notes are specific to the camera manufacturer. Note that libexif may not be able  
to decode all maker note tags.  
```rust  
use libexif_wrapper::{Exif, ExifIfd, ExifTag};  
  
fn main() {  
    let exif = Exif::from_jpeg_file("test_resources/DSC_5613.jpg").unwrap();  
    let make = exif.get_entry_value(ExifIfd::Ifd0, ExifTag::Make).unwrap();  
    println!("The picture was taken with a camera by {}", make);  
    // Get maker notes specific to Nikon cameras  
    if make == "NIKON CORPORATION" {  
        // 0x0084 is the maker note tag ID for lens type in Nikon's photos.  
        // A list of maker notes for specific camera manudacturers can be found at https://exiftool.org/TagNames/index.html  
        let lens = exif.get_maker_note(0x0084).unwrap();  
        println!("The lens used was: {}", lens.value);  
    }  
}  
```  
  
## Installing libexif  
```bash  
curl -L https://github.com/libexif/libexif/releases/download/v0.6.24/libexif-0.6.24.tar.bz2 | tar -jsx  
cd libexif-0.6.24  
./configure && make  
sudo make install  
```
  
Libexif is only required during build time, as it is linked statically.  
  
Libexif is licensed under the LGPL-2.1 license  
This crate is licensed under the Apache-2.0 license.  
