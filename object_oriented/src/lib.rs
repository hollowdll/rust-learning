// Rust isn't directly an object oriented language,
// but it does have some OOP features that are
// a bit different than in actual OOP languages.

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

// Encapsulation
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

// Polymorphism and inheritance with trait objects
// TODO