use crate::*;
use becs::pending_entity::PendingEntity;

pub fn new_bullet(owner: GlobalEntityID, position: PointFloat) -> PendingEntity{
    PendingEntity::new7(
            RenderComp{ colour: (100,50,50) },
            PositionComp{ pos: position},
            CollisionComp{ useless: false },
            VelocityComp{ vel: PointFloat::new(1.0,1.0) },
            OwnedComp { owner },
            SizeComp{ size: PointFloat::new(50.0, 50.0) },
            ScenePersistent{ keep_alive: false }
        )
    }

    pub fn new_wasd_pawn(owner: GlobalEntityID, position: PointFloat) -> PendingEntity{
        PendingEntity::new9(
            RenderComp{ colour: (255,255,255)},
            ShootMouseComp{ time_since_shot: 0.0 },
            SizeComp{ size: PointFloat::new(100.0, 100.0)},
            PositionComp{ pos: position },
            VelocityComp{ vel: PointFloat::new(0.0, 0.0) },
            OwnedComp { owner },
            VelocityWithInputsComp{ speed: 2.0 },
            LifeComp{ life: 100.0, max_life: 100.0 },
            CollisionComp{ useless: false },
        )
    }
