use crate::cells::Cell;

#[derive(Clone)]
pub struct Map {
    width: usize,
    height: usize,
    current_step: u8,
    cells: Vec<Cell>,
}

impl Map {
    
    pub fn new(width: usize, height: usize) -> Self {
        let length = width * height;

        let cells = vec![Cell::default(); length as usize];

        Map{
            width: width,
            height: height,
            current_step: 0,
            cells,
        }
        
    }

    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn get_step(&self) -> u8 {
        self.current_step
    }
    pub fn set_step(&mut self, step: u8) {
        self.current_step = step;
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

    /// Copy all state from `other` into `self`.
    /// Intended for ping-pong/double-buffer simulation passes.
    pub fn copy_from(&mut self, other: &Map) {
        self.width = other.width;
        self.height = other.height;
        self.current_step = other.current_step;
        self.cells.clone_from(&other.cells);
    }

    pub fn in_bounds(&self, x: usize, z: usize) -> bool {
        x < self.width && z < self.height
    }

    //The index of a grid is (row * number of columns) + column
    pub fn index_of(&self, x: usize, z: usize) -> Option<usize> {
        if !self.in_bounds(x, z) {
            return None;
        }
        Some((z as usize) * (self.width as usize) + (x as usize))
    }

    pub fn get_cell(&self, x: usize, z: usize) -> Option<&Cell> {
        let index = self.index_of(x, z);
        if let Some(index) = index {
            return Some(&self.cells[index]);
        }
        None
    }

    pub fn get_cell_mut(&mut self, x: usize, z: usize) -> Option<&mut Cell> {
        let index = self.index_of(x, z);
        if let Some(index) = index {
            return Some(&mut self.cells[index]);
        }
        None
    }
}



