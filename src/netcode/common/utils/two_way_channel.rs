use crossbeam_channel::{Sender, Receiver, unbounded, RecvError, SendError};
use crate::netcode::netcode_types::*;
use crate::pub_types::*;
pub fn new_two_way<A,B>() -> (TwoWay<A,B>, TwoWay<B,A>){
    let (a_snk, a_rec) = unbounded();
    let (b_snk, b_rec) = unbounded();
    let way_a = TwoWay{
        sink: a_snk,
        rec: b_rec,
    };
    let way_b = TwoWay{
        sink: b_snk,
        rec: a_rec,
    };
    return (way_a, way_b);
}
pub struct TwoWay<S,R>{
    sink: Sender<S>,
    rec: Receiver<R>,
}

impl<S,R> TwoWay<S,R>{
    pub fn recv(&self) -> Result<R, RecvError>{
        self.rec.recv()
    }
    pub fn send(&mut self, value: S) -> Result<(), SendError<S>>{
        self.sink.send(value)
    }
    pub fn split(self) -> (Sender<S>, Receiver<R>){
        return(self.sink, self.rec);
    }
}














