pub mod chess {
    use std::fmt;

    #[derive(Clone, Copy, Debug)]
    enum Color {
        White,
        Black,
    }
    // impl fmt::Display for Color {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         write!(f, "{:?}", self)
    //     }
    // }

    #[derive(Clone, Copy, Debug)]
    enum Piece {
        Pawn,
        Knight,
        Bishop,
        King,
        Queen,
        Rook,
    }
    
    #[derive(Clone, Copy, Debug)]
    struct Square {
        color: Color,
        piece: Piece,
    }

    impl Square {
        fn new(color: Color, piece: Piece) -> Square {
            Square {
                color: color,
                piece: piece,
            }
        }
    }
    impl fmt::Display for Square {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?} {:?}", self.color, self.piece)
        }
    }

    pub struct Board {
        squares: [Option<Square>; 64],
    }

    impl fmt::Display for Board {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut str = "".to_owned();
            for it in self.squares.iter() {
                if it.is_none() {
                    str.push_str(" ");
                } else {
                    str.push_str(&it.unwrap().to_string());
                }
            }
            write!(f, "{}", str)
        }
    }

    impl Board {
        pub fn new() -> Self {
            let mut board: [Option<Square>; 64] = [None; 64];
            board[0] = Some(Square::new(Color::White, Piece::Rook));
            board[7] = Some(Square::new(Color::White, Piece::Rook));
            board[1] = Some(Square::new(Color::White, Piece::Knight));
            board[6] = Some(Square::new(Color::White, Piece::Knight));
            board[2] = Some(Square::new(Color::White, Piece::Bishop));
            board[5] = Some(Square::new(Color::White, Piece::Bishop));
            board[3] = Some(Square::new(Color::White, Piece::Queen));
            board[4] = Some(Square::new(Color::White, Piece::King));
            board[63] = Some(Square::new(Color::Black, Piece::Rook));
            board[56] = Some(Square::new(Color::Black, Piece::Rook));
            board[62] = Some(Square::new(Color::Black, Piece::Knight));
            board[57] = Some(Square::new(Color::Black, Piece::Knight));
            board[61] = Some(Square::new(Color::Black, Piece::Bishop));
            board[58] = Some(Square::new(Color::Black, Piece::Bishop));
            board[59] = Some(Square::new(Color::Black, Piece::Queen));
            board[60] = Some(Square::new(Color::Black, Piece::King));
            for number in 8..16 {
                board[number] = Some(Square::new(Color::Black, Piece::Pawn));
                board[number + 40] = Some(Square::new(Color::Black, Piece::Pawn));
            }
            Board { squares: board }
        }
    }
}
