pub trait Collide<Other = Self> {
    fn is_colliding_with(&self, other: &Other) -> bool;
}
