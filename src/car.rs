pub struct Car {
    pub is_rule_breaker: bool,
    pub current_road_index: usize,
    pub speed: usize,
    pub id: usize
}

impl Car {
    pub fn new(is_rule_breaker: bool, current_road_index: usize, speed: usize, id: usize) -> Self {
        Car { is_rule_breaker, current_road_index, speed, id }
    }

    pub fn move_forward(&mut self) {
        self.current_road_index += self.speed;
    }

    pub fn has_finished(&self, road_len: usize) -> bool {
        self.current_road_index >= road_len
    }
}