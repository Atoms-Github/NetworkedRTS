use crate::*;
use ggez::graphics::Color;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Eq, Hash, Copy)]
pub enum JigsawPlayerProperty {
    PiecesLocated,
}
pub fn new_player(owner: GlobalEntityID) -> PendingEntity{
    PendingEntity::new6(
        PlayerComp{ name: "NamelessWonder".to_string(), connected: false, color: Shade(1.0,0.5,1.0) },
        OwnsResourcesComp::<JigsawPlayerProperty>::default(),
        CameraComp{ translation: PointFloat::new(0.0,0.0), zoom: 1.0 },
        InputComp::default(),
        ScenePersistent{ keep_alive: true },
        JigsawPlayerComp{ held_piece: None },
    )
}
pub fn new_scene_manager() -> PendingEntity{
    PendingEntity::new2(
        JigsawSceneManager { current: JigsawSceneType::None, next: JigsawSceneType::Lobby, completed_rounds: 0 },
        ScenePersistent{ keep_alive: true },
    )
}
pub fn new_lobby() -> PendingEntity{
    PendingEntity::new1(
        LobbyManagerComp { chosen_jigsaw: "".to_string() },
    )
}

pub fn new_jigsaw_mat(jigsaw_name: String, size: PointFloat, next_piece_z: ZType) -> PendingEntity{
    PendingEntity::new6(
        JigsawMatComp{
            jigsaw_name,
            next_piece_z
        },
        UninteractableComp {
            useless: false
        },
        SizeComp{
            size: size.clone(),
        },
        PositionComp {
            pos: PointFloat::new(size.x / 2.0 - JIGSAW_PIECE_SIZE / 2.0, size.y / 2.0 - JIGSAW_PIECE_SIZE / 2.0),
        },
        RenderComp{
            z: JZValue::Mat.g(),
            only_render_owner: false
        },
        SimpleViewerComp{
            texture: RenderTexture::Color(0.05,0.05,0.05,0.2),
            shape: RenderShape::Rectangle,
        }

    )
}
pub fn new_jigsaw_piece(image: String, coords: PointInt, position: PointFloat, z: ZType) -> PendingEntity{
    PendingEntity::new5(
        SizeComp{
            size: PointFloat::new(JIGSAW_PIECE_SIZE, JIGSAW_PIECE_SIZE),
        },
        RenderComp{
            z,
            only_render_owner: false
        },
        PositionComp{
            pos: position,
        },
        ClickableComp{
            clicking_on: None
        },
        JigsawPieceComp{
            coords,
            image
        },
    )
}
pub fn new_jigsaw_selection_button(map: String, position: PointFloat, already_selected: bool, size: f32) -> PendingEntity{
    PendingEntity::new7(
        RenderComp{
            z: JZValue::UI.g(),
            only_render_owner: false
        },
        SimpleViewerComp{
            texture: RenderTexture::Image(map.clone()),
            shape: RenderShape::Rectangle,
        },
        SizeComp{ size: PointFloat::new(size, size)},
        PositionComp{ pos: position },
        ClickableComp {
            clicking_on: None
        },
        JigsawButtonComp{
            selected: already_selected,
            map
        },
        UIComp{
            useless: false
        }
    )
}