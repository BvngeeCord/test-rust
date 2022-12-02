//Smallest component of the application. Registers a main information source with optional upper and lower info sources.
//Rendered as a rectangle within a somewhere within a window with text inside it.

use crate::info_sources::InfoSource;

struct FormattedInfo {
    pub info_sources: Vec<InfoSource>,
    pub formatter: Option<String>,
}

pub trait Section {
    ///Updates the section (gets new files, whatever), then:
    ///returns a list of string slices, referencing a width-sized portion of the actual data
    fn update_and_fetch(&mut self, width: usize, height: usize) -> Vec<&str>;
}

pub struct MainSection {
    top_info: Option<FormattedInfo>,
    bottom_info: Option<FormattedInfo>, 

    filenames: Vec<String>,
}

impl Section for MainSection {
    fn update_and_fetch(&mut self, width: usize, height: usize) -> Vec<&str> {
        let mut vec: Vec<&str> = Vec::with_capacity(height);
        for filename in &self.filenames {
            vec.push(&filename[0..width]);
        }

        vec
    }
}

pub struct ParentsSection {
    top_info: Option<FormattedInfo>,
    bottom_info: Option<FormattedInfo>, 

    parents: Vec<String>,
}

impl Section for ParentsSection {
    fn update_and_fetch(&mut self, width: usize, height: usize) -> Vec<&str> {
        vec!["parent1", "parent2", "parent3"]
    }
}

// enum PreviewType {
//     Text,
//     Binary,
//     Image,
//     Executable,
// }

pub struct PreviewSection {
    preview: Vec<String>,

}

impl Section for PreviewSection {
    fn update_and_fetch(&mut self, width: usize, height: usize) -> Vec<&str> {
        vec!["preview1", "preview2", "preview3", "preview4"]
    }
}

//other sections, like rename/delete/copy list, or other info sources?
