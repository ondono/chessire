#[cfg(test)]
mod test_positions {
    #[test]
    fn start_position() {
        use chessire::board::*;
        use chessire::piece::*;
        let mut b = Board::new();
        b.set_start_position();
        // there's 16 pieces per color in there
        assert_eq!(
            b.get_piece_list()
                .iter()
                .filter(|(i, p)| p.get_color() == Color::Black)
                .count(),
            16
        );
        assert_eq!(
            b.get_piece_list()
                .iter()
                .filter(|(i, p)| p.get_color() == Color::White)
                .count(),
            16
        );
    }
}
