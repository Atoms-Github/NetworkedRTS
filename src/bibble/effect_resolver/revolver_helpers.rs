pub use crate::bibble::data::data_types::*;
use crate::rts::GameState;
use crate::ecs::GlobalEntityID;
use crate::ecs::comp_store::CompStorage;
use crate::rts::compsys::*;
use crate::bibble::effect_resolver::revolver::Revolver;
use crate::pub_types::PointFloat;
use crate::rts::game::z_values::ZValue;


impl<'a> Revolver<'a>{
    pub fn spawn_seeker_proj(&mut self, data: &GameData, mould: &SeekingProjectileMould, shooter: GlobalEntityID, target: GlobalEntityID){
        let owner = self.c.get_unwrap::<OwnedComp>(shooter).owner;
        let position = self.c.get_unwrap::<PositionComp>(shooter).pos.clone();
        let mut pending = PendingEntity::new3(
            PositionComp{ pos: position },
            OwnedComp { owner },
            SeekingProjComp{
                speed: mould.speed,
                hit_effect: mould.hit_effect.clone(),
                target
            }
        );
        self.add_actor(data, &mould.actor, mould.size, &mut pending);
        self.changes.new_entities.push(pending);
    }
    pub fn spawn_unit(&mut self, data: &GameData, mould: &UnitMould, position: &PointFloat, owner: GlobalEntityID){
        let mut abilities_comp = AbilitiesComp{
            abilities: mould.abilities.iter().map(|ability_id|{AbilityInstance{
                id: *ability_id,
                time_since_use: 0.0
            }}).collect(),
        };

        let mut speed = 0.0;
        if let UnitFlavour::HIKER(hiker_info) = &mould.unit_flavour{
            speed = hiker_info.movespeed;
            abilities_comp.abilities.push(AbilityInstance{
                id: AbilityID::ATTACK_GROUND,
                time_since_use: 0.0
            });
            abilities_comp.abilities.push(AbilityInstance{
                id: AbilityID::WALK,
                time_since_use: 0.0
            });
        }


        let mut pending = PendingEntity::new7(
            PositionComp{ pos: position.clone() },
            OwnedComp { owner },
            LifeComp{ life: mould.life, max_life: mould.life },
            SelectableComp{ is_selected: false },
            OrdersComp{ orders_queue: vec![], state: OrderState::NONE, executing_order_target_loc: PointFloat::new(0.0, 0.0) },
            HikerComp::new(speed),
            WorkerComp{
                resource_gain_per_ms: mould.periodic_gain.clone()
            }
        );
        if let UnitFlavour::HIKER(hiker_info) = &mould.unit_flavour{
            pending.add_comp(HikerCollisionComp{
                radius: mould.radius,
                fly: hiker_info.fly,
            });
        }
        if let UnitFlavour::STRUCTURE(structure) = &mould.unit_flavour{
            pending.add_comp(UnitStructureComp{
                structure_info: structure.clone(),
            });
        }
        pending.add_comp(abilities_comp);
        if mould.weapons.len() > 0{
            pending.add_comp(WeaponComp{
                time_since_shot: 0.0,
                wep_ability_id: *mould.weapons.get(0).unwrap()
            });
        }
        self.add_actor(data, &mould.actor, mould.radius, &mut pending);
        self.changes.new_entities.push(pending);
    }
    pub fn add_actor(&mut self, data: &GameData, mould: &ActorMould, radius: f32, pending: &mut PendingEntity){
        pending.add_comp(RenderComp{
            z: ZValue::GamePiece.g(),
            texture: RenderTexture::Image(mould.image.clone()),
            shape: RenderShape::Rectangle,
            only_render_owner: false
        });
        pending.add_comp(SizeComp{ size: PointFloat::new(radius * 2.0, radius * 2.0) });
    }
}

