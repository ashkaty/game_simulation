use crate::cells::{Cell};
use crate::world::Map;

#[derive(Debug, Copy, Clone)]
pub struct SimConfig {

    pub max_total_moveable_height: u16,

    pub max_transfer_per_pass: u16,

    pub alternate_scan_direction: bool,

    pub max_fire_height: u16,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            max_total_moveable_height: 64,
            max_transfer_per_pass: 4,
            alternate_scan_direction: true,
            max_fire_height: 8,
        }
    }
}


pub struct Simulator {
    pub config: SimConfig,
    next_buffer: Option<Map>,
}

impl Default for Simulator {
    fn default() -> Self {
        Self::new(SimConfig::default())
    }
}

impl Simulator {
    pub fn new(config: SimConfig) -> Self {
        Self {
            config,
            next_buffer: None,
        }
    }


    pub fn advance_tick<'a>(&mut self, world: &'a mut Map) ->  Map {
        let src = world;
        let mut dst = src.clone();

        self.ensure_next_buffer(&src);

        dst.increment_step();
    
        // update_reactions(&mut src,&mut dst, &self.config);
        // update_burnable(&mut src,&mut dst, &self.config);
        update_granular(& src,&mut dst, &self.config);
        // update_liquid(&mut src,&mut dst, &self.config);
        // update_solids(&mut src,&mut dst, &self.config);
    
        dst
    }


    fn ensure_next_buffer(&mut self, world_buffer: &Map) {
    }

    fn get_neighourhood<'a>(&self, world_buffer: &'a mut Map, x: u32, z: u32) -> [Option<&'a Cell>; 4] {
    let cell1a = world_buffer.get_cell(x, z);
    let cell1b = world_buffer.get_cell(x + 1, z);
    let cell2a = world_buffer.get_cell(x, z + 1);
    let cell2b = world_buffer.get_cell(x + 1, z);

    [cell1a, cell1b, cell2a, cell2b]

    } 
}

fn update_granular(src: &Map, dst: &mut Map, cfg: &SimConfig) {
    //calculate the slope of the surface around it
    //if the slope is greater than the threshold, the material will flow
    //if the slope is less than the threshold, the material will not flow
    //the material will flow to the lowest point around it
    //the material will flow to the lowest point around it
    //use Layer struct to get the height of the material
}