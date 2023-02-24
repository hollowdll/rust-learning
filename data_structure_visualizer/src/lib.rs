// Small data structure visualizer.
// Visualize data structures in your terminal
// by drawing a small graph/table view of it.

//#![allow(unused)]

#[derive(Debug)]
pub struct DataStructure {
    pub field_1: String,
    pub field_2: String,
    pub field_3: String,
    pub field_4: String,
}

impl DataStructure {
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
}

impl DataStructure {
    // Associated functions
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