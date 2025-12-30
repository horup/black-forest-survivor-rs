#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Textures {
    None,
    Tree1,
    Zombie,
    Grass
}

impl Default for Textures {
    fn default() -> Self {
        Textures::None
    }
}
