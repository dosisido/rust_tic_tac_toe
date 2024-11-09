#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Z,
    X,
    O,
}

impl std::fmt::Display for Tile{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        let symbol = match self{
            Tile:: Z => ".",
            Tile:: X => "X",
            Tile:: O => "O",
        };
        write!(f, "{}", symbol)?;
        Ok(())
    }
}