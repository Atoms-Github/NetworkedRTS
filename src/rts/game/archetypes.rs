use crate::ecs::pending_entity::PendingEntity;
use crate::ecs::GlobalEntityID;
use crate::pub_types::PointFloat;
use crate::rts::compsys::*;
use crate::rts::compsys::owns_resources::{OwnsResourcesComp, RESOURCES_COUNT};

impl PendingEntity{
    pub fn new_bullet(owner: GlobalEntityID, position: PointFloat) -> Self{
        Self::new6(
            RenderComp{ colour: (100,50,50) },
            PositionComp{ pos: position},
            CollisionComp{  },
            VelocityComp{ vel: PointFloat::new(1.0,1.0) },
            OwnedComp { owner },
            SizeComp{ size: PointFloat::new(50.0, 50.0) }
        )
    }
    pub fn new_player(owner: GlobalEntityID) -> Self{
        Self::new4(
            PlayerComp{ name: [0; PLAYER_NAME_SIZE_MAX] },
            OwnsResourcesComp{ resources: [0; RESOURCES_COUNT] },
            CameraComp{ translation: PointFloat::new(0.0,0.0), zoom: 1.0 },
            InputComp{ inputs: Default::default(), mode: InputMode::None, hovered_entity: None, mouse_pos_game_world: PointFloat::new(0.0,0.0) },
        )
    }
    pub fn new_wasd_pawn(owner: GlobalEntityID, position: PointFloat) -> Self{
        Self::new9(
            RenderComp{ colour: (255,255,255)},
            ShootMouseComp{ time_since_shot: 0.0 },
            SizeComp{ size: PointFloat::new(100.0, 100.0)},
            PositionComp{ pos: position },
            VelocityComp{ vel: PointFloat::new(0.0, 0.0) },
            OwnedComp { owner },
            VelocityWithInputsComp{ speed: 2.0 },
            LifeComp{ life: 100.0, max_life: 100.0 },
            CollisionComp{  },
        )
    }
    pub fn new_test_unit(owner: GlobalEntityID, position: PointFloat) -> Self{
        Self::new10(
            RenderComp{ colour: (255,255,255)},
            WorkerComp{ },
            SizeComp{ size: PointFloat::new(30.0, 30.0)},
            PositionComp{ pos: position },
            OwnedComp { owner },
            LifeComp{ life: 100.0, max_life: 100.0 },
            SelectableComp{ is_selected: false },
            OrdersComp{},
            HikerComp{
                destination: None,
                speed: 2.0,
                quest_importance: 0
            },
            HikerCollisionComp{
                radius: 30.0
            }

        )
    }
    pub fn new_sel_box(owner: GlobalEntityID, position: PointFloat) -> Self{
        Self::new5(
            RenderComp{ colour: (255,255,255)},
            SizeComp{ size: PointFloat::new(100.0, 100.0)},
            SelBoxComp{
                starting_pos: position
            },
            PositionComp{ pos: position },
            OwnedComp { owner },
        )
    }
    pub fn new_arena() -> Self{
        Self::new1(
            ArenaComp{
                pathing: [[true; ARENA_SIZE]; ARENA_SIZE]
            }
        )
    }
}