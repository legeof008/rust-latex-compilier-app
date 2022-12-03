pub struct NotatexFile {
    name : String,
    body : String,
}
impl NotatexFile {
    pub fn name(&self) -> &String {
        &self.name
    }
    fn body(&self) -> &String {
        &self.body
    }
    pub fn new(name: String, body: String) -> Self {
        return NotatexFile { name: name, body: body };
    }
}