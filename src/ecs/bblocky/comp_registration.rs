use super::super_any_tests::*;
use crate::ecs::bblocky::*;
use crate::ecs::comp_store::*;
use lazy_static::lazy_static;
use crate::rts::compsys::*;
use crate::bibble::data::data_types::AbilityMould;


use std::any::{TypeId, Any};
use std::collections::HashMap;
use serde::*;

use crate::rts::compsys::*;

use crate::utils::*;
use serde::de::DeserializeOwned;
use crate::ecs::comp_store::*;
use serde::ser::SerializeStruct;
use serde::de::Visitor;
use std::fmt::Write;
use std::fmt;
// use super::my_benchmark::BenchStruct;


lazy_static! {
    pub static ref FUNCTION_MAP: FunctionMap = {
        let mut map = FunctionMap::default();
        map.register_type::<TestStructA>();
        map.register_type::<TestStructB>();
        map.register_type::<TestStructC>();
        map.register_type::<TestComp0>();
        map.register_type::<TestComp1>();
        map.register_type::<ShootMouseComp>();
        map.register_type::<VelocityComp>();
        map.register_type::<VelocityWithInputsComp>();
        map.register_type::<PositionComp>();
        map.register_type::<RadiusComp>();
        map.register_type::<SizeComp>();
        map.register_type::<CollisionComp>();
        map.register_type::<HikerComp>();
        map.register_type::<HikerCollisionComp>();
        map.register_type::<LifeComp>();
        map.register_type::<OrdersComp>();
        map.register_type::<SelectableComp>();
        map.register_type::<CameraComp>();
        map.register_type::<InputComp>();
        map.register_type::<SelectableComp>();
        map.register_type::<SelBoxComp>();
        map.register_type::<OwnedComp>();
        map.register_type::<OwnsResourcesComp>();
        map.register_type::<PlayerComp>();
        map.register_type::<ArenaComp>();
        map.register_type::<AbilitiesComp>();
        map.register_type::<WeaponComp>();
        map.register_type::<WorkerComp>();
        map.register_type::<RenderComp>();
        map.register_type::<TechTreeComp>();
        map.register_type::<SeekingProjComp>();
        map.register_type::<SceneManager>();
        map.register_type::<ScenePersistent>();
        map.register_type::<LobbyManager>();
        map.register_type::<ButtonComp>();
        map.register_type::<RaceButtonComp>();
        map.register_type::<MapButtonComp>();
        map.register_type::<UIComp>();
        map.register_type::<UnitStructureComp>();
        // map.register_type::<BenchStruct>();

        map
    };
}


#[derive(Default)]
pub struct FunctionMap{
    map: HashMap<TypeIdNum, SuperbFunctions>,
}
struct MyData{
    numa: u8,
    numb: u8,
    numc: u8,
}
impl FunctionMap{
    pub fn register_type<T : 'static + Serialize + Clone + DeserializeOwned + Send>(&mut self){
        let size = std::mem::size_of::<T>();
        assert!(size > 0, "Components with size of 0 are disallowed.");
        self.map.insert(gett::<T>(), SuperbFunctions {
            do_clone: |item| {
                let casted = (*item).downcast_ref::<T>().unwrap();
                Box::new(casted.clone())
            },
            ser: |item| {
                let casted = (*item).downcast_ref::<T>().unwrap();
                return bincode::serialize(casted).unwrap();
            },
            deser: |bytes| {
                let item = bincode::deserialize::<T>(bytes).unwrap();
                return Box::new(item);
            },
            meme_ser: |item| {
                let as_type :&T = unsafe{crate::unsafe_utils::u8_slice_to_ref(item)};
                return bincode::serialize(as_type).unwrap();
            },
            meme_deser_and_forget: |serialized_bytes| {
                let item : T = bincode::deserialize::<T>(serialized_bytes).unwrap();
                let my_ref = unsafe{crate::unsafe_utils::struct_as_u8_slice(&item)};
                let to_return = my_ref.to_vec();
                std::mem::forget(item); // TODO: Confirm this.
                return to_return;
            },
            meme_clone_and_forget: |item|{
                let as_type :&T = unsafe{crate::unsafe_utils::u8_slice_to_ref(item)};
                let cloned = as_type.clone();
                let back_to_bytes = unsafe{crate::unsafe_utils::struct_as_u8_slice(&cloned)}.to_vec();
                std::mem::forget(cloned);
                return back_to_bytes;
            },
            item_size: size,
        });
    }
    pub fn get_from_type_id(&self, type_id: TypeId) -> &SuperbFunctions {
        return self.get(crack_type_id(type_id));
    }
    pub fn get(&self, type_id_num: TypeIdNum) -> &SuperbFunctions {
        return self.map.get(&type_id_num).expect("Type wasn't registered!");
    }
}
pub struct SuperbFunctions {
    pub do_clone: fn(&Box<dyn Any + Send>) -> Box<dyn Any + Send>,
    pub ser: fn(&Box<dyn Any + Send>) -> Vec<u8>,
    pub deser: fn(&Vec<u8>) -> Box<dyn Any + Send>,

    pub meme_ser: fn(&[u8]) -> Vec<u8>,
    pub meme_deser_and_forget: fn(&Vec<u8>) -> Vec<u8>,
    pub meme_clone_and_forget: fn(&[u8]) -> Vec<u8>,

    pub item_size: usize,
}