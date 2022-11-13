pub enum ListSource {
    Filenames,
    Parents,
    Permissions,
    Other,
}

pub enum InfoSource {
    Preview,
    Permissions,
    Parent,
}

pub trait Source {
    fn fetch(self) -> Vec<String>;
}

impl Source for ListSource {
    fn fetch(self) -> Vec<String> {

        match self {
            ListSource::Filenames => { vec![String::from("file1"), String::from("file2"), String::from("file3")] },

            _ => vec![String::from("default"), String::from("vec")],
        }

    }
}

impl Source for InfoSource {
    fn fetch(self) -> Vec<String> {

        vec![String::from("test\ninfo\nsource")]

    }
}
