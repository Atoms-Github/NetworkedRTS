use crate::*;
use serde::*;
use serde::de::DeserializeOwned;

use netcode::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Grid<T>{
    data: Vec<Vec<T>>
}
impl<T: Clone + Default> Grid<T>{
    pub fn resize_to_fit(&mut self, bottom_right: &GridBox){
        self.data.resize(bottom_right.x as usize + 1, vec![]);
        for column in &mut self.data{
            column.resize(bottom_right.y as usize + 1, T::default());
        }
    }
    pub fn new(sizex: usize, sizey: usize) -> Self{
        Self{
            data: vec![vec![T::default(); sizey]; sizex]
        }
    }
}
impl<T> Grid<T>{
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
    pub fn get(&self, target: &GridBox) -> Option<&T>{
        // Could be simplified?
        if let Some(item) = self.data.get(target.x as usize){
            return item.get(target.y as usize);
        }
        return None;
    }
    pub fn get_mut(&mut self, target: &GridBox) -> Option<&mut T>{
        // Could be simplified?
        if let Some(column) = self.data.get_mut(target.x as usize){
            return column.get_mut(target.y as usize);
        }
        return None;
    }
    pub fn set(&mut self, target: &GridBox, value: T){
        if self.is_valid(target){
            *self.data.get_mut(target.x as usize).unwrap().get_mut(target.y as usize).unwrap() = value;
        }
    }
    pub fn iter_square(&self, top_left: GridBox, bottom_right: GridBox) -> GridIter<T>{
        return GridIter::new(self, top_left, bottom_right);
    }
    pub fn iter_all(&self) -> GridIter<T>{
        self.iter_square(GridBox::new(0,0), GridBox::new(self.len_x() as i32 - 1, self.len_y() as i32 - 1))
    }

    pub fn raw(&self) -> &Vec<Vec<T>>{
        return &self.data;
    }
    pub fn raw_mut(&mut self) -> &mut Vec<Vec<T>>{
        return &mut self.data;
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
            if self.grid.is_valid(&self.current){
                let value = self.grid.data.get(self.current.x as usize).unwrap().get(self.current.y as usize).unwrap();
                Some((self.current.clone(), value))
            }else{
                self.next()
            }
        }
    }
}



