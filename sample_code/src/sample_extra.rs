// #![allow(dead_code)]

use std::ops::Add;

/// Very dangerous creature. Have caution
#[derive(Debug)]
pub struct Enemy {
    name: &'static str,
    pub x: f32,
    pub y: f32,
    id: usize,
}

pub enum FightOptions {
    Attack(Enemy),
    UseItem,
    Run,
}

#[repr(transparent)]
struct Wrapper<T> {
    value: T,
}

impl Add<f32> for Wrapper<f32> {
    type Output = Wrapper<f32>;
    fn add(self, rhs: f32) -> Self::Output {
        // A comment about how you should implement this
        todo!(
            "not yet implemented :(. But anyway, check out this number I found {}",
            rhs
        )
    }
}

#[cfg(test)]
mod test {

}
