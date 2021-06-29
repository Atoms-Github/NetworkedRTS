use crate::ecs::pending_entity::PendingEntity;
use crate::rts::compsys::*;
use crate::ecs::GlobalEntityID;
use crate::pub_types::PointFloat;

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
        Self::new3(
            PlayerComp{ name: [0; PLAYER_NAME_SIZE_MAX] },
            CameraComp{ translation: PointFloat::new(0.0,0.0), zoom: 1.0 },
            InputComp{ inputs: Default::default(), mode: InputMode::None },
        )
    }
    pub fn new_pawn(owner: GlobalEntityID, position: PointFloat) -> Self{
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
}