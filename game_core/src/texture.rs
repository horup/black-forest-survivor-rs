#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Texture {
    None,
    Tree1,
    Zombie,
    Grass
}

impl Default for Texture {
    fn default() -> Self {
        Texture::None
    }
}
