pub struct Transform {
    pub offset_x: f32,
    pub offset_y: f32,
    pub rotation: f32,
}

impl Transform {
    pub fn new(offset_x: f32, offset_y: f32, rotation: f32) -> Transform {
        Transform {
            offset_x,
            offset_y,
            rotation,
        }
    }
    
    pub fn new_centered(rotation: f32) -> Transform {
        let offset_x = 0.0;
        let offset_y = 0.0;
        
        Transform {
            offset_x,
            offset_y,
            rotation
        }
    }


}