use iced::widget::{row, checkbox, text, button, container, Space};
use iced::{Element, Alignment, Length, Center};
use crate::Message;


// #[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct FdFile {
    pub fddescription: String,
    pub fdcompleted: bool,
}

#[derive(Debug, Clone)]
pub enum FdFileMessage {
    Fdcompleted(bool),
}

impl FdFile {
    pub fn new(fddescription: String) -> Self {
        FdFile {
            fddescription,
            fdcompleted: false,
        }
    }

    pub fn update(&mut self, message: FdFileMessage) {
        match message {
            FdFileMessage::Fdcompleted(fdcomp) => {
                self.fdcompleted = fdcomp;
            }
        }
    }

    pub fn view(&self, _i: usize) -> Element<FdFileMessage> {
                let checkbox = checkbox(
                    &self.fddescription,
                    self.fdcompleted).on_toggle(FdFileMessage::Fdcompleted);
                row![
                    checkbox,
                ]
                .spacing(20)
                .align_y(Alignment::Center)
                .into()

    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fdfilter {
    All,
    Active,
    Completed,
}

impl Default for Fdfilter {
    fn default() -> Self {
        Fdfilter::All
    }
}

impl Fdfilter {
    pub fn matches(&self, fdfile: &FdFile) -> bool {
        match self {
            Fdfilter::All => true,
            Fdfilter::Active => !fdfile.fdcompleted,
            Fdfilter::Completed => fdfile.fdcompleted,
        }
    }
}

pub fn fdview_controls(fdfiles: &[FdFile], current_filter: Fdfilter) -> Element<Message> {
    let files_left = fdfiles.iter().filter(|fdfile| fdfile.fdcompleted).count();
    let filter_button = |label, fdfilter, current_filter| {
        let label = text(label).size(16);

        let button = button(label).style(if fdfilter == current_filter {
            button::primary
        } else {
            button::text
        });
        button.on_press(Message::FdFilterChanged(fdfilter)).padding(8)
    };
        row![Space::with_width(Length::Fixed(20.0)),
            text(format!(
            "{} {} selected",
            files_left,
            if files_left == 1 { "file" } else { "files" }
        ))
        .size(16),
            filter_button("All", Fdfilter::All, current_filter),
            filter_button("Not Selected", Fdfilter::Active, current_filter),
            filter_button("Selected", Fdfilter::Completed, current_filter,),
        ]
        .width(Length::Shrink)
        .spacing(10)
    .align_y(Alignment::Center)
    .into()
}

pub fn fdempty_message(message: &str) -> Element<'_, Message> {
    container(
        text(message)
            .width(Length::Fill)
            .size(25)
            .align_x(Center)
            .color([0.7, 0.7, 0.7]),
    )
    .width(Length::Fill)
    .height(Length::Fixed(200.0))
    .into()
}
