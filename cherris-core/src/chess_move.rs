use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use crate::{Color, Error, Position, Role, Square};

/// Represents a move in a chess game.
#[derive(Clone, Copy, PartialEq)]
pub enum Move {
    Standard {
        from: Square,
        to: Square,
        role: Role,
        capture: Option<Role>,
        promotion: Option<Role>,
        en_passant_square: Option<Square>,
    },
    EnPassant {
        from: Square,
        to: Square,
        target: Square,
    },
    CastleShort,
    CastleLong,
}

impl Move {
    pub fn from_lan(lan: &LAN, position: &Position) -> Result<Move, Error> {
        let moves = position.legal_moves();

        for mv in moves {
            match mv {
                Move::Standard {
                    from,
                    to,
                    promotion,
                    ..
                } => {
                    if from == lan.from && to == lan.to && promotion == lan.promotion {
                        return Ok(mv);
                    }
                }
                Move::EnPassant { from, to, .. } => {
                    if from == lan.from && to == lan.to {
                        return Ok(mv);
                    }
                }
                Move::CastleShort => match (lan.from, lan.to, position.color_to_move) {
                    (Square::E1, Square::G1, Color::White) => return Ok(mv),
                    (Square::E8, Square::G8, Color::Black) => return Ok(mv),
                    _ => {}
                },
                Move::CastleLong => match (lan.from, lan.to, position.color_to_move) {
                    (Square::E1, Square::C1, Color::White) => return Ok(mv),
                    (Square::E8, Square::C8, Color::Black) => return Ok(mv),
                    _ => {}
                },
            }
        }

        Err(Error::InvalidMove)
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Standard {
                from,
                to,
                promotion,
                ..
            } => {
                if let Some(promotion) = promotion {
                    write!(f, "{}{}{}", from, to, promotion)
                } else {
                    write!(f, "{}{}", from, to)
                }
            }
            Move::EnPassant { from, to, .. } => {
                write!(f, "{}{}", from, to)
            }
            Move::CastleShort => write!(f, "O-O"),
            Move::CastleLong => write!(f, "O-O-O"),
        }
    }
}

impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Standard {
                from, to, capture, ..
            } => {
                if capture.is_some() {
                    write!(f, "{}x{}", from, to)
                } else {
                    write!(f, "{}-{}", from, to)
                }
            }
            Move::EnPassant { from, to, .. } => {
                write!(f, "{}x{}", from, to)
            }
            Move::CastleShort => write!(f, "O-O"),
            Move::CastleLong => write!(f, "O-O-O"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct LAN {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<Role>,
}

impl FromStr for LAN {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let from = Square::from_str(&s[0..2])?;
        let to = Square::from_str(&s[2..4])?;

        if s.len() == 5 {
            let role = Role::from_str(&s[4..5])?;
            Ok(LAN {
                from,
                to,
                promotion: Some(role),
            })
        } else {
            Ok(LAN {
                from,
                to,
                promotion: None,
            })
        }
    }
}

impl Display for LAN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.from, self.to)?;
        if let Some(role) = self.promotion {
            write!(f, "{}", role)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_from_lan() {
        let lan = LAN::from_str("e2e4").unwrap();
        let position = Position::default();

        let mv = Move::from_lan(&lan, &position).unwrap();
        let expected = Move::Standard {
            from: Square::E2,
            to: Square::E4,
            role: Role::Pawn,
            capture: None,
            promotion: None,
            en_passant_square: Some(Square::E3),
        };

        assert_eq!(mv, expected);
    }

    #[test]
    fn lan_to_string_no_promotion() {
        let lan = LAN {
            from: Square::E2,
            to: Square::E4,
            promotion: None,
        };

        assert_eq!(lan.to_string(), "e2e4");
    }

    #[test]
    fn lan_to_string_promotion() {
        let lan = LAN {
            from: Square::E7,
            to: Square::E8,
            promotion: Some(Role::Queen),
        };

        assert_eq!(lan.to_string(), "e7e8Q");
    }

    #[test]
    fn lan_from_string_no_promotion() {
        let lan = LAN::from_str("e2e4").unwrap();
        let expected = LAN {
            from: Square::E2,
            to: Square::E4,
            promotion: None,
        };

        assert_eq!(lan, expected);
    }
    #[test]
    fn lan_from_string_promotion() {
        let lan = LAN::from_str("e7e8Q").unwrap();
        let expected = LAN {
            from: Square::E7,
            to: Square::E8,
            promotion: Some(Role::Queen),
        };

        assert_eq!(lan, expected);
    }
}
