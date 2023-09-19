use crate::modules::consts::TICS_SPAWN_TIMER;

/// Описывает объект, который содержит параметры для спавна новой ноды на поле по истечению таймера
pub struct Spawner {
    upper_limit: u64,
    increase_price : u64,
    ticks_to_spawn: u64,
    reduce_ticks_price : u64,
}

impl Spawner {
    pub fn increase_limit(&mut self) {
        self.upper_limit += 1;
    }

    pub fn get_limit(&self) -> u64 {
        self.upper_limit
    }
}

impl Default for Spawner {
    fn default() -> Self {
        Spawner { upper_limit: 1, increase_price: 11, ticks_to_spawn: TICS_SPAWN_TIMER, reduce_ticks_price : 11  }
    }
}