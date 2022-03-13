use crate::*;
use becs::pending_entity::PendingEntity;
use crate::game_state_smash::SmashPlayerProperty;
use crate::compsys::LifeComp;

pub fn new_bullet(owner: GlobalEntityID, position: PointFloat) -> PendingEntity{
    let e: LifeComp;
    PendingEntity::new6(
        RenderComp{
            z: 100,
            texture: RenderTexture::Color(0.0,0.0,0.0,1.0),
            shape: RenderShape::Circle,
            only_render_owner: false
        },
        PositionComp{ pos: position},
        CollisionComp{ useless: false },
        VelocityComp{ vel: PointFloat::new(1.0,1.0) },
        OwnedComp { owner },
        SizeComp{ size: PointFloat::new(50.0, 50.0) },
    )
}

pub fn new_wasd_pawn(owner: GlobalEntityID, position: PointFloat, shade: Shade) -> PendingEntity{
    PendingEntity::new9(
        RenderComp{
            z: 100,
            texture: RenderTexture::Color(shade.0, shade.1, shade.2,1.0),
            shape: RenderShape::Circle,
            only_render_owner: false
        },
        ShootMouseComp{ time_since_shot: 0.0 },
        SizeComp{ size: PointFloat::new(100.0, 100.0)},
        PositionComp{ pos: position },
        VelocityComp{ vel: PointFloat::new(0.0, 0.0) },
        OwnedComp { owner },
        VelocityWithInputsComp{ speed: 0.15 },
        LifeComp{ life: 100.0, max_life: 100.0 },
        CollisionComp{ useless: false },
    )
}
pub fn new_player(owner: GlobalEntityID) -> PendingEntity{
    PendingEntity::new4(
        PlayerComp{ name: "NamelessWonder".to_string(), connected: false, color: Shade(1.0,0.5,1.0) },
        OwnsResourcesComp::<SmashPlayerProperty>::default(),
        CameraComp{ translation: PointFloat::new(0.0,0.0), zoom: 1.0 },
        InputComp{ is_panning: false, inputs: Default::default(), hovered_entity: None, mouse_pos_game_world: PointFloat::new(0.0, 0.0) },
    )

}
pub fn new_stage() -> PendingEntity{
    PendingEntity::new5(
        RenderComp{
            z: 2,
            texture: RenderTexture::Color(0.7,0.7,0.7,1.0),
            shape: RenderShape::Circle,
            only_render_owner: false
        },
        ShootMouseComp{ time_since_shot: 0.0 },
        SizeComp{ size: PointFloat::new(1000.0, 1000.0)},
        PositionComp{ pos: PointFloat::new(0.0, 0.0) },
        CommonOverseer::default(),
    )
}
