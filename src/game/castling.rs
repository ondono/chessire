use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct CastlingRights {
    pub white_king_side: bool,
    pub white_queen_side: bool,
    pub black_king_side: bool,
    pub black_queen_side: bool,
}

impl Default for CastlingRights {
    fn default() -> CastlingRights {
        Self {
            white_king_side: true,
            white_queen_side: true,
            black_king_side: true,
            black_queen_side: true,
        }
    }
}
impl CastlingRights {
    pub fn new() -> Self {
        Self::default()
    }
}

impl fmt::Display for CastlingRights {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.white_king_side {
            write!(f, "K")?;
        }
        if self.white_queen_side {
            write!(f, "Q")?;
        }
        if self.black_king_side {
            write!(f, "k")?;
        }
        if self.black_queen_side {
            write!(f, "q")?;
        }
        if !self.white_king_side
            && !self.white_queen_side
            && !self.black_king_side
            && !self.black_queen_side
        {
            write!(f, "-")?;
        }
        Ok(())
    }
}
