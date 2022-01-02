use std::time::{SystemTime, Duration};

#[derive(Clone, Debug)]
pub struct DT{ // Debug Timer.
    time: SystemTime,
    pub name: String
}
impl Default for DT{
    fn default() -> Self {
        Self{
            time: SystemTime::now(),
            name: "NoNameSet".to_string()
        }
    }
}
impl DT{
    pub fn start(name: &str) -> DT{
        DT::start_fmt(String::from(name))
    }
    pub fn start_fmt(name: String) -> DT{
        DT{
            time: SystemTime::now(),
            name
        }
    }

    pub fn stop(self) -> Duration{
        let time_since = SystemTime::now().duration_since(self.time).unwrap();
        return time_since;
    }
    pub fn stop_fmt(self) -> String{
        let time_since = SystemTime::now().duration_since(self.time).unwrap();
        return format!("TIMER {} -> {:?}", self.name, time_since);
    }
    pub fn stop_warn(self, micro_seconds_limit: u128){

        let duration = SystemTime::now().duration_since(self.time).unwrap();
    }
}
