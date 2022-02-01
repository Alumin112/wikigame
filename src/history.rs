#[derive(Debug)]
pub struct History(Vec<String>, i32);

impl History {
    pub fn empty() -> Self {
        Self(vec![], -1)
    }

    pub fn prev(&mut self) {
        self.1 -= 1;
    }

    pub fn next(&mut self) {
        self.1 += 1;
    }

    pub fn is_last(&self) -> bool {
        if self.1 < 0 {
            return true;
        }
        self.1 == (self.0.len() - 1) as i32
    }

    pub fn is_first(&self) -> bool {
        self.1 <= 0
    }

    pub fn new_next(&mut self, value: &str) {
        if self.1 < 0 || self.0[self.1 as usize] != value {
            self.1 += 1;
            self.0.truncate(self.1 as usize);
            self.0.push(value.to_owned());
        }
    }

    pub fn url(&self) -> String {
        if self.1 < 0 {
            return "?".to_string();
        }
        self.0[self.1 as usize].clone()
    }

    pub fn add_home(&mut self) {
        if self.1 >= 0 {
            self.new_next(&self.0[0].clone());
        }
    }

    pub fn clean(&mut self) {
        self.0.clear();
        self.1 = -1;
    }
}
