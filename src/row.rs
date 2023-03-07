
pub struct Row {
    // content
    content: String
}

impl Row {
    pub fn new() -> Self {
        Row {
            content: "".to_string()
        }
    }

    pub fn get_size(&mut self) -> u16  {return self.content.len() as u16;} 
    pub fn get_content(&mut self) -> String {return self.content.clone()}

    pub fn set_content(&mut self, s: String) {self.content = s}
    pub fn append_content(&mut self, c: char) {self.content.push(c)}
}