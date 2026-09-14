/// Monotonic timer contract used by transport keepalive logic.
pub trait Timer {
    fn now_ms(&self) -> u64;
    fn delay_ms(&mut self, milliseconds: u32);
}
