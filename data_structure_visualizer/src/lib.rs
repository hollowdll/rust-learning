// Small data structure visualizer.
// Visualize data structures in your terminal
// by drawing a small graph/table view of it.

//#![allow(unused)]

#[derive(Debug)]
pub struct DataStructure {
    pub name: String,
    pub field_1: String,
    pub field_2: String,
    pub field_3: String,
    pub field_4: String,
}

impl DataStructure {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn field_1(&self) -> &str {
        &self.field_1
    }

    pub fn field_2(&self) -> &str {
        &self.field_2
    }

    pub fn field_3(&self) -> &str {
        &self.field_3
    }

    pub fn field_4(&self) -> &str {
        &self.field_4
    }

    // Example for setters
    // pub fn field(&mut self, val: type)

    pub fn draw(&self) {
        // Disabled
        /*
        let name_len = self.name.chars().count();
        let mut name_hyphens = String::from("");

        for _ in 0..name_len {
            name_hyphens += "-";
        }
        */

        // Final output
        println!(
"
------------------
| Data Structure |
------------------
 name    | {}
 field_1 | {}
 field_2 | {}
 field_3 | {}
 field_4 | {}
",
self.name,
self.field_1,
self.field_2,
self.field_3,
self.field_4,
        );
    }
}

pub struct Config {}

impl Config {

}

pub struct DataStructurePool {}

impl DataStructurePool {

}



#[cfg(test)]
mod tests {

}