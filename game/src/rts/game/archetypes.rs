use crate::ecs::pending_entity::PendingEntity;
use crate::ecs::GlobalEntityID;
use netcode::*;
use crate::pub_types::PointFloat;
use crate::rts::compsys::*;
use crate::rts::compsys::owns_resources::{OwnsResourcesComp, RESOURCES_COUNT};
use crate::bibble::data::data_types::{WeaponID, GameData, RaceID};
use ggez::graphics::Color;
use crate::rts::game::z_values::ZValue;

impl<C> PendingEntity<C>{
    // pub fn new_bullet(owner: GlobalEntityID, position: PointFloat) -> Self{
    //     Self::new7(
    //         RenderComp{ colour: (100,50,50) },
    //         PositionComp{ pos: position},
    //         CollisionComp{ useless: false },
    //         VelocityComp{ vel: PointFloat::new(1.0,1.0) },
    //         OwnedComp { owner },
    //         SizeComp{ size: PointFloat::new(50.0, 50.0) },
    //         ScenePersistent{ keep_alive: false }
    //     )
    // }
    pub fn new_player(owner: GlobalEntityID) -> Self{
        Self::new7(
            PlayerComp{ name: "NamelessWonder".to_string(), alive: true, connected: false, race: RaceID::ROBOTS, color: Shade(1.0,0.5,1.0) },
            OwnsResourcesComp{ resources: [0.0; RESOURCES_COUNT] },
            CameraComp{ translation: PointFloat::new(0.0,0.0), zoom: 1.0 },
            InputComp{ is_panning: false, inputs: Default::default(), mode: InputMode::None, hovered_entity: None, mouse_pos_game_world: PointFloat::new(0.0, 0.0) },
            TechTreeComp{ tree: GameData::gen_game_data() },
            ScenePersistent{ keep_alive: true },
            JigsawPlayerComp{ held_item: None },
        )
    }
    pub fn new_scene_manager() -> Self{
        Self::new2(
            SceneManager{ current: SceneType::None, next: SceneType::Lobby, completed_rounds: 0, connected_players: 0 },
            ScenePersistent{ keep_alive: true },
        )
    }
    pub fn new_lobby(game_start_cooldown: f32) -> Self{
        Self::new1(
            LobbyManager{ game_start_cooldown },
        )
    }
    pub fn new_cursor(player: GlobalEntityID, shade: Shade) -> Self{
        Self::new6(
            CursorComp{
                player,
            },
            IgnoreHoverComp{
                useless: false
            },
            SizeComp{
                size: PointFloat::new(10.0,10.0),
            },
            PositionComp{
                pos: PointFloat::new(0.0,0.0),
            },
            ScenePersistent{
                keep_alive: true
            },
            RenderComp{
                z: ZValue::Cursor.g(),
                texture: RenderTexture::Color(shade.0, shade.1, shade.2, 1.0),
                shape: RenderShape::Rectangle,
                only_render_owner: false
            }

        )
    }
    pub fn new_jigsaw_mat(jigsaw_name: String, size: PointFloat) -> Self{
        Self::new5(
            JigsawMatComp{
                jigsaw_name,
                next_jigsaw_z: ZValue::GamePiece.g() + 1
            },
            IgnoreHoverComp{
                useless: false
            },
            SizeComp{
                size: size.clone(),
            },
            PositionComp {
                pos: PointFloat::new(size.x / 2.0 - JIGSAW_PIECE_SIZE / 2.0, size.y / 2.0 - JIGSAW_PIECE_SIZE / 2.0),
            },
            RenderComp{
                z: ZValue::Arena.g(),
                texture: RenderTexture::Color(0.05,0.05,0.05,0.2),
                shape: RenderShape::Rectangle,
                only_render_owner: false
            }

        )
    }
    pub fn new_jigsaw_piece(image: String, coords: PointInt, position: PointFloat) -> Self{
        Self::new5(
            SizeComp{
                size: PointFloat::new(JIGSAW_PIECE_SIZE, JIGSAW_PIECE_SIZE),
            },
            RenderComp{
                z: ZValue::GamePiece.g(),
                texture: RenderTexture::Jigsaw(image.clone(), coords.clone()),
                shape: RenderShape::Rectangle,
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
    // pub fn new_wasd_pawn(owner: GlobalEntityID, position: PointFloat) -> Self{
    //     Self::new9(
    //         RenderComp{ colour: (255,255,255)},
    //         ShootMouseComp{ time_since_shot: 0.0 },
    //         SizeComp{ size: PointFloat::new(100.0, 100.0)},
    //         PositionComp{ pos: position },
    //         VelocityComp{ vel: PointFloat::new(0.0, 0.0) },
    //         OwnedComp { owner },
    //         VelocityWithInputsComp{ speed: 2.0 },
    //         LifeComp{ life: 100.0, max_life: 100.0 },
    //         CollisionComp{ useless: false },
    //     )
    // }
    pub fn new_sel_box(owner: GlobalEntityID, position: PointFloat) -> Self{
        Self::new5(
            RenderComp{ z: ZValue::SelectionBox.g(), texture: RenderTexture::Color(1.0,1.0,1.0,0.5), shape: RenderShape::Rectangle,
                only_render_owner: false
            },
            SizeComp{ size: PointFloat::new(40.0, 40.0)},
            SelBoxComp{
                starting_pos: position
            },
            PositionComp{ pos: position },
            OwnedComp { owner },
        )
    }
    pub fn new_race_selection_button(race: RaceID, position: PointFloat, image: String) -> Self{
        Self::new6(
            RenderComp{ z: ZValue::UI.g(), texture: RenderTexture::Image(image), shape: RenderShape::Rectangle,
                only_render_owner: false
            },
            SizeComp{ size: PointFloat::new(50.0, 50.0)},
            PositionComp{ pos: position },
            ClickableComp {
                clicking_on: None
            },
            RaceButtonComp{
                race
            },
            UIComp{
                useless: false
            }
        )
    }
    pub fn new_map_selection_button(map: String, position: PointFloat, already_selected: bool, size: f32) -> Self{
        Self::new6(
            RenderComp{ z: ZValue::UI.g(), texture: RenderTexture::Image(map.clone()), shape: RenderShape::Rectangle,
                only_render_owner: false
            },
            SizeComp{ size: PointFloat::new(size, size)},
            PositionComp{ pos: position },
            ClickableComp {
                clicking_on: None
            },
            MapButtonComp{
                selected: already_selected,
                map
            },
            UIComp{
                useless: false
            }
        )
    }
    pub fn new_arena(map_name: String) -> Self{
        let mut arena = ArenaComp::new();
        arena.load_map((format!("maps/{}", map_name)).to_string());
        Self::new1(
            arena,
        )
    }
}