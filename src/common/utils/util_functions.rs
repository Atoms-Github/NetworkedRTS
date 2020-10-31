

// I know util functions are sins, but still ....

pub fn vec_replace_or_end<T>(vec: &mut Vec<T>, insertion_index: usize, item: T){
    if vec.is_empty() || vec.len()-1 < insertion_index{
        vec.push(item);
    }else{
        vec.remove(insertion_index);
        vec.insert(insertion_index, item);
    }
}

trait Flatten<T> {
    fn flatten(self) -> Option<T>;
}
impl<T> Flatten<T> for Option<Option<T>> {
    fn flatten(self) -> Option<T> {
        match self {
            None => None,
            Some(v) => v,
        }
    }
}