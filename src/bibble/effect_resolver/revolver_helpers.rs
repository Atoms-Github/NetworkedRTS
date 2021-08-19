pub use crate::bibble::data::data_types::*;
use crate::rts::GameState;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::rts::compsys::*;
use crate::bibble::effect_resolver::revolver::Revolver;
use crate::pub_types::PointFloat;



impl<'a> Revolver<'a>{
    pub fn spawn_unit(&mut self, data: &GameData, mould: &UnitMould, position: &PointFloat, owner: GlobalEntityID){
        let mut abilities_comp = AbilitiesComp{
            abilities: mould.abilities.clone(),
        };
        let mut pending = PendingEntity::new7(
            PositionComp{ pos: position.clone() },
            OwnedComp { owner },
            LifeComp{ life: 100.0, max_life: 100.0 },
            SelectableComp{ is_selected: false },
            OrdersComp{ orders_queue: vec![], state: OrderState::NONE },
            HikerComp{
                destination: None,
                speed: 2.0,
                quest_importance: 0
            },
            HikerCollisionComp{
                radius: mould.radius
            }
        );
        pending.add_comp(abilities_comp);
        if mould.weapons.len() > 0{
            pending.add_comp(WeaponComp{
                weapon_id: *mould.weapons.get(0).unwrap(),
                time_since_shot: 0.0
            });
        }
        self.add_actor(data, &mould.actor, &mut pending);
        self.changes.new_entities.push(pending);
    }
    pub fn add_actor(&mut self, data: &GameData, mould: &ActorMould, pending: &mut PendingEntity){
        pending.add_comp(RenderComp{ colour: mould.colour});
        pending.add_comp(SizeComp{ size: PointFloat::new(30.0, 30.0)});
    }
}

