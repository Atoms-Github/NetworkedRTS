use std::collections::{BTreeMap, BTreeSet};
use crate::utils::TypeIdNum;
use serde::*;
use crate::ecs_new_dawn::eid_manager::{GlorifiedHashMap, GlobalEntityID};
use anymap::AnyMap;
use crate::ecs_new_dawn::pending_entity::PendingEntity;

type ByteBlock = Vec<u8>;
type Column = Vec<Vec<ByteBlock>>;

pub type CompositionID = usize;
pub type GenerationNum = usize;
pub type InternalIndex = usize;
pub type TypesHash = u64;
pub type TypeSet = BTreeSet<TypeIdNum>;

#[derive(Clone, Serialize, Default, Deserialize, Debug, Hash, Copy, PartialEq)]
pub struct InternalEntity {
    composition_id: CompositionID,
    internal_index: InternalIndex,
}

#[derive(Default)]
pub struct CompStorage {
    columns: BTreeMap<TypeIdNum, Column>,
    entities: GlorifiedHashMap,
    composition_ids: BTreeMap<TypesHash, CompositionID>,
    next_composition_id: CompositionID
}
pub struct DeleteResult{

}
impl CompStorage{
    pub fn get_comp<T : 'static>(&self, entity_id: GlobalEntityID) -> Option<&T>{
        let internal = self.entities.get(entity_id)?;
        let column = self.get_column::<T>()?;
        let bytes = column.get(internal.composition_id)?.get(internal.internal_index)?;

        let casted : &T = unsafe{u8_slice_to_ref(bytes.as_slice())};
        return Some(casted);
    }
    pub fn delete_entity(&mut self, entity_id: GlobalEntityID) -> Option<DeleteResult>{
        let deleting_internal = self.entities.get(entity_id)?;

        unimplemented!();
        None
    }
    pub fn get_composition_id(&mut self, types_hash: TypesHash) -> CompositionID{
        if let Some(composition_id) = self.composition_ids.get(&types_hash){
            return *composition_id;
        }
        let new_composition_id = self.next_composition_id;
        self.next_composition_id += 1;

        self.composition_ids.insert(types_hash, new_composition_id);

        return new_composition_id;
    }
    pub fn create_entity(&mut self, pending_entity: PendingEntity) -> InternalIndex{
        let composition_id = self.get_composition_id(pending_entity.hash_types());

        let mut internal_index : Option<InternalIndex> = None;
        for (type_id, bytes) in pending_entity.iter(){
            let block = self.get_block_or_make(*type_id, composition_id);
            let new_internal_index = block.len();
            block.push(bytes.clone());

            if let Some(existing_internal_index) = internal_index{
                assert_eq!(existing_internal_index, new_internal_index, "ECS internal indices for new entity were different!");
            }else{
                internal_index = Some(new_internal_index);
            }
        }

        let internal_entity = InternalEntity{
            composition_id,
            internal_index: internal_index.expect("New entity had 0 components! This is disallowed,"),
        };
        let global_entity_id = self.entities.create_entity(internal_entity);
        return global_entity_id;
    }
    fn get_block_or_make(&mut self, type_id: TypeIdNum, composition_id: CompositionID) -> &mut Vec<ByteBlock>{
        let column = self.get_column_mut_or_make_key(type_id);
        for new_block_index in column.len()..(composition_id + 1){
            column.push(vec![]);
        }
        return column.get_mut(composition_id).unwrap();
    }
    fn get_column<T : 'static>(&self) -> Option<&Column>{
        self.columns.get(&crate::utils::get_type_id::<T>())
    }
    fn get_column_mut_or_make<T : 'static>(&mut self) -> &mut Column{
        let key = crate::utils::get_type_id::<T>();
        return self.get_column_mut_or_make_key(key);
    }
    fn get_column_mut_or_make_key(&mut self, type_id: TypeIdNum) -> &mut Column{
        if self.columns.get(&type_id).is_none(){
            self.columns.insert(type_id, vec![]);
        }
        return self.columns.get_mut(&type_id).unwrap();
    }

}
unsafe fn u8_slice_to_ref<T>(bytes: &[u8]) -> &T {
    let bytes_ptr = bytes.as_ptr();
    let test : *const T = unsafe{ std::mem::transmute(bytes_ptr) };
    let value = unsafe {test.as_ref()}.unwrap();
    return value;
}


// unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
//     ::std::slice::from_raw_parts(
//         (p as *const T) as *const u8,
//         ::std::mem::size_of::<T>(),
//     )
// }
