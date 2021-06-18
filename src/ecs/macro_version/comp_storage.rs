use std::collections::BTreeMap;
use crate::utils::TypeIdNum;

type ByteBlock = Vec<u8>;

pub struct CompStorage {
    columns: BTreeMap<TypeIdNum, Vec<Vec<ByteBlock>>>
}
struct Column{

}