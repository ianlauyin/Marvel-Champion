use crate::features::cards::Identity;

pub struct GamePlayer {
    player_order: usize,
    max_health: u8,
    health: u8,
}

impl GamePlayer {
    pub fn new(player_order: usize, identity: &Identity) -> Self {
        Self {
            player_order,
            max_health: identity.get_health(),
            health: identity.get_health(),
        }
    }

    pub fn is_first_player(&self) -> bool {
        self.player_order == 0
    }

    pub fn damaged(&mut self, damage: u8) {
        if damage >= self.health {
            self.health = 0;
            return;
        }
        self.health -= damage;
    }

    pub fn healed(&mut self, amount: u8) {
        self.health += amount;
    }

    pub fn health_reduced(&mut self, amount: u8) {
        if amount >= self.health {
            self.max_health = 0;
            self.health = 0;
            return;
        }
        self.max_health -= amount;
        self.damaged(amount);
    }

    pub fn health_gained(&mut self, amount: u8) {
        self.max_health += amount;
        self.healed(amount);
    }
}
