use bevy::prelude::{FromWorld, Resource};
use rand::prelude::*;
use rand_xoshiro::Xoshiro512StarStar;

pub mod prelude {
    pub use rand::prelude::*;

    pub use super::Random;
}

// TODO: Need to handle seeds
#[derive(Debug, Resource)]
pub struct Random(Xoshiro512StarStar);

impl Random {
    pub fn from_entropy(&self) -> Self {
        Self(Xoshiro512StarStar::from_entropy())
    }

    pub fn roll_3d6(&mut self) -> i32 {
        self.roll_dice(3)
    }

    pub fn roll_die(&mut self) -> i32 {
        self.roll_poly_die(6)
    }

    pub fn roll_dice(&mut self, dice: u32) -> i32 {
        (0..dice).map(|_| self.roll_die()).sum()
    }

    pub fn roll_poly_die(&mut self, sides: i32) -> i32 {
        debug_assert!(sides > 0, "Number of die sides must be positive");
        self.0.gen_range(1..=sides)
    }
}

impl RngCore for Random {
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill_bytes(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.0.try_fill_bytes(dest)
    }
}

impl FromWorld for Random {
    fn from_world(_: &mut bevy::prelude::World) -> Self {
        Self(Xoshiro512StarStar::from_entropy())
    }
}
