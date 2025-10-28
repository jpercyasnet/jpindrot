use std::path::Path;
use std::time::Instant;
use little_exif::metadata::Metadata;
use little_exif::exif_tag::ExifTag;
use image::ImageFormat::Jpeg;
pub fn rotateit(dir_value: String, listofimages: Vec<String>, rottype: i32) -> (u32, String) {
     let mut errcode:u32 = 0;
     let mut errstring  = " ".to_string();
     let mut bolok = true;
     if rottype < 0 || rottype > 2 {
         errstring = format!("********* invalid rottype of {} **********",rottype);
         errcode = 1;
         bolok = false;
     }
     let start_time = Instant::now();
     let lenmg1 = listofimages.len();
     if bolok {
         let mut numrot = 0;
         for indl in 0..lenmg1 {
              numrot = numrot + 1;
              let str_cur_dirfrom = dir_value.clone();
              let fullfrom = str_cur_dirfrom.clone() + "/" + &listofimages[indl].clone();
              if !Path::new(&fullfrom).exists() {
                  errstring = format!("********* rotate ERROR file: {} does not exist **********",fullfrom);
                  errcode = 2;
                  bolok = false;
                  break;
              }
              match image::open(&fullfrom) {   
                  Ok(dyn_img) => {
                      let mut save_meta: Metadata = Metadata::new();
                      let mut have_meta: bool = false;
                      match Metadata::new_from_path(Path::new(&fullfrom)) {
                          Ok(mut jpg_data) => {
                              let yy: Vec<u16> = vec![1];
                              jpg_data.set_tag(ExifTag::Orientation(yy));
                              save_meta = jpg_data;
                              have_meta = true;
                          },
                          Err(_err) => {
                          }
                       }
                       let mut dyn_img1 = dyn_img.rotate270(); 
                       if rottype == 2 {
                           dyn_img1 = dyn_img.rotate180();
                       } else if rottype == 0 {
                           dyn_img1 = dyn_img.rotate90();
                       }
                       match  dyn_img1.save_with_format(fullfrom.clone(), Jpeg) {
                           Ok(_okval)=> {
                               if have_meta {
                                   match save_meta.write_to_file(Path::new(&fullfrom)) {
                                       Ok(_okkval) => {
                                       },
                                       Err(errx) => {
                                           errstring = format!("Failure to save metadata for file: {} error: {}", fullfrom, errx);
                                           errcode = 3;
                                           bolok = false;
                                           break;
                                       }
                                   }
                               }
                           },
                           Err(err) => {
                               errstring = format!("Failure to save file: {}  error: {}", fullfrom, err);
                               errcode = 4;
                               bolok = false;
                               break;
                           }
                       }
                  },
                  Err(err) => {
                      errstring = format!("Failure to open image file: {}  error: {}", fullfrom, err);
                      errcode = 5;
                      bolok = false;
                      break;
                  }
              }
         }
     }
     if bolok {
         let diffx = start_time.elapsed();     
         errstring = format!("rotated {} files in {} seconds", lenmg1, diffx.as_secs());
         errcode = 0;
     }
     (errcode, errstring)
}
