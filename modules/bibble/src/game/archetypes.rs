use crate::*;
use ggez::graphics::Color;

impl PendingEntity{

    pub fn new_player(owner: GlobalEntityID) -> Self{
        Self::new7(
            PlayerComp{ name: "NamelessWonder".to_string(), alive: true, connected: false, race: RaceID::ROBOTS, color: Shade(1.0,0.5,1.0) },
            OwnsResourcesComp{ resources: [0.0; RESOURCES_COUNT] },
            CameraComp{ translation: PointFloat::new(0.0,0.0), zoom: 1.0 },
            InputComp{ is_panning: false, inputs: Default::default(), mode: InputMode::None, hovered_entity: None, mouse_pos_game_world: PointFloat::new(0.0, 0.0) },
            TechTreeComp{ tree: GameData::gen_game_data() },
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
