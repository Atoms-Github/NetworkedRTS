use crate::*;


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SizeComp{
    pub size: PointFloat,
}



impl SizeComp{
    pub fn set_abs(&mut self, new_size: &PointFloat){
        self.size.x = new_size.x.abs();
        self.size.y = new_size.y.abs();
    }
    pub fn get_corner_top_left(&self, position: &PositionComp) -> PointFloat{
        return PointFloat::new(position.pos.x - self.size.x / 2.0, position.pos.y - self.size.y / 2.0);
    }
    pub fn get_corner_bottom_right(&self, position: &PositionComp) -> PointFloat{
        return PointFloat::new(position.pos.x + self.size.x / 2.0, position.pos.y + self.size.y / 2.0);
    }
    pub fn get_as_rect(&self, position: &PositionComp) -> ggez::graphics::Rect{
        let top_left = self.get_corner_top_left(position);
        let bottom_right = self.get_corner_bottom_right(position);

        ggez::graphics::Rect{
            x: top_left.x,
            y: top_left.y,
            w: bottom_right.x - top_left.x,
            h: bottom_right.y - top_left.y,
        }
    }
}