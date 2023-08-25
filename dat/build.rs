// build.rs

// use tsr_tst_dat_xz::Dat;
// use xz2::read::XzDecoder;
// use tar::Archive;

use mcr::*;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    panic!("hello {:?}", mcr::get_name());
    // panic!("hellooo world!");



    // let duk_tar_xz = Dat::get("duk.tar.xz").unwrap();
    // panic!("hellooo world! {:?}", duk_tar_xz.data.as_ref());

    // let duk_tar_xz = Dat::get("duk.tar.xz").unwrap();
    // let duk_tar_dec = XzDecoder::new(duk_tar_xz.data.as_ref());
    // let mut duk_tar = Archive::new(duk_tar_dec);
    
    // for file in duk_tar.entries().unwrap() {
    //     // Make sure there wasn't an I/O error
    //     let mut file = file.unwrap();
        
    //     // Inspect metadata about the file
    //     println!("{:?}", file.header().path().unwrap());
    //     println!("{}", file.header().size().unwrap());

    //     // // files implement the Read trait
    //     // let mut s = String::new();
    //     // file.read_to_string(&mut s).unwrap();
    //     // println!("{}", s);
    // }
}