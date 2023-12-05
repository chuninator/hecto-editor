

pub struct FileType {
    name: String, 
    hl_options: HighlightingOptions, 
}
#[derive(Default, Clone, Copy)]
pub struct HighlightingOptions {
    pub numbers: bool,
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            name: String::from("No filetype"),
            hl_options: HighlightingOptions::default(),
        }
    }
}

impl FileType {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn highlighting_options(&self) -> HighlightingOptions {
        self.hl_options
    }

    pub fn from(file_name: &str) -> Self {
        if file_name.ends_with(".rs") {
            return Self {
                name: String::from("Rust"),
                hl_options: HighlightingOptions { numbers: true },
            };
        }
        Self::default()
    }
}