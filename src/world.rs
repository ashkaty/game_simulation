
pub struct Map {
    width: u32,
    height: u32,
    current_step: u8,
    cells: Vec<Cell>,
}

impl Map {
    
    pub fn new(width: u32, height: u32) -> Self {
        let length = width * height;

        let mut cells = vec![Cell::empty(), length];

        Map{
            width: width,
            height: height,
            step: 0,
            cells: cells
        }
        
    }

}

pub struct World {
    pub run: bool
}