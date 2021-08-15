use super::bblocky_tests::*;
use super::bblocky::*;
use crate::ecs::comp_store::*;
use lazy_static::lazy_static;
use crate::rts::compsys::*;
use crate::bibble::data::data_types::AbilityMould;

lazy_static! {
    pub static ref FUNCTION_MAP: FunctionMap = {
        let mut map = FunctionMap::default();
        map.register_type::<TestStructB>();
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


        map
    };
}