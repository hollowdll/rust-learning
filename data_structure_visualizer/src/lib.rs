// Small data structure visualizer.
// Visualize data structures in your terminal
// by drawing a small table view of it.

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

    /// Draw a table view of this data structure instance
    /// in your terminal.
    /// 
    /// `Data Structure` text default output color is yellow.
    /// It can be changed by modifying the escape characters.
    pub fn draw(&self) {
        println!(
"
------------------
| \x1b[93mData Structure\x1b[0m |
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

impl DataStructure {
    /// Draw a table view of this data structure
    /// without creating an instance of it.
    pub fn draw_structure() {
        println!(
"
------------------
| \x1b[93mData Structure\x1b[0m |
------------------
 name
 field_1
 field_2
 field_3
 field_4
"
        );
    }
}
