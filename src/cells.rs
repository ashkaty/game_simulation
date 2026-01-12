
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
pub struct Layer {
    pub material_id: MaterialId,
    pub thickness: usize,
}

impl Default for Layer {
    fn default() -> Self {
        Self {
            material_id: MaterialId::Sand,
            thickness: 0,
        }
    }
}

impl Layer {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.thickness == 0 || self.material_id == MaterialId::Empty
    }
}
/// A single (x,z) column in the thicknessfield CA.
///
/// - `immoveable_ground_level` is the static terrain thickness for this column.
/// - `moveable_ground` is the moveable granular/solid layer above terrain.
/// - `moveable_liquid` is the moveable liquid layer above moveable ground.
#[derive(Debug, Clone)]
pub struct Cell {
    pub immoveable_ground_level: usize,
    pub moveable_ground: Vec<Layer>,
    pub moveable_liquid: Layer,
    generated_this_tick: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
                immoveable_ground_level: 0,
                moveable_ground: vec![Layer::default()],
                moveable_liquid: Layer::default(),
                generated_this_tick: false
            }
        }
    }
     
impl Cell {
    pub fn new(_id: usize, _x_cord: usize, _y_cord: usize) -> Self {
        Self::default()
    }

    
    
    pub fn total_moveable_thickness(&self) -> usize {
        let thickness:usize;
        let top_layer_option = self.moveable_ground.last();
        if let Some(top_layer) = top_layer_option {
            thickness = top_layer.thickness
        }
        else {
            thickness = 0
        }
        thickness + self.moveable_liquid.thickness as usize
    }

    pub fn column_thickness(&self) -> usize {
        self.immoveable_ground_level as usize + self.total_moveable_thickness()
    }

    //this function assumes that moveable liquid will always be on top of moveable ground
    pub fn surface_material_id(&self) -> MaterialId {
        if self.moveable_liquid.thickness > 0 && self.moveable_liquid.material_id != MaterialId::Empty {
            self.moveable_liquid.material_id
        } else if let Some(top_layer) = self.moveable_ground.last() {
            if top_layer.thickness > 0 && top_layer.material_id != MaterialId::Empty {
                top_layer.material_id
            } else if self.immoveable_ground_level > 0 {
                MaterialId::Rock
            } else {
                MaterialId::Empty
            }
        } else if self.immoveable_ground_level > 0 {
            MaterialId::Rock
        } else {
            MaterialId::Empty
        }
    }

    pub fn get_amount(&self, material_id: MaterialId) -> usize {
        let mut total = 0usize;
        for layer in &self.moveable_ground {
            if layer.material_id == material_id {
                total += layer.thickness as usize;
            }
        }
        if self.moveable_liquid.material_id == material_id {
            total += self.moveable_liquid.thickness as usize;
        }
        total
    }

    pub fn set_generated_this_tick(&mut self, value: bool) {
        self.generated_this_tick = value;
    }

    pub fn was_generated_this_tick(&self) -> bool {
        self.generated_this_tick
    }

    /// Add material to the cell
    /// If the top layer is the same material, adds thickness to it
    /// Otherwise, creates a new layer on top
    pub fn add_layer(&mut self, material_id: MaterialId, thickness: usize) {
        if thickness == 0 {
            return;
        }

        // Check if top layer is the same material
        if let Some(top_layer) = self.moveable_ground.last_mut() {
            if top_layer.material_id == material_id {
                // Same material - just add thickness
                top_layer.thickness += thickness;
                return;
            }
        }

        // Different material or no layers - create new layer
        let layer = Layer {
            material_id,
            thickness,
        };
        self.moveable_ground.push(layer);
    }

    /// Remove thickness from the top layer(s) of the moveable_ground stack
    /// If a layer's thickness reaches 0, it is popped from the stack
    /// Continues removing from lower layers if needed
    /// Returns the amount of thickness actually removed
    pub fn remove_layer(&mut self, mut thickness: usize) -> usize {
        let initial_thickness = thickness;

        while thickness > 0 && !self.moveable_ground.is_empty() {
            let top_layer = self.moveable_ground.last_mut().unwrap();

            if top_layer.thickness <= thickness {
                // Remove entire layer
                thickness -= top_layer.thickness;
                self.moveable_ground.pop();
            } else {
                // Remove partial thickness from top layer
                top_layer.thickness -= thickness;
                thickness = 0;
            }
        }

        // Return the amount actually removed
        initial_thickness - thickness
    }
}