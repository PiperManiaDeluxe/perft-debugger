- [ ] Write a version of `PerftProvider` and reference generators for different types of pseudo legal moves.
    - Base pseudo legal move settings.
        1. [ ] Kings allowed on board (Just remove otherwise).
            - [ ] Kings can move
        2. [ ] Pawns allowed on board.
            - [ ] Pawns can move
        3. [ ] Knights allowed on board.
            - [ ] Knights can move
        4. [ ] Bishops allowed on board.
            - [ ] Bishops can move
        5. [ ] Rook allowed on board.
            - [ ] Rooks can move
        6. [ ] Queen allowed on board.
            - [ ] Queen can move
    - Legality modifier settings.
        - Default behavior: Every move is considered, everything is attacked, nothing can block.
        - King moves:
            1. [ ] Castling disabled/enabled (Full legality considered apart from check).
            2. [ ] Only opposite color pieces can be attacked by kings.
            3. [ ] Check disabled/enabled (check not considered).
            4. [ ] Castling through or into an attacked square is disabled/enabled.
            5. [ ] Capturing the opposing king is disabled/enabled.
        - Pawn moves:
            1. [ ] Pawns can move forward only into empty squares.
            2. [ ] Pawns can capture only occupied diagonal squares.
            3. [ ] Pawns can double step when on their starting rank.
            4. [ ] Pawn promotion enabled/disabled.
                - [ ] Queen only or any legal promotion piece.
            5. [ ] En passant disabled/enabled. (Only on valid EP moves, Only capture opposing pawns).
            6. [ ] Only opposite color pieces can be attacked by pawns.
        - Knight moves:
            1. [ ] Only opposite color pieces can be attacked by knights.
        - bishop/rook/queen moves:
            1. [ ] Pieces can block sliding moves.
            2. [ ] Only opposite color pieces can be attacked by sliding moves.
