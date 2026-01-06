
#[derive(Debug, Copy, Clone)]
pub struct MaterialProperties {
    pub density: f32,
    pub flammable: bool,
    pub granular: bool,
    pub liquid: bool,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MaterialId {
    Empty = 0,
    Rock  = 1,
    Sand  = 2,
    Water = 3,
    Oil   = 4,
    Fire  = 5,
}

pub static MATERIALS: [MaterialProperties; 6] = [
    /* Empty */ MaterialProperties { density: 0.0, flammable: false, granular: false, liquid: false },
    /* Rock */ MaterialProperties { density: 2.7, flammable: false, granular: false, liquid: false },
    /* Sand */ MaterialProperties { density: 2.6, flammable: false, granular: true,  liquid: false },
    /* Water */ MaterialProperties { density: 1.0, flammable: false, granular: false, liquid: true  },
    /* Oil */ MaterialProperties { density: 0.8, flammable: true,  granular: false, liquid: true  },
    /* Fire */ MaterialProperties { density: 0.0, flammable: true,  granular: false, liquid: false },
];

// This allows for a person to do materials.props to get the properties of the material instead of having to materials[MaterialId]
impl MaterialId {
    #[inline]
    pub fn props(self) -> &'static MaterialProperties {
        &MATERIALS[self as usize]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
d

impl Layer {

    pub fn new(material: MaterialId, height: usize) -> Self {
        Layer { material_id: material, height: height }
    }
    pub fn empty() -> Self {
        Self::new(MaterialId::Empty, 0)
    }

    pub fn is_empty(&self) -> bool {
        self.height == 0 || self.material_id == MaterialId::Empty
    }
}
/// A single (x,z) column in the heightfield CA.
///
/// - `immoveable_ground_level` is the static terrain height for this column.
/// - `moveable_ground` is the moveable granular/solid layer above terrain.
/// - `moveable_liquid` is the moveable liquid layer above moveable ground.
#[derive(Debug, Copy, Clone)]
pub struct Cell {
    pub immoveable_ground_level: u16,
    pub moveable_ground: Layer,
    pub moveable_liquid: Layer,
    generated_this_tick: bool,
}
impl Default for Cell {
    fn default() -> Self {
        Self {
                immoveable_ground_level: 0,
                moveable_ground: Layer::new(MaterialId::Sand, 1),
                moveable_liquid: Layer::new(MaterialId::Empty, 0),
                generated_this_tick: false
            }
        }
    }
     
impl Cell {
    pub fn new() -> Self {
        Self::default()
    }

    
    
    pub fn total_moveable_height(&self) -> usize {
        self.moveable_ground.height as usize + self.moveable_liquid.height as usize
    }

    pub fn column_height(&self) -> usize {
        self.immoveable_ground_level as usize + self.total_moveable_height()
    }

    //this function assumes that moveable liquid will always be on top of moveable ground
    pub fn surface_material_id(&self) -> MaterialId {
        if self.moveable_liquid.height > 0 && self.moveable_liquid.material_id != MaterialId::Empty {
            self.moveable_liquid.material_id
        } else if self.moveable_ground.height > 0
            && self.moveable_ground.material_id != MaterialId::Empty
        {
            self.moveable_ground.material_id
        } else if self.immoveable_ground_level > 0 {
            MaterialId::Rock
        } else {
            MaterialId::Empty
        }
    }

    pub fn get_amount(&self, material_id: MaterialId) -> usize {
        let mut total = 0usize;
        if self.moveable_ground.material_id == material_id {
            total += self.moveable_ground.height as usize;
        }
        if self.moveable_liquid.material_id == material_id {
            total += self.moveable_liquid.height as usize;
        }
        total
    }

    pub fn set_generated_this_tick(&mut self, value: bool) {
        self.generated_this_tick = value;
    }

    pub fn was_generated_this_tick(&self) -> bool {
        self.generated_this_tick
    }

    pub fn add_layer(&self, Layer) -> Self{

    }

}