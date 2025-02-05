pub mod advanced; // Import submodule

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}


#[derive(Debug)]
pub struct Point {
    pub vector: Vec<i32>,
}
