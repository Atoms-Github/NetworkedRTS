

// I know util functions is a sin, but still ....

pub fn vec_replace_or_end<T>(vec: &mut Vec<T>, insertion_index: usize, item: T){
    if vec.len()-1 < insertion_index{
        vec.push(item);
    }else{
        vec.remove(insertion_index);
        vec.insert(insertion_index, item);
    }
}