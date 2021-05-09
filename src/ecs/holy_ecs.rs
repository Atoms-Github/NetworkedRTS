use std::any::{Any, TypeId};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Debug;
use std::hash::Hash;

use anymap::any::CloneAny;
use anymap::AnyMap;
use serde::{Deserialize, Serialize, Serializer};

use crate::ecs::{Ecs, System, GlobalEntityID};
use crate::ecs::ecs_shared::{SerdeObject, Component};
use crate::ecs::my_anymap::SerdeAnyMap;
use crate::ecs::systems_man::SystemsMan;
use crate::utils::TypeIdNum;
use serde::de::DeserializeOwned;
use crate::ecs::liquid_garbage::TOH;


pub type TypeSet = BTreeSet<TypeIdNum>;
pub type InternalIndex = u16;
pub type GenerationNum = u16;


// #[derive(Serialize, Deserialize, Clone)]
// struct Column<T : Component + 'static>{
//     vec: Vec<Option<T>>,
// }
// impl<T : Component + 'static> SerdeObject for Column<T>{
//     fn my_clone(&self) -> Box<dyn SerdeObject> {
//         Box::new(self.clone())
//     }
//     fn my_ser(&self) -> Vec<u8> {
//         return bincode::serialize(self).unwrap();
//     }
// }

#[derive(Serialize, Deserialize, Clone)]
struct InternalEntity{
    index: InternalIndex,
    generation: GenerationNum,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HolyEcs {
    storages: BTreeMap<TypeIdNum, Vec<TOH>>,
    generations: Vec<GenerationNum>,
    external_entity_lookup: BTreeMap<GlobalEntityID, InternalEntity>,
}
impl HolyEcs {
    pub fn new() -> Self{
        HolyEcs{
            storages: Default::default(),
            generations: vec![],
            external_entity_lookup: Default::default()
        }
    }
    // fn get_storage_mut<T : 'static + Component + DeserializeOwned>(&mut self) -> &mut Column<T>{
    //     if !self.storages.contains_key::<Column<T>>(){
    //         let new_column : Column<T> = Column{ vec: vec![] };
    //         self.storages.insert(new_column);
    //     }
    //     return self.storages.get_mut::<Column<T>>().unwrap();
    // }
}

impl Ecs for HolyEcs {
    fn add_entity(&mut self, new_components: SerdeAnyMap) -> GlobalEntityID {
        let new_entity_index = self.generations.len();

        return new_entity_index;
    }

    fn query(&self, types: Vec<TypeId>) -> Vec<GlobalEntityID> {
        unimplemented!();
    }

    fn get<T: SerdeObject>(&self, entity_id: GlobalEntityID) -> &T {
        unimplemented!()
    }
    fn get_mut<T: SerdeObject>(&mut self, entity_id: GlobalEntityID) -> &mut T {
        unimplemented!()
    }

    fn run_systems(&mut self, systems: &SystemsMan) {
        for system in &systems.systems{
            system.run(self);
        }
    }
}







