//Main window component. Similar to a complete instance of the file manager. Can be arranged in
//tabs, splits (hori/verti), nested splits, splits within tabs, etc. Houses Panes of information

use crate::display::section::ParentsSection;

use super::section::{MainSection, PreviewSection, Section};

pub struct Window<'a> {
    //x y start end stuff? or is handled by screen?

    parents_sections: Option<&'a mut ParentsSection>,
    main_section: &'a mut MainSection,
    preview_section: Option<&'a mut PreviewSection>,
    //other sections?
}

impl<'a> Window<'a> {
    pub fn update_and_fetch(&mut self, start_char_x: u16, end_char_x: u16, start_char_y: u16, end_char_y: u16) -> Vec<&str> {
        let width: usize = (end_char_x - start_char_x) as usize;
        let height: usize = (end_char_y - start_char_y) as usize;

        if let Some(left_pane) = &mut self.parents_sections {
           left_pane.update_and_fetch(width, height);
        }

        self.main_section.update_and_fetch(width, height);

        if let Some(right_pane) = &mut self.preview_section {
            right_pane.update_and_fetch(width, height);
        }

        vec![""]
    }
}
