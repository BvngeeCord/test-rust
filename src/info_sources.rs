pub enum InfoSource {
    Permissions,
    FileSize,
    FileCount, //will need to depend on the section its registered to (left, middle, right)
    DateModified,
    DateCreated,
    AbsolutePath,
}

impl InfoSource {
    fn fetch(&self, width: u32, height: u32) -> String {

        match self {

            _ => "test info source".to_owned()
        }

    }
}
