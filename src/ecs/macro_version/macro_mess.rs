use serde::*;
use anymap::AnyMap;
use crate::ecs::macro_version::macro_ecs::*;

pub struct MacroMess {
    anymap: AnyMap,
}
#[derive(Serialize, Deserialize, Hash, Default)]
struct MacroByteables {
    life: Option<EStorage<LifeC>>,
}
impl MacroMess {
    pub fn get_storage<T : 'static>(&self) -> Option<&EStorage<T>>{
        return self.anymap.get::<EStorage<T>>();
    }
    pub fn get_storage_mut<T : 'static>(&mut self) -> Option<&mut EStorage<T>>{
        return self.anymap.get_mut::<EStorage<T>>();
    }

    pub fn new() -> Self{
        let mut map = AnyMap::new();
        // This is fine for now, but impl could be changed to be bytes that make up empty vec, * field count, then call from_bytes. Non macro, maxi - stodge.
        map.insert::<EStorage<LifeC>>(EStorage::default());
        Self{
            anymap: map
        }
    }
    pub fn from_bytes(bytes: &Vec<u8>) -> bincode::Result<Self>{
        let byteables = bincode::deserialize::<MacroByteables>(bytes)?;

        let mut map = AnyMap::new();
        if let Some(comp) = byteables.life{
            map.insert(comp);
        }
        Ok(Self{
            anymap: map,
        })
    }
    pub fn to_bytes(&mut self) -> Vec<u8>{
        let mut bytables = MacroByteables::default();
        // Move from anymap to macro list.
        bytables.life = self.anymap.remove::<EStorage<LifeC>>();
        let bytes = bincode::serialize(&bytables).unwrap();
        // Move back from macro list to anymap.
        if let Some(comp) = bytables.life{
            self.anymap.insert(comp);
        }
        return bytes;
    }
}