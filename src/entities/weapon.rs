use crate::systems::attributes::AttributeScore;

pub trait Weapon {
	fn calculate_damage(&self, damage: AttributeScore) -> u32;
}

struct Sword {
	base_damage: u32
}

impl Weapon {
	pub fn new(base_damage: u32) -> Sword {
		Sword {
			base_damage: base_damage
		}
	}
}

impl Weapon for Sword {
	fn calculate_damage(&self, damage: AttributeScore) -> u32 {
		let AttributeScore(damage) = damage;
		self.base_damage * damage
	}
}