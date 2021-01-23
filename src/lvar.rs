pub struct LVar {
    name :String,
    offset: isize,
}

impl LVar {
    pub fn new(name: String, offset: isize) -> Self {
        Self {
            name,
            offset,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_offset(&self) -> isize {
        self.offset
    }
}