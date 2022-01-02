use crossbeam_channel::Receiver;

pub fn pull_latest<T>(rx: &mut Receiver<T>) -> T{
    // Discards all states in the pipeline until empty, then uses the last one.
    let mut render_state = rx.recv().unwrap();

    let mut next_state_maybe = rx.try_recv();
    while next_state_maybe.is_ok(){
        render_state = next_state_maybe.unwrap();
        next_state_maybe = rx.try_recv();
    }
    render_state
}
