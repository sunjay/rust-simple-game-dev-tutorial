#[derive(Debug)]
pub struct Window {
    title: String,
}

impl Window {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_owned(),
        }
    }
}
