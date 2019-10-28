use super::meta::*;

/// Actions available for the player to take.
#[derive(Debug)]
pub enum Action {
    Idle,
    Walk,
    Dash,
    Jump,
    Attack(Attack),
}

/// Different types of attacks.
#[derive(Debug)]
pub enum Attack {
    DashAttack,
    Shielding,
    Basics(BasicClass, AttackDir),
    Ability(Ability),
}

/// The direction of an attack.
#[derive(Debug)]
pub enum AttackDir {
    Up,
    Down,
    Side,
}