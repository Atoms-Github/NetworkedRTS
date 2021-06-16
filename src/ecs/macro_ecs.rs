use serde::*;

#[derive(Serialize, Deserialize, PartialEq, Hash, Default)]
pub struct LifeC {

}

pub struct MacroEcs<R>{
    systems: Vec<System<R>>,
    byteables: Byteables
}
impl<R> MacroEcs<R>{
    pub fn new() -> Self{
        Self{
            systems: vec![],
            byteables: Byteables::default()
        }
    }
    pub fn register_system(&mut self, system: System<R>){
        self.systems.push(system);
    }

    pub fn from_bytes(bytes: &Vec<u8>, systems: Vec<System<R>>) -> Option<Self>{
        let bytables = bincode::deserialize::<Byteables>(bytes);
        if bytables.is_err(){
            return None;
        }
        Some(Self{
            systems: vec![],
            byteables: bytables.unwrap(),
        })
    }
    pub fn to_bytes(&self) -> Vec<u8>{
        bincode::serialize(&self.byteables).unwrap()
    }
    pub fn clone(&self) -> Self{
        Self::from_bytes(&self.to_bytes(), self.systems.clone()).unwrap()
    }
}
#[derive(Clone)]
struct System<R>{
    test: fn(R, Byteables),
}
#[derive(Serialize, Deserialize, Hash, Default)]
struct Byteables{
    life: Storage<LifeC>,
}
impl Byteables{

}
#[derive(Serialize, Deserialize, Hash, Default)]
struct Storage<T>{
    items: Vec<Vec<T>>
}