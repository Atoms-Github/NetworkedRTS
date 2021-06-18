use crate::ecs::macro_version::macro_mess::MacroMess;
use anymap::AnyMap;
use crate::ecs::macro_version::macro_ecs::*;
use serde::*;
use std::collections::BTreeSet;
use crate::utils::TypeIdNum;
use crate::ecs::unmoving_vec::UnmovingVec;
use mopa::Any;

pub type CompositionID = usize;
pub type GlobalEntityID = usize;
pub type GenerationNum = usize;
pub type InternalIndex = usize;
pub type TypeSet = BTreeSet<TypeIdNum>;



#[derive(Clone, Serialize, Deserialize, Debug, Hash)]
struct InternalEntity {
    global_id: GlobalEntityID,
    composition_id: CompositionID,
    internal_index: InternalIndex,
    generation: GenerationNum,
}
pub struct Entity {
    id: GlobalEntityID,
    generation: GenerationNum,
}

pub struct EntityManager{
    macromess: MacroMess,
    entity_id_lookup: UnmovingVec<InternalEntity>,
}
#[derive(Debug)]
pub struct PendingEntity {
    types: TypeSet,
    components_to_add: AnyMap,
}
impl EntityManager{
    pub fn new() -> Self{
        Self{
            macromess: MacroMess::new(),
            entity_id_lookup: Default::default()
        }
    }
    pub fn get_component<T : 'static>(&self, entity_handle: Entity) -> Option<&T>{
        let internal_entity = self.entity_id_lookup.get(entity_handle.id)?;
        if internal_entity.generation != entity_handle.generation{
            // Entity deleted.
            return None;
        }
        let storage = self.macromess.get_storage::<T>().expect("Component type doesn't exist. This should've been made on startup. Did you forget to add new component type to manually made list?");
        return storage.get_comp(internal_entity.composition_id, internal_entity.internal_index);
    }
    pub fn create_entity(&mut self, new_entity: PendingEntity){

    }
}
#[derive(Serialize, Deserialize, Hash, Default)]
pub struct EStorage<T>{
    items: Vec<Vec<T>>
}
impl<T> EStorage<T>{
    pub fn new() -> Self{
        Self{
            items: Vec::new()
        }
    }
    fn get_comp(&self, composition_id: CompositionID, internal_index: InternalIndex) -> Option<&T>{
        return self.items.get(composition_id).expect("Invalid composition. Assuming valid internal entity, this composition should have been made before entity exists.")
            .get(internal_index);
    }
}