use ggez::graphics::Color;
use netcode::Shade;
use crate::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PlotFlooring {
    WALL,
    PATH,
    GREEN_RESOURCE,
    STRUCTURE,
}
impl PlotFlooring{
    pub fn get_color(&self) -> Shade{
        match self {
            PlotFlooring::WALL => {
                Shade(0.2, 0.2, 0.2)
            }
            PlotFlooring::PATH => {
                Shade(0.2, 0.5, 0.2)
            }
            PlotFlooring::GREEN_RESOURCE => {
                Shade(0.2, 0.8, 0.2)
            }
            PlotFlooring::STRUCTURE => {
                Shade(0.5,0.2,0.2)
            }
        }
    }
    pub fn can_walk_over(&self) -> bool{
        return !(*self == PlotFlooring::WALL || *self == PlotFlooring::STRUCTURE);
    }
    pub fn from_color(color: Color) -> Self{
        let (r,g,b) = color.to_rgb();
        match (r,g,b){
            (255,255,255) => {Self::PATH}
            (0,255,0) => {Self::GREEN_RESOURCE}
            (0,0,0) => {Self::WALL}
            (_, _, _) => {Self::PATH}
        }
    }
}
impl Default for PlotFlooring {
    fn default() -> Self {
        return Self::PATH;
    }
}