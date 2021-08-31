use crate::ecs::comp_store::CompStorage;
use crate::rts::compsys::ArenaComp;
use crate::utils::gett;

pub trait MyCompStorage{
    fn find_arena(&self) -> Option<&mut ArenaComp>;
}
impl MyCompStorage for CompStorage{
    fn find_arena(&self) -> Option<&mut ArenaComp> {
        return self.get_mut(*self.query(vec![gett::<ArenaComp>()]).get(0)?);
    }
}