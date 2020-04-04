use crate::systems::health::Health;

pub struct Person {
    name: String,
    health: Health,
}

impl Person {
    pub fn new(name: String) -> Person {
        Person {
            name: name,
            health: Health {
                points: 0,
            },
        }
    }
}
