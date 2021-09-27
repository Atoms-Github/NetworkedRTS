use serde::*;
use serde::de::DeserializeOwned;

pub type GridBox = nalgebra::Vector2<i32>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Grid<T>{
    data: Vec<Vec<T>>
}
impl<T: Serialize + DeserializeOwned + Clone + Default> Grid<T>{
    pub fn new(sizex: usize, sizey: usize) -> Self{
        Self{
            data: vec![vec![T::default(); sizey]; sizex]
        }
    }
    pub fn len_x(&self) -> usize{
        return self.data.len();
    }
    pub fn len_y(&self) -> usize{
        return self.data.get(0).unwrap().len();
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
    pub fn iter_square(){

    }
}
pub struct GridIter<'a, T>{
    top_left: GridBox,
    current: GridBox,
    bottom_right: GridBox,
    grid: &'a Grid<T>,

}
impl<'a, T> GridIter<'a, T>{
    pub fn new(grid: &'a Grid<T>, top_left: GridBox, bottom_right: GridBox) -> Self{
        Self{
            top_left: top_left.clone(),
            current: GridBox::new(top_left.x - 1, top_left.y),
            bottom_right,
            grid,
        }
    }
}
impl<'a, T : 'static> Iterator for GridIter<'a, T> {
    type Item = (GridBox, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.current.x += 1;
        if self.current.x > self.bottom_right.x{
            self.current.x = self.top_left.x;
            self.current.y += 1;
        }
        return if self.current.y > self.bottom_right.y {
            None
        } else {
            let value = self.grid.data.get(self.current.x as usize).unwrap().get(self.current.y as usize).unwrap();
            Some((self.current.clone(), value))
        }
    }
}