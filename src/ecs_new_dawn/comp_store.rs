use std::collections::{BTreeMap, BTreeSet};
use crate::utils::TypeIdNum;
use serde::*;
use crate::ecs_new_dawn::eid_manager::GlorifiedHashMap;
use anymap::AnyMap;

type ByteBlock = Vec<u8>;
type Column = Vec<Vec<ByteBlock>>;

pub type CompositionID = usize;
pub type GlobalEntityID = usize;
pub type GenerationNum = usize;
pub type InternalIndex = usize;
pub type TypeSet = BTreeSet<TypeIdNum>;

#[derive(Clone, Serialize, Default, Deserialize, Debug, Hash, Copy, PartialEq)]
pub struct InternalEntity {
    global_id: GlobalEntityID,
    composition_id: CompositionID,
    internal_index: InternalIndex,
}

#[derive(Default)]
pub struct CompStorage {
    columns: BTreeMap<TypeIdNum, Column>,
    entities: GlorifiedHashMap,
}
pub struct DeleteResult{

}
#[derive(Debug)]
pub struct PendingEntity {
    types: TypeSet,
    components_to_add: AnyMap,
}
impl CompStorage{
    pub fn get_comp<T : 'static>(&self, entity_id: GlobalEntityID) -> Option<&T>{
        let internal = self.entities.get(entity_id)?;
        let column = self.get_column::<T>()?;
        let bytes = column.get(internal.composition_id)?.get(internal.internal_index)?;

        let casted : &T = unsafe{u8_slice_to_ref(bytes.as_slice())};
        return Some(casted);
    }
    pub fn delete_entity(&mut self, composition_id: CompositionID, internal_index: InternalIndex) -> Option<DeleteResult>{
        None
    }
    pub fn create_entity(&mut self, pending_entity: PendingEntity) -> InternalIndex{
        return 0;
    }
    // fn get_block_mut_or_make<T : 'static>(&mut self) -> &mut Column{
    //     ;
    // }
    fn get_column<T : 'static>(&self) -> Option<&Column>{
        self.columns.get(&crate::utils::get_type_id::<T>())
    }
    fn get_column_mut_or_make<T : 'static>(&mut self) -> &mut Column{
        let key = crate::utils::get_type_id::<T>();
        if self.columns.get(&key).is_none(){
            self.columns.insert(key, vec![]);
        }
        return self.columns.get_mut(&key).unwrap();
    }

}
unsafe fn u8_slice_to_ref<T>(bytes: &[u8]) -> &T {
    let bytes_ptr = bytes.as_ptr();
    let test : *const T = unsafe{ std::mem::transmute(bytes_ptr) };
    let value = unsafe {test.as_ref()}.unwrap();
    return value;
}
unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::std::mem::size_of::<T>(),
    )
}

// unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
//     ::std::slice::from_raw_parts(
//         (p as *const T) as *const u8,
//         ::std::mem::size_of::<T>(),
//     )
// }
