mod get_dirlistc;
mod fdox;
mod get_dirlistr;
mod get_dirlist;
mod dump_file;
mod listpressi;
mod nextpressi;
mod firstpressi;
mod rotatepressi;
mod rotateit;
extern crate image as create_image;

use listpressi::listpressi;
use nextpressi::nextpressi;
use firstpressi::firstpressi;
use rotatepressi::rotatepressi;
use rotateit::rotateit;

use get_dirlistr::get_dirlistr;
use get_dirlist::get_dirlist;

use get_dirlistc::get_dirlistc;

use crate::fdox::Fdfilter;
use crate::fdox::FdFileMessage;
use crate::fdox::FdFile;
use crate::fdox::fdview_controls;
use crate::fdox::fdempty_message;

use iced::widget::{Column, text, column, button, Row, row, image, Space,
                   text_input, Radio, horizontal_space, container, scrollable, checkbox};
use iced::{Element, Task, Length, Alignment, Color, Center};

use std::env;
use std::path::Path;
// use std::process::Command as stdCommand;

fn main() -> iced::Result {
     let widthxx: f32 = 1300.0;
     let heightxx: f32 = 650.0;
     iced::application(MainX::title, MainX::update, MainX::view)
        .window_size((widthxx, heightxx))
        .run_with(MainX::new)

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageChoice {
    MAIN,
//--------file dialog choice
    GD,
 }

impl Default for PageChoice {
    fn default() -> Self {
        PageChoice::MAIN
    }
}

struct MainX {
    mess_color: Color,
    msg_value: String,
    filter: Filter,
    images: Vec<ImageItem>,
    dir_value: String,
    from_value: String,
    to_value: String,
    size_value: String,
    dirset: u32,
    pagechoice_value: PageChoice,
// --- file dialog variables
    scrol_value: String,
    fdoutdir_value: String,
    fdfilter: Fdfilter,
    fdfiles: Vec<FdFile>,
    fdgetdiritems: bool,

}
#[derive(Clone, Debug)]
enum Message {
    PageRadioSelected(PageChoice),
// main program
    FilterChanged(Filter),
    ImageMessage(usize, ImageMessage),
    ListPressed,
    NextGroupPressed,
    FirstGroupPressed,
    RotateClockwisePressed,
    RotateCounterClockwisePressed,
    Rotate180Pressed,
    FromChanged(String),
    ToChanged(String),
    SizeChanged(String),
//    Size(Size),
// --- file dialog messages
    FdSetDirPressed,
    FdListPressed,
    FdChgDirPressed,
    FdFilterChanged(Fdfilter),
    FdFileMessage(usize, FdFileMessage),
    FdGetDirItemsChk(bool),
}

impl MainX {
    fn new() -> (Self, iced::Task<Message>) {
        let mut parmdir = "no directory".to_string();
        let mut msgclr = Color::from([0.5, 0.5, 1.0]);
        let msgval: String;
        let args: Vec<_> = env::args().collect();
        if args.len() > 1 {
            if Path::new(&args[1]).exists() {
                parmdir = args[1].to_string();
                msgclr = Color::from([0.0, 1.0, 0.0]);
                msgval = "got a existing item. Hopefully a directory".to_string();
            } else {
                msgclr = Color::from([1.0, 0.0, 0.0]);
                msgval = format!("parameter directory of {} does not exist", args[1]);
            }
        } else {
            msgval = format!(" no input parameters");
        }

        (  MainX {
                pagechoice_value: PageChoice::MAIN,
                mess_color: msgclr,
                msg_value: msgval.to_string(),
                dir_value: parmdir.to_string(),
                dirset: 0,
                size_value: "160".to_string(),
                filter:Filter::All,
                from_value: "1".to_string(),
                to_value: "16".to_string(),
                scrol_value: " No directory selected \n \
                            ".to_string(),
                images:Vec::<ImageItem>::new(),
// --- file dialog variables
                fdoutdir_value: String::new(),
                fdfilter:Fdfilter::All,
                fdfiles:Vec::<FdFile>::new(),
                fdgetdiritems: false,
           },
            Task::none(),
        )

    }

    fn title(&self) -> String {
        String::from("File dialog test")
    }

    fn update(&mut self, message: Message) -> Task<Message>  {
        let command = match message {
            Message::FilterChanged(filter) => {
                      self.filter = filter;

                        Task::none()
            }

            Message::PageRadioSelected(xchoice) => {
                let mut strx: String;  
                match xchoice {
                    PageChoice::MAIN => {
                             strx = "page choice main selected".to_string();
                             self.dirset = 0;
                    },
                    PageChoice::GD => {
                             strx = "get directory selected".to_string();
                             self.dirset = 1;
                             let (errcd, errstr, newdir, listitems) = get_dirlistc(self.dir_value.clone(), self.fdgetdiritems.clone());
                             if errcd == 0 {
                                 self.fdfiles.clear();                         
                                 self.fdoutdir_value = newdir.to_string();
                                 let listitemlen = listitems.len();
                                 let newtoi = listitemlen as i32 ;
                                 for indexi in 0..newtoi {
                                      self.fdfiles.push(FdFile::new(listitems[indexi as usize].clone()));
                                 } 
                                 self.mess_color = Color::from([0.0, 1.0, 0.0]);
                             } else {
                                 self.mess_color = Color::from([1.0, 0.0, 0.0]);
                                 strx = errstr;
                             }
                    },
                };
                self.pagechoice_value = xchoice;
                self.mess_color = Color::from([0.0, 1.0, 0.0]);
                self.msg_value = strx.to_string();
                Task::none()
            }
// main program message
                    Message::ListPressed => {
                       let (errcd, errstr, listitems, from_int1, _to_int1, newtoi, totfiles, icon_int1) = listpressi(self.dir_value.clone(), self.from_value.clone(), self.to_value.clone(), self.size_value.clone());
                       if errcd == 0 {
                           self.msg_value = format!("from: {}   to: {}  of {} images", from_int1, newtoi, totfiles);
                           self.images.clear();                         
                           for indexi in (from_int1 - 1)..newtoi {
                                let fullpath = self.dir_value.clone() + "/" + &listitems[indexi as usize];
                                let newwidth: u32;
                                let newheight: u32;
                                if let Ok((iwidth, iheight)) = create_image::image_dimensions(fullpath.clone()) {
                                    if iwidth > iheight {
                                        newwidth = icon_int1;
                                        newheight = icon_int1 * iheight / iwidth;
                                    } else {
                                        newheight = icon_int1;
                                        newwidth = icon_int1 * iwidth / iheight;
                                    }
                                    let loadimg = create_image::open(fullpath.clone()).unwrap();
                                    let imgbuffer = create_image::imageops::thumbnail(&loadimg, newwidth, newheight);
                                    let rgbconv = imgbuffer.into_vec();
                                    self
                                       .images
                                       .push(ImageItem::new(listitems[indexi as usize].clone(), rgbconv, newwidth, newheight, icon_int1.clone()));

                                }
                            }
                            self.mess_color = Color::from([0.0, 1.0, 0.0]);          
                       } else {
                            self.msg_value = errstr.to_string();
                            self.mess_color = Color::from([1.0, 0.0, 0.0]);
                       };
                       Task::none()
                    }
                    Message::NextGroupPressed => {
                       let (errcode, errstrx, fromstr, tostr) = nextpressi(self.dir_value.clone(), self.from_value.clone(), self.to_value.clone());
                       if errcode == 0 {
                           let (errcd, errstr, listitems, from_int1, _to_int1, newtoi, totfiles, icon_int1) = listpressi(self.dir_value.clone(), fromstr.clone(), tostr.clone(), self.size_value.clone());
                           if errcd == 0 {
                               self.msg_value = format!("from: {}   to: {}  of {} images", from_int1, newtoi, totfiles);
                               self.images.clear();                         
                               for indexi in (from_int1 - 1)..newtoi {
                                    let fullpath = self.dir_value.clone() + "/" + &listitems[indexi as usize];
                                    let newwidth: u32;
                                    let newheight: u32;
                                    if let Ok((iwidth, iheight)) = create_image::image_dimensions(fullpath.clone()) {
                                        if iwidth > iheight {
                                            newwidth = icon_int1;
                                            newheight = icon_int1 * iheight / iwidth;
                                        } else {
                                            newheight = icon_int1;
                                            newwidth = icon_int1 * iwidth / iheight;
                                        }
                                        let loadimg = create_image::open(fullpath.clone()).unwrap();
                                        let imgbuffer = create_image::imageops::thumbnail(&loadimg, newwidth, newheight);
                                        let rgbconv = imgbuffer.into_vec();
                                        self
                                           .images
                                           .push(ImageItem::new(listitems[indexi as usize].clone(), rgbconv, newwidth, newheight, icon_int1.clone()));
                                    }
                               }
                               self.mess_color = Color::from([0.0, 1.0, 0.0]);          
                               self.from_value = fromstr.to_string();
                               self.to_value = tostr.to_string();
                           } else {
                               self.msg_value = errstr.to_string();
                               self.mess_color = Color::from([1.0, 0.0, 0.0]);
                           }
                       } else {
                          self.msg_value = errstrx.to_string();
                          self.mess_color = Color::from([1.0, 0.0, 0.0]);
                       };
                       Task::none()
                    }
                    Message::FirstGroupPressed => {
                       let (errcode, errstrx, fromstr, tostr) = firstpressi(self.dir_value.clone(), self.from_value.clone(), self.to_value.clone());
                       if errcode == 0 {
                           let (errcd, errstr, listitems, from_int1, _to_int1, newtoi, totfiles, icon_int1) = listpressi(self.dir_value.clone(), fromstr.clone(), tostr.clone(), self.size_value.clone());
                           if errcd == 0 {
                               self.msg_value = format!("from: {}   to: {}  of {} images", from_int1, newtoi, totfiles);
                               self.images.clear();                         
                               for indexi in (from_int1 - 1)..newtoi {
                                    let fullpath = self.dir_value.clone() + "/" + &listitems[indexi as usize];
                                    let newwidth: u32;
                                    let newheight: u32;
                                    if let Ok((iwidth, iheight)) = create_image::image_dimensions(fullpath.clone()) {
                                        if iwidth > iheight {
                                            newwidth = icon_int1;
                                            newheight = icon_int1 * iheight / iwidth;
                                        } else {
                                            newheight = icon_int1;
                                            newwidth = icon_int1 * iwidth / iheight;
                                        }
                                        let loadimg = create_image::open(fullpath.clone()).unwrap();
                                        let imgbuffer = create_image::imageops::thumbnail(&loadimg, newwidth, newheight);
                                        let rgbconv = imgbuffer.into_vec();
                                        self
                                           .images
                                           .push(ImageItem::new(listitems[indexi as usize].clone(), rgbconv, newwidth, newheight, icon_int1.clone()));
                                    }
                               }
                               self.mess_color = Color::from([0.0, 1.0, 0.0]);          
                               self.from_value = fromstr.to_string();
                               self.to_value = tostr.to_string();
                           } else {
                              self.msg_value = errstr.to_string();
                              self.mess_color = Color::from([1.0, 0.0, 0.0]);
                           }
                       } else {
                          self.msg_value = errstrx.to_string();
                          self.mess_color = Color::from([1.0, 0.0, 0.0]);
                       };
                       Task::none()
                    }
                    Message::RotateClockwisePressed => {
                       let errfnd: i32;
                       let mut listofimages: Vec<String> = Vec::new();
                       let images_left = self.images.iter().filter(|imageitem| imageitem.completed).count();
                       if images_left < 1 {
                           self.msg_value = "no images selected".to_string();
                           self.mess_color = Color::from([1.0, 0.0, 0.0]);
                           errfnd = 1;
                       } else {
                           for imagesy in self.images.iter() {
                                if imagesy.completed {
                                    listofimages.push(imagesy.description.clone());
                                }
                           }
                           let (errcode, errstr) = rotatepressi(self.dir_value.clone(), listofimages.clone());
                           if errcode == 0 {
                               self.msg_value = format!("{} images selected {}", images_left, listofimages[0]);
                               self.mess_color = Color::from([0.0, 1.0, 0.0]);
                               errfnd = 0;
                           } else {
                               self.msg_value = errstr.to_string();
                               self.mess_color = Color::from([1.0, 0.0, 0.0]);
                               errfnd = 2;
                           }
                       }
                       if errfnd == 0 {
                           let (errcd1, errstr1) = rotateit(self.dir_value.clone(), listofimages.clone(), 0);
                           self.msg_value = errstr1;
                           if errcd1 == 0 {
                               self.mess_color = Color::from([0.0, 1.0, 0.0]);
                           } else {
                               self.mess_color = Color::from([1.0, 0.0, 0.0]);
                           }
                       }
                       Task::none()
                    }
                    Message::RotateCounterClockwisePressed => {
                       let errfnd: i32;
                       let mut listofimages: Vec<String> = Vec::new();
                       let images_left = self.images.iter().filter(|imageitem| imageitem.completed).count();
                       if images_left < 1 {
                           self.msg_value = "no images selected".to_string();
                           self.mess_color = Color::from([1.0, 0.0, 0.0]);
                           errfnd = 1;
                       } else {
                           for imagesy in self.images.iter() {
                                if imagesy.completed {
                                    listofimages.push(imagesy.description.clone());
                                }
                           }
                           let (errcode, errstr) = rotatepressi(self.dir_value.clone(), listofimages.clone());
                           if errcode == 0 {
                               self.msg_value = format!("{} images selected {}", images_left, listofimages[0]);
                               self.mess_color = Color::from([0.0, 1.0, 0.0]);
                               errfnd = 0;
                           } else {
                               self.msg_value = errstr.to_string();
                               self.mess_color = Color::from([1.0, 0.0, 0.0]);
                               errfnd = 2;
                           }
                       }
                       if errfnd == 0 {
                           let (errcd1, errstr1) = rotateit(self.dir_value.clone(), listofimages.clone(), 1);
                           self.msg_value = errstr1;
                           if errcd1 == 0 {
                               self.mess_color = Color::from([0.0, 1.0, 0.0]);
                           } else {
                               self.mess_color = Color::from([1.0, 0.0, 0.0]);
                           }
                       }
                       Task::none()
                    }
                    Message::Rotate180Pressed => {
                       let errfnd: i32;
                       let mut listofimages: Vec<String> = Vec::new();
                       let images_left = self.images.iter().filter(|imageitem| imageitem.completed).count();
                       if images_left < 1 {
                           self.msg_value = "no images selected".to_string();
                           self.mess_color = Color::from([1.0, 0.0, 0.0]);
                           errfnd = 1;
                       } else {
                           for imagesy in self.images.iter() {
                                if imagesy.completed {
                                    listofimages.push(imagesy.description.clone());
                                }
                           }
                           let (errcode, errstr) = rotatepressi(self.dir_value.clone(), listofimages.clone());
                           if errcode == 0 {
                               self.msg_value = format!("{} images selected {}", images_left, listofimages[0]);
                               self.mess_color = Color::from([0.0, 1.0, 0.0]);
                               errfnd = 0;
                           } else {
                               self.msg_value = errstr.to_string();
                               self.mess_color = Color::from([1.0, 0.0, 0.0]);
                               errfnd = 2;
                           }
                       }
                       if errfnd == 0 {
                           let (errcd1, errstr1) = rotateit(self.dir_value.clone(), listofimages.clone(), 2);
                           self.msg_value = errstr1;
                           if errcd1 == 0 {
                               self.mess_color = Color::from([0.0, 1.0, 0.0]);
                           } else {
                               self.mess_color = Color::from([1.0, 0.0, 0.0]);
                           }
                       }
                       Task::none()
                    }
                    Message::FromChanged(value) => { self.from_value = value; Task::none() }
                    Message::ToChanged(value) => { self.to_value = value; Task::none() }
                    Message::SizeChanged(value) => { self.size_value = value; Task::none() }
                    Message::ImageMessage(i, image_message) => {
                        if let Some(image) = self.images.get_mut(i) {

                            image.update(image_message);

                               Task::none()
                        } else {
                            Task::none()
                        }
                    }
// -------------------------
// --- file dialog messages
            Message::FdFilterChanged(fdfilter) => {
                self.fdfilter = fdfilter;
                Task::none()
            }
            Message::FdFileMessage(i, fdfile_message) => {
                if let Some(fdfile) = self.fdfiles.get_mut(i) {
                    fdfile.update(fdfile_message);
                    Task::none()
                } else {
                    Task::none()
                }
            }
            Message::FdListPressed => {
                let (errcd, errstr, newdir, listitems) = get_dirlistc(self.fdoutdir_value.clone(), self.fdgetdiritems.clone());
                self.msg_value = errstr.to_string();
                if errcd == 0 {
                    self.fdfiles.clear();                         
                    self.fdoutdir_value = newdir.to_string();
                    let listitemlen = listitems.len();
                    let newtoi = listitemlen as i32 ;
                    for indexi in 0..newtoi {
                         self.fdfiles.push(FdFile::new(listitems[indexi as usize].clone()));
                    } 
                    self.mess_color = Color::from([0.0, 1.0, 0.0]);
                } else {
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                }
                Task::none()
            }
            Message::FdSetDirPressed => {
                let a_dir: String = self.fdoutdir_value.clone();
                if Path::new(&a_dir).exists() {
                    self.msg_value = format!("directory has been set with {}", a_dir);
                    if self.dirset == 1 {
                        self.dir_value = a_dir.clone();
                        let dir_path = Path::new(&a_dir);
                        let (errcd, errstr, newliststr) = get_dirlist(dir_path.to_path_buf());
                        if errcd == 0 {
                            self.scrol_value  = newliststr.to_string();
                            self.dir_value = a_dir.to_string();
                            self.mess_color = Color::from([0.0, 1.0, 0.0]);
                            self.pagechoice_value = PageChoice::MAIN;
                        } else {
                            self.mess_color = Color::from([1.0, 0.0, 0.0]);
                            self.msg_value = errstr;
                        }
                    }
                } else {
                    self.msg_value = format!("directory {} does not exist", a_dir);
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                }
                Task::none()
            }
            Message::FdGetDirItemsChk(chked) => {
                self.fdgetdiritems = chked;
                Task::none()
            } 
            Message::FdChgDirPressed => {
                let files_selected = self.fdfiles.iter().filter(|fileitem| fileitem.fdcompleted).count();
                if files_selected < 1 {
                    self.msg_value = "no item selected".to_string();
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                } else if files_selected > 1 {
                    self.msg_value = "more than 1 item selected".to_string();
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                } else {
                    let mut itemstr: String = " ".to_string();
                    for filesy in self.fdfiles.iter() {
                         if filesy.fdcompleted {
                             itemstr = filesy.fddescription.clone();
                         }
                    }
                    let lineparse: Vec<&str> = itemstr[0..].split(" | ").collect();
                    if lineparse[0] != "DIR" {
                        self.msg_value = format!("{} is not a directory", itemstr);
                        self.mess_color = Color::from([1.0, 0.0, 0.0]);
                    } else {
                        let newdirx: String;
                        if lineparse[2] == "..parent" {
                            newdirx = lineparse[1].to_string();
                        } else {
                            newdirx = format!("{}/{}", self.fdoutdir_value, lineparse[1]);
                        }
                        let (errcd, errstr, newdir, listitems) = get_dirlistc(newdirx.clone(), self.fdgetdiritems.clone());
                        self.msg_value = errstr.to_string();
                        if errcd == 0 {
                            self.fdfiles.clear();                         
                            self.fdoutdir_value = newdir.to_string();
                            let listitemlen = listitems.len();
                            let newtoi = listitemlen as i32 ;
                            for indexi in 0..newtoi {
                                 self.fdfiles.push(FdFile::new(listitems[indexi as usize].clone()));
                            } 
                            self.mess_color = Color::from([0.0, 1.0, 0.0]);
                         } else {
                            self.mess_color = Color::from([1.0, 0.0, 0.0]);
                         }
                    }
                }
                Task::none()
            }
        };
        Task::batch(vec![command, Task::none()])
    }

    fn view(&self) -> Element<Message> {
            let selected_pagechoice = Some(self.pagechoice_value);
            let ua = Radio::new(
                     "Main",
                     PageChoice::MAIN,
                     selected_pagechoice,
                     Message::PageRadioSelected,
            ).size(15);
// --- file dialog button
            let ub = Radio::new(
                     "Get Directory: ",
                     PageChoice::GD,
                     selected_pagechoice,
                     Message::PageRadioSelected,
            ).size(15);

            let mut topshow = Column::new().spacing(5);
            topshow = topshow.push(container(row![text("Message:").size(20),
                                              text(&self.msg_value).size(20).color(*&self.mess_color),
                                              ].align_y(Alignment::Center).spacing(5).padding(5),
            ));
            topshow = topshow.push(container(row![
                                              ua
                                              ].align_y(Alignment::Center).spacing(5).padding(5),
            ));
            topshow = topshow.push(container(row![
                                              ub, text(&self.dir_value).size(15)
                                              ].align_y(Alignment::Center).spacing(5).padding(5),
            ));

            let mut subshow = Column::new().spacing(5);

            if self.dirset > 0 {
                let controlsf = fdview_controls(&self.fdfiles, *&self.fdfilter);
                let filtered_files =
                    self.fdfiles.iter().filter(|file| self.fdfilter.matches(file));

                let mut filescol1 = Column::new().spacing(5);
                let mut n = 0;
                if filtered_files.clone().count() == 0 {
                    filescol1 = filescol1.push(container(row![fdempty_message(match self.fdfilter {
                        Fdfilter::All => "No directory selected or no files in directory",
                        Fdfilter::Active => "All files have been selected",
                        Fdfilter::Completed => "No files have been selected" 
                    })]));
                } else {
                    for filesy in self.fdfiles.iter() {
                         if filesy.fdcompleted {
                             if (self.fdfilter == Fdfilter::All) || (self.fdfilter == Fdfilter::Completed) {
                                 filescol1 = filescol1.push(container(row![filesy.view(n).map(move |message| {
                                    Message::FdFileMessage(n, message)
                                   })]));
                             }
                         } else {
                             if (self.fdfilter == Fdfilter::All) || (self.fdfilter == Fdfilter::Active) {
                                 filescol1 = filescol1.push(container(row![filesy.view(n).map(move |message| {
                                    Message::FdFileMessage(n, message)
                                   })]));
                             }
                         }
                         n = n + 1;
                    }
                }
                let mut filesrow = Row::new().spacing(5);
                filesrow = filesrow.push(container(filescol1).padding(5).width(Length::Fixed(400.0)));

                let scrollable_contentf: Element<Message> =
                  Element::from(scrollable(
                    filesrow
                )
                .height(Length::Fill)
               .direction({
                    let scrollbar = scrollable::Scrollbar::new()
                        .width(10)
                        .margin(10)
                        .scroller_width(10);

                    scrollable::Direction::Both {
                        horizontal: scrollbar,
                        vertical: scrollbar,
                    }
                 })
                ); 
                   subshow = subshow.push(container(row![horizontal_space(),
                                                          checkbox("Get Directory Items", self.fdgetdiritems).on_toggle(Message::FdGetDirItemsChk),
                                                          horizontal_space(),
                                                          button("List Directory Button").on_press(Message::FdListPressed),
                                                          horizontal_space(),
                                                          button("Change Directory Button").on_press(Message::FdChgDirPressed), 
                                                          horizontal_space(),
                                              ].align_y(Alignment::Center).spacing(5).padding(5),
                         ));
                    subshow = subshow.push(container(controlsf
                        ),
                           );
                    subshow = subshow.push(container(scrollable_contentf
                        ),
                           );
                    subshow = subshow.push(container(row![horizontal_space(),
                                                                         button("Set Directory Button").on_press(Message::FdSetDirPressed),
                                                                         text(&self.fdoutdir_value).size(15), 
                                                                         horizontal_space(),
                                              ].align_y(Alignment::Center).spacing(5).padding(5),
                        ));
            }

            let mut secshow = Column::new().spacing(5);
            let thrshow = Column::new().spacing(5);

            match self.pagechoice_value  {
// main program
                PageChoice::MAIN => {
                    secshow = secshow.push(container(row![button("List").on_press(Message::ListPressed),
                 button("Next Group").on_press(Message::NextGroupPressed),
                 button("First Group").on_press(Message::FirstGroupPressed),
                 button("Rotate Clockwise").on_press(Message::RotateClockwisePressed),
                 button("Rotate CounterClockwise").on_press(Message::RotateCounterClockwisePressed),
                 button("Rotate 180").on_press(Message::Rotate180Pressed),
                 ].align_y(Alignment::Center).spacing(100).padding(10),
                         ));

                    secshow = secshow.push(container(row![text("From: "),
                 text_input("1", &self.from_value)
                            .on_input(Message::FromChanged).padding(2),
                 text("                    To: "),
                 text_input("16", &self.to_value).on_input(Message::ToChanged).padding(2),
                 text("                    Icon Size: "),
                 text_input("160", &self.size_value).on_input(Message::SizeChanged).padding(2).width(80),
                 ].align_y(Alignment::Center).spacing(20).padding(10),
                         ));
                    let controls = view_controls(&self.images, *&self.filter);
                    let filtered_images =
                        self.images.iter().filter(|imageitem| self.filter.matches(imageitem));

                    let mut imagescol1 = Column::new().spacing(10);
                    let mut imagescol2 = Column::new().spacing(10);
                    let mut imagescol3 = Column::new().spacing(10);
                    let mut imagescol4 = Column::new().spacing(10);
                    let mut colpos = 0;
                    let mut n = 0;
                    if filtered_images.clone().count() == 0 {
                        n = 1;
                        imagescol1 = imagescol1.push(container(row![empty_message(match self.filter {
                            Filter::All => "No directory selected or no files in directory",
                            Filter::Active => "All files have been selected",
                            Filter::Completed => "No files have been selected" 
                        })]));
                    } else {
                        for imagesy in self.images.iter() {
                             if imagesy.completed {
                                 if (self.filter == Filter::All) || (self.filter == Filter::Completed) {
                                   if colpos == 0 {
                                     imagescol1 = imagescol1.push(container(row![imagesy.view(n).map(move |message| {
                                        Message::ImageMessage(n, message)
                                       })]));
                                     colpos  = 1;
                                   } else if colpos == 1 {
                                     imagescol2 = imagescol2.push(container(row![imagesy.view(n).map(move |message| {
                                        Message::ImageMessage(n, message)
                                       })]));
                                     colpos = 2;
                                   } else if colpos == 2 {
                                     imagescol3 = imagescol3.push(container(row![imagesy.view(n).map(move |message| {
                                        Message::ImageMessage(n, message)
                                      })]));
                                     colpos = 3;
                                   } else if colpos == 3 {
                                     imagescol4 = imagescol4.push(container(row![imagesy.view(n).map(move |message| {
                                        Message::ImageMessage(n, message)
                                      })]));
                                     colpos = 0;
                                   }
                                }
                             } else {
                                 if (self.filter == Filter::All) || (self.filter == Filter::Active) {
                                   if colpos == 0 {
                                     imagescol1 = imagescol1.push(container(row![imagesy.view(n).map(move |message| {
                                        Message::ImageMessage(n, message)
                                       })]));
                                     colpos  = 1;
                                   } else if colpos == 1 {
                                     imagescol2 = imagescol2.push(container(row![imagesy.view(n).map(move |message| {
                                        Message::ImageMessage(n, message)
                                       })]));
                                     colpos = 2;
                                   } else if colpos == 2 {
                                     imagescol3 = imagescol3.push(container(row![imagesy.view(n).map(move |message| {
                                        Message::ImageMessage(n, message)
                                       })]));
                                     colpos = 3;
                                   } else if colpos == 3 {
                                     imagescol4 = imagescol4.push(container(row![imagesy.view(n).map(move |message| {
                                        Message::ImageMessage(n, message)
                                       })]));
                                     colpos = 0;
                                   }
                               }
                             }
                             n = n + 1;
                        }
                    }
                    let mut imagesrow = Row::new().spacing(20);
                    imagesrow = imagesrow.push(container(imagescol1).padding(10).width(Length::Fixed(300.0)));
                    if n > 1 {
                        imagesrow = imagesrow.push(container(imagescol2).padding(10).width(Length::Fixed(300.0)));
                    }
                    if n > 2 {
                        imagesrow = imagesrow.push(container(imagescol3).padding(10).width(Length::Fixed(300.0)));
                    }
                    if n > 3 {
                        imagesrow = imagesrow.push(container(imagescol4).padding(10).width(Length::Fixed(300.0)));
                    }

                    let scrollable_content: Element<Message> =
                      Element::from(scrollable(
                        imagesrow
                    )
                    .height(Length::Fill)
                    .direction({
                        let scrollbar = scrollable::Scrollbar::new()
                            .width(10)
                            .margin(10)
                            .scroller_width(10);

                        scrollable::Direction::Both {
                            horizontal: scrollbar,
                            vertical: scrollbar,
                        }
                     })
                    ); 

                    let columnfrom = column![controls, scrollable_content].width(Length::Fill);

                    secshow = secshow.push(container(row![columnfrom].spacing(5).padding(5),
                         ));
                },

// --- file dialog setup
                PageChoice::GD => {
                    secshow = subshow;
                },
// --- file dialog setup
// --- end of file dialog
           }
        column![
         topshow,
         secshow,
         thrshow
         ]
         .padding(1)
        .into()
    }
}


#[derive(Debug, Clone)]
struct ImageItem {
    description: String,
    completed: bool,
    rgbconv: Vec<u8>,
    twidth: u32,
    theight: u32,
    ticon: u32,
}

#[derive(Debug, Clone)]
pub enum ImageMessage {
    Completed(bool),
}

impl ImageItem {

    fn new(description: String, rgbconv: Vec<u8>, twidth:  u32, theight: u32, ticon: u32) -> Self {
        ImageItem {
            description,
            completed: false,
            rgbconv,
            twidth,
            theight,
            ticon,
        }
    }

    fn update(&mut self, message: ImageMessage) {
        match message {
            ImageMessage::Completed(completed) => {
                self.completed = completed;
            }
        }
    }

    fn view(&self, _i: usize) -> Element<ImageMessage> {
        let checkbox = checkbox(
            &self.description,
            self.completed).on_toggle(ImageMessage::Completed).width(Length::Fill);
        let newimage = image::Handle::from_rgba(self.twidth.clone(), self.theight.clone(), self.rgbconv.clone()); 
        let newiconh = self.ticon as f32 + 5.0;
        column![
           container(
        // This should go away once we unify resource loading on native
        // platforms
             image::Viewer::new(newimage)
                 .height(Length::Fixed(newiconh)),
           )
           .width(Length::Fill),
            checkbox,
        ]
        .align_x(Alignment::Center)
        .spacing(5)
        .into()

    }
}

fn view_controls(images: &[ImageItem], current_filter: Filter) -> Element<Message> {
    let images_left = images.iter().filter(|imageitem| imageitem.completed).count();

    let filter_button = |label, filter, current_filter| {
        let label = text(label).size(16);

        let button = button(label).style(if filter == current_filter {
            button::primary
        } else {
            button::text
        });

        button.on_press(Message::FilterChanged(filter)).padding(8)
    };

    row![Space::with_width(20),
        text(format!(
            "{} {} selected",
            images_left,
            if images_left == 1 { "file" } else { "files" }
        ))
        .width(Length::Fill)
        .size(16),
        row![
            filter_button("All", Filter::All, current_filter),
            filter_button("Not Selected", Filter::Active, current_filter),
            filter_button("Selected", Filter::Completed, current_filter,),
        ]
        .width(Length::Shrink)
        .spacing(10)
    ]
    .spacing(20)
    .align_y(Alignment::Center)
    .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Default for Filter {
    fn default() -> Self {
        Filter::All
    }
}

impl Filter {
    fn matches(&self, imageitem: &ImageItem) -> bool {
        match self {
            Filter::All => true,
            Filter::Active => !imageitem.completed,
            Filter::Completed => imageitem.completed,
        }
    }
}
fn empty_message(message: &str) -> Element<'_, Message> {
    container(
        text(message)
            .width(Length::Fill)
            .size(25)
            .align_x(Center)
            .color([0.5, 0.5, 1.0]),
    )
    .width(Length::Fill)
    .height(Length::Fixed(200.0))
    .into()
}
