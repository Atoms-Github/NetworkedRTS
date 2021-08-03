use std::collections::HashMap;
use chrono::DateTime;
use crate::netcode::common::time::timekeeping::DT;
use std::time::Duration;

#[derive(Default, Clone)]
pub struct EcsDebugTimer{
    entries: HashMap<String, Entry>,
    running_timers: HashMap<String, DT>,
}


#[derive(Default, Clone)]
struct Entry{
    total: Duration,
    entries: u32
}
impl EcsDebugTimer{
    pub fn new() -> Self{
        Self{
            entries: Default::default(),
            running_timers: Default::default()
        }
    }
    fn get_entry(&mut self, name: String) -> &mut Entry{
        if !self.entries.contains_key(&name){
            self.entries.insert(name.clone(), Entry{
                total: Duration::from_micros(0),
                entries: 0
            });
        }
        return self.entries.get_mut(&name).unwrap();
    }
    pub fn start_timer(&mut self, name: String){
        let timer = DT::start(name.clone().as_str());
        self.running_timers.insert(name, timer);
    }
    pub fn stop_timer(&mut self, name: String){
        let timer = self.running_timers.remove(&name).unwrap();
        let time = timer.stop();
        let mut entry = self.get_entry(name);

        entry.total += time;
        entry.entries += 1;

    }
    pub fn print_all(&self){
        println!(" ---- Times: ---- ");
        for (key, value) in &self.entries{
            let average = value.total / value.entries;
            println!("{} - {:?}", key, average);
        }
    }
}