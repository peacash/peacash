#[derive(Debug)]
pub struct Sync {
    pub index_0: usize,
    pub index_1: usize,
    pub avg: f32,
    pub new: usize,
    pub syncing: bool,
}
impl Sync {
    pub fn handler(&mut self) {
        self.avg += self.new as f32;
        self.avg /= 2_f32;
        self.new = 0;
        self.syncing = self.avg > 1_f32;
    }
}
impl Default for Sync {
    fn default() -> Self {
        Self {
            index_0: 0,
            index_1: 0,
            avg: 0.0,
            new: 0,
            syncing: false,
        }
    }
}
