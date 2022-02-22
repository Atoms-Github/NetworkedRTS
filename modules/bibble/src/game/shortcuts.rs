use crate::*;
use crate::ecs::comp_store::CompStorage;
use crate::utils::gett;

pub trait MyCompStorage{
    fn find_arena(&self) -> Option<&mut ArenaComp>;
    fn find_jigsaw_mat(&self) -> Option<&mut JigsawMatComp>;
    fn find_scene(&self) -> &mut SceneManager;
}
impl MyCompStorage for CompStorage{
    fn find_arena(&self) -> Option<&mut ArenaComp> {
        return self.get_mut(*self.query(vec![gett::<ArenaComp>()]).get(0)?);
    }
    fn find_jigsaw_mat(&self) -> Option<&mut JigsawMatComp> {
        return self.get_mut(*self.query(vec![gett::<JigsawMatComp>()]).get(0)?);
    }
    fn find_scene(&self) -> &mut SceneManager {
        return self.get_mut(SCENE_MAN_ENT_ID).unwrap();
    }
}
