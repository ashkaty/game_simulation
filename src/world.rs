use crate::cells::Cell;

pub struct Map {
    width: u32,
    height: u32,
    current_step: u8,
    cells: Vec<Cell>,
}

impl Map {
    
    pub fn new(width: u32, height: u32) -> Self {
        let length = width * height;

        let cells = vec![Cell::empty(); length as usize];

        Map{
            width: width,
            height: height,
            current_step: 0,
            cells,
        }
        
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }
    pub fn get_height(&self) -> u32 {
        self.height
    }
    pub fn get_step(&self) -> u8 {
        self.current_step
    }
    pub fn get_cells(&self) -> &Vec<Cell> {
        &self.cells
    }
    pub fn get_cells_mut(&mut self) -> &mut Vec<Cell> {
        &mut self.cells
    }
    pub fn increment_step(&mut self) {
        self.current_step = self.current_step.wrapping_add(1);
    }

    pub fn in_bounds(&self, x: u32, z: u32) -> bool {
        x < self.width && z < self.height
    }

    //The index of a grid is (row * number of columns) + column
    pub fn index_of(&self, x: u32, z: u32) -> Option<usize> {
        if !self.in_bounds(x, z) {
            return None;
        }
        Some((z as usize) * (self.width as usize) + (x as usize))
    }

    pub fn get_cell(&self, x: u32, z: u32) -> Option<&Cell> {
        let index = self.index_of(x, z);
        if let Some(index) = index {
            return Some(&self.cells[index]);
        }
        None
    }

    pub fn get_cell_mut(&mut self, x: u32, z: u32) -> Option<&mut Cell> {
        let index = self.index_of(x, z);
        if let Some(index) = index {
            return Some(&mut self.cells[index]);
        }
        None
    }
}



