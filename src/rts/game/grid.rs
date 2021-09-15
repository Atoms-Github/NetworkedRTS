use serde::*;
use serde::de::DeserializeOwned;

pub type GridBox = nalgebra::Vector2<i32>;

pub struct Grid<T>{
    data: Vec<Vec<T>>
}
impl<T: Serialize + DeserializeOwned + Clone + Default> Grid<T>{
    pub fn new(sizex: usize, sizey: usize) -> Self{
        Self{
            data: vec![vec![T::default(); sizey]; sizex]
        }
    }
    pub fn is_valid(&self, target: &GridBox) -> bool{
        return target.x >= 0 && target.x < self.data.len() as i32
            && target.y >= 0 && target.y < self.data.get(0).unwrap().len() as i32;
    }
    pub fn get_unwrap(&self, target: &GridBox) -> &T{
        return self.data.get(target.x as usize).unwrap().get(target.y as usize).unwrap();
    }
    pub fn set(&mut self, target: &GridBox, value: T){
        if self.is_valid(target){
            *self.data.get_mut(target.x as usize).unwrap().get_mut(target.y as usize).unwrap() = value;
        }
    }
}