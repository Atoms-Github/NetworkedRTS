use std::collections::BTreeMap;
use crate::utils::TypeIdNum;
use crate::ecs::macro_version::generic_version::*;

type ByteBlock = Vec<u8>;
type Column = Vec<Vec<ByteBlock>>;

#[derive(Default)]
pub struct CompStorage {
    columns: BTreeMap<TypeIdNum, Column>
}
pub struct DeleteResult{

}
impl CompStorage{
    pub fn get_comp<T : 'static>(&self, composition_id: CompositionID, internal_index: InternalIndex) -> Option<&T>{
        let column = self.get_column::<T>()?;
        let bytes = column.get(composition_id)?.get(internal_index)?;

        let casted : &T = unsafe{u8_slice_to_ref(bytes.as_slice())};
        return Some(casted);
    }
    pub fn delete_entity(&mut self, composition_id: CompositionID, internal_index: InternalIndex) -> Option<DeleteResult>{
        None
    }
    pub fn create_component<T : 'static>(&mut self, comp: T, composition_id: CompositionID) -> InternalIndex{
        let column = self.get_column_mut_or_make::<T>();

        return 0;
    }
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
