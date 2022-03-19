use crate::*;

pub trait MyCompStorage{
    fn find_arena(&self) -> Option<&mut ArenaComp>;
    fn find_scene(&self) -> Option<&mut SceneManager>;
}
impl MyCompStorage for CompStorage{
    fn find_arena(&self) -> Option<&mut ArenaComp> {
        return self.get_mut(*self.query(vec![gett::<ArenaComp>()]).get(0)?);
    }
    fn find_scene(&self) -> &mut SceneManager {
        return self.get_mut(*self.query(vec![gett::<SceneManager>()]).get(0)?).unwrap();
    }
}
