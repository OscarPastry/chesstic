use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam, Mesh, MeshBuilder, Text};
use ggez::{Context, ContextBuilder, GameResult};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum PieceColor {
    White,
    Black,
}
impl PieceColor {
    fn opposite(self) -> Self {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
impl PieceType {
    fn value(self) -> i32 {
        match self {
            PieceType::Pawn => 100,
            PieceType::Knight => 300,
            PieceType::Bishop => 300,
            PieceType::Rook => 500,
            PieceType::Queen => 900,
            PieceType::King => 20000,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Piece {
    color: PieceColor,
    kind: PieceType,
}
type Board = [[Option<Piece>; 8]; 8];

const PAWN_TABLE: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [50, 50, 50, 50, 50, 50, 50, 50],
    [10, 10, 20, 30, 30, 20, 10, 10],
    [5, 5, 10, 25, 25, 10, 5, 5],
    [0, 0, 0, 20, 20, 0, 0, 0],
    [5, -5, -10, 0, 0, -10, -5, 5],
    [5, 10, 10, -20, -20, 10, 10, 5],
    [0, 0, 0, 0, 0, 0, 0, 0],
];

const KNIGHT_TABLE: [[i32; 8]; 8] = [
    [-50, -40, -30, -30, -30, -30, -40, -50],
    [-40, -20, 0, 0, 0, 0, -20, -40],
    [-30, 0, 10, 15, 15, 10, 0, -30],
    [-30, 5, 15, 20, 20, 15, 5, -30],
    [-30, 0, 15, 20, 20, 15, 0, -30],
    [-30, 5, 10, 15, 15, 10, 5, -30],
    [-40, -20, 0, 5, 5, 0, -20, -40],
    [-50, -40, -30, -30, -30, -30, -40, -50],
];

const BISHOP_TABLE: [[i32; 8]; 8] = [
    [-20, -10, -10, -10, -10, -10, -10, -20],
    [-10, 0, 0, 0, 0, 0, 0, -10],
    [-10, 0, 5, 10, 10, 5, 0, -10],
    [-10, 5, 5, 10, 10, 5, 5, -10],
    [-10, 0, 10, 10, 10, 10, 0, -10],
    [-10, 10, 10, 10, 10, 10, 10, -10],
    [-10, 5, 0, 0, 0, 0, 5, -10],
    [-20, -10, -10, -10, -10, -10, -10, -20],
];

const ROOK_TABLE: [[i32; 8]; 8] = [
    [0, 0, 0, 5, 5, 0, 0, 0],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [5, 10, 10, 10, 10, 10, 10, 5],
    [0, 0, 0, 5, 5, 0, 0, 0],
];

const QUEEN_TABLE: [[i32; 8]; 8] = [
    [-20, -10, -10, -5, -5, -10, -10, -20],
    [-10, 0, 0, 0, 0, 0, 0, -10],
    [-10, 0, 5, 5, 5, 5, 0, -10],
    [-5, 0, 5, 5, 5, 5, 0, -5],
    [0, 0, 5, 5, 5, 5, 0, -5],
    [-10, 0, 5, 5, 5, 5, 0, -10],
    [-10, 0, 0, 0, 0, 0, 0, -10],
    [-20, -10, -10, -5, -5, -10, -10, -20],
];

const KING_TABLE: [[i32; 8]; 8] = [
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-20, -30, -30, -40, -40, -30, -30, -20],
    [-10, -20, -20, -20, -20, -20, -20, -10],
    [20, 20, 0, 0, 0, 0, 20, 20],
    [20, 30, 10, 0, 0, 10, 30, 20],
];

fn piece_square_value(kind: PieceType, color: PieceColor, row: usize, col: usize) -> i32 {
    let r = if color == PieceColor::White {
        row
    } else {
        7 - row
    };
    match kind {
        PieceType::Pawn => PAWN_TABLE[r][col],
        PieceType::Knight => KNIGHT_TABLE[r][col],
        PieceType::Bishop => BISHOP_TABLE[r][col],
        PieceType::Rook => ROOK_TABLE[r][col],
        PieceType::Queen => QUEEN_TABLE[r][col],
        PieceType::King => KING_TABLE[r][col],
    }
}
fn inital_board() -> Board {
    use PieceColor::*;
    use PieceType::*;

    let back_row = [Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook];
    let mut board: Board = [[None; 8]; 8];
    for i in 0..8 {
        board[0][i] = Some(Piece {
            color: Black,
            kind: back_row[i],
        });
        board[1][i] = Some(Piece {
            color: Black,
            kind: Pawn,
        });
        board[6][i] = Some(Piece {
            color: White,
            kind: Pawn,
        });
        board[7][i] = Some(Piece {
            color: White,
            kind: back_row[i],
        });
    }
    board
}

fn slide(
    board: &Board,
    row: usize,
    col: usize,
    color: PieceColor,
    dir: &[(i32, i32)],
    moves: &mut Vec<(usize, usize)>,
) {
    for (dr, dc) in dir {
        let mut r = row as i32 + dr;
        let mut c = col as i32 + dc;
        while r >= 0 && r < 8 && c >= 0 && c < 8 {
            match board[r as usize][c as usize] {
                Some(p) if p.color == color => break, // Blocked by own piece
                Some(_) => {
                    moves.push((r as usize, c as usize)); // Capture opponent piece
                    break;
                }
                None => moves.push((r as usize, c as usize)), // Empty square
            }
            r += dr;
            c += dc;
        }
    }
}

fn get_pseudo_moves(
    board: &Board,
    row: usize,
    col: usize,
    en_passant_target: Option<(usize, usize)>,
    castling_rights: CastlingRights,
    moves: &mut Vec<(usize, usize)>,
) {
    let piece = match board[row][col] {
        Some(p) => p,
        None => return,
    };
    let color = piece.color;

    // nr = next row, nc = next col
    match piece.kind {
        PieceType::Pawn => {
            let dir: i32 = if color == PieceColor::White { -1 } else { 1 };
            let startrow = if color == PieceColor::White { 6 } else { 1 };
            // Forward move
            let forward_row = row as i32 + dir;
            if forward_row >= 0 && forward_row < 8 {
                let nr = forward_row as usize; // Check forward move
                if board[nr][col].is_none() {
                    moves.push((nr, col));
                    if row == startrow {
                        // Check double move from starting position
                        let nr2 = (row as i32 + dir * 2) as usize;
                        if board[nr2][col].is_none() {
                            moves.push((nr2, col));
                        }
                    }
                }
                for dc in [-1i32, 1] {
                    let nc = col as i32 + dc; // Check captures
                    if nc >= 0 && nc < 8 {
                        if let Some(t) = board[nr][nc as usize] {
                            if t.color != color {
                                moves.push((nr, nc as usize));
                            }
                        }
                        if en_passant_target == Some((nr, nc as usize)) {
                            moves.push((nr, nc as usize)); // En passant capture
                        }
                    }
                }
            }
        }
        PieceType::Knight => {
            for (dr, dc) in [
                (-2, -1i32),
                (-2, 1),
                (-1, 2),
                (-1, -2),
                (1, 2),
                (1, -2),
                (2, 1),
                (2, -1),
            ] {
                let (r, c) = (row as i32 + dr, col as i32 + dc);
                if r >= 0 && r < 8 && c >= 0 && c < 8 {
                    if board[r as usize][c as usize].map_or(true, |p| p.color != color) {
                        moves.push((r as usize, c as usize));
                    }
                }
            }
        }
        PieceType::Bishop => {
            slide(
                board,
                row,
                col,
                color,
                &[(-1, -1), (-1, 1), (1, -1), (1, 1)],
                moves,
            );
        }
        PieceType::Rook => {
            slide(
                board,
                row,
                col,
                color,
                &[(-1, 0), (1, 0), (0, -1), (0, 1)],
                moves,
            );
        }
        PieceType::Queen => {
            slide(
                board,
                row,
                col,
                color,
                &[
                    (-1, -1),
                    (-1, 1),
                    (1, -1),
                    (1, 1),
                    (-1, 0),
                    (1, 0),
                    (0, -1),
                    (0, 1),
                ],
                moves,
            );
        }
        PieceType::King => {
            for dr in -1i32..=1 {
                for dc in -1i32..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                    let (r, c) = (row as i32 + dr, col as i32 + dc);
                    if r >= 0 && r < 8 && c >= 0 && c < 8 {
                        if board[r as usize][c as usize].map_or(true, |p| p.color != color) {
                            moves.push((r as usize, c as usize));
                        }
                    }
                }
            }
            let enemy_color = color.opposite();
            let king_row = if color == PieceColor::White { 7 } else { 0 };
            let can_kingside = (color == PieceColor::White && castling_rights.white_kingside)
                || (color == PieceColor::Black && castling_rights.black_kingside);
            let can_queenside = (color == PieceColor::White && castling_rights.white_queenside)
                || (color == PieceColor::Black && castling_rights.black_queenside);

            // King must be on its original square and not currently in check
            if (can_kingside || can_queenside)
                && row == king_row
                && col == 4
                && !is_attacked(board, row, col, enemy_color)
            {
                // Kingside castling
                if can_kingside {
                    if board[king_row][5].is_none()
                        && board[king_row][6].is_none()
                        && !is_attacked(board, king_row, 5, enemy_color)
                        && !is_attacked(board, king_row, 6, enemy_color)
                    {
                        moves.push((king_row, 6));
                    }
                }
                // Queenside castling
                if can_queenside {
                    if board[king_row][3].is_none()
                        && board[king_row][2].is_none()
                        && board[king_row][1].is_none()
                        && !is_attacked(board, king_row, 3, enemy_color)
                        && !is_attacked(board, king_row, 2, enemy_color)
                    {
                        moves.push((king_row, 2));
                    }
                }
            }
        }
    }
}
fn is_attacked(board: &Board, row: usize, col: usize, by_color: PieceColor) -> bool {
    // Check for pawns
    let pawn_dir = if by_color == PieceColor::White { 1 } else { -1 };
    let pawn_row = row as i32 + pawn_dir;
    if pawn_row >= 0 && pawn_row < 8 {
        for dc in [-1i32, 1] {
            let nc = col as i32 + dc;
            if nc >= 0 && nc < 8 {
                if let Some(p) = board[pawn_row as usize][nc as usize] {
                    if p.color == by_color && p.kind == PieceType::Pawn {
                        return true;
                    }
                }
            }
        }
    }

    // Check for knights
    for (dr, dc) in [
        (-2, -1i32),
        (-2, 1),
        (-1, 2),
        (-1, -2),
        (1, 2),
        (1, -2),
        (2, 1),
        (2, -1),
    ] {
        let r = row as i32 + dr;
        let c = col as i32 + dc;
        if r >= 0 && r < 8 && c >= 0 && c < 8 {
            if let Some(p) = board[r as usize][c as usize] {
                if p.color == by_color && p.kind == PieceType::Knight {
                    return true;
                }
            }
        }
    }

    // Check for kings
    for dr in -1i32..=1 {
        for dc in -1i32..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let r = row as i32 + dr;
            let c = col as i32 + dc;
            if r >= 0 && r < 8 && c >= 0 && c < 8 {
                if let Some(p) = board[r as usize][c as usize] {
                    if p.color == by_color && p.kind == PieceType::King {
                        return true;
                    }
                }
            }
        }
    }

    // Check for sliding pieces
    for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let mut r = row as i32 + dr;
        let mut c = col as i32 + dc;
        while r >= 0 && r < 8 && c >= 0 && c < 8 {
            if let Some(p) = board[r as usize][c as usize] {
                if p.color == by_color && (p.kind == PieceType::Rook || p.kind == PieceType::Queen)
                {
                    return true;
                }
                break;
            }
            r += dr;
            c += dc;
        }
    }

    for (dr, dc) in [(-1, -1), (-1, 1), (1, -1), (1, 1)] {
        let mut r = row as i32 + dr;
        let mut c = col as i32 + dc;
        while r >= 0 && r < 8 && c >= 0 && c < 8 {
            if let Some(p) = board[r as usize][c as usize] {
                if p.color == by_color
                    && (p.kind == PieceType::Bishop || p.kind == PieceType::Queen)
                {
                    return true;
                }
                break;
            }
            r += dr;
            c += dc;
        }
    }

    false
}
fn is_in_check(board: &Board, color: PieceColor) -> bool {
    let mut king = (0usize, 0usize);
    //find the king
    'find: for r in 0..8 {
        for c in 0..8 {
            if let Some(p) = board[r][c] {
                if p.color == color && p.kind == PieceType::King {
                    king = (r, c);
                    break 'find;
                }
            }
        }
    }
    //check if any opponent piece can move to the king's position
    is_attacked(board, king.0, king.1, color.opposite())
}
fn get_legal_moves(
    board: &Board,
    row: usize,
    col: usize,
    en_passant_target: Option<(usize, usize)>,
    castling_rights: CastlingRights,
    legal_moves: &mut Vec<(usize, usize)>,
) {
    let color = match board[row][col] {
        Some(p) => p.color,
        None => return,
    };

    let is_king = board[row][col].map_or(false, |p| p.kind == PieceType::King);

    // Find the king once if we're not moving it
    let mut king_pos = (0, 0);
    if !is_king {
        'find: for r in 0..8 {
            for c in 0..8 {
                if let Some(p) = board[r][c] {
                    if p.color == color && p.kind == PieceType::King {
                        king_pos = (r, c);
                        break 'find;
                    }
                }
            }
        }
    }

    let mut pseudo_moves = Vec::with_capacity(32);
    get_pseudo_moves(
        board,
        row,
        col,
        en_passant_target,
        castling_rights,
        &mut pseudo_moves,
    );

    for (nr, nc) in pseudo_moves {
        let mut temp_board = *board;
        temp_board[nr][nc] = temp_board[row][col];
        temp_board[row][col] = None;

        // If en passant capture, remove the captured pawn
        if let Some(piece) = temp_board[nr][nc] {
            if piece.kind == PieceType::Pawn && nc != col && board[nr][nc].is_none() {
                temp_board[row][nc] = None;
            }
        }

        let current_king_pos = if is_king { (nr, nc) } else { king_pos };

        // Keep the moves only if King is not in check after it
        if !is_attacked(
            &temp_board,
            current_king_pos.0,
            current_king_pos.1,
            color.opposite(),
        ) {
            legal_moves.push((nr, nc));
        }
    }
}

fn is_insufficient_material(board: &Board) -> bool {
    let mut white_pieces = 0;
    let mut black_pieces = 0;
    let mut white_knights = 0;
    let mut black_knights = 0;
    let mut white_bishops = 0;
    let mut black_bishops = 0;

    for r in 0..8 {
        for c in 0..8 {
            if let Some(p) = board[r][c] {
                match p.color {
                    PieceColor::White => {
                        white_pieces += 1;
                        match p.kind {
                            PieceType::Knight => white_knights += 1,
                            PieceType::Bishop => white_bishops += 1,
                            PieceType::Pawn | PieceType::Rook | PieceType::Queen => return false,
                            PieceType::King => {}
                        }
                    }
                    PieceColor::Black => {
                        black_pieces += 1;
                        match p.kind {
                            PieceType::Knight => black_knights += 1,
                            PieceType::Bishop => black_bishops += 1,
                            PieceType::Pawn | PieceType::Rook | PieceType::Queen => return false,
                            PieceType::King => {}
                        }
                    }
                }
            }
        }
    }

    // K vs K
    if white_pieces == 1 && black_pieces == 1 {
        return true;
    }

    // K + N vs K or K + B vs K
    if white_pieces == 2 && black_pieces == 1 {
        if white_knights == 1 || white_bishops == 1 {
            return true;
        }
    }
    if white_pieces == 1 && black_pieces == 2 {
        if black_knights == 1 || black_bishops == 1 {
            return true;
        }
    }

    // K + B vs K + B (Any colors for simplicity FIDE rules allow mate if on opposite colors, but many digital rules just say KB v KB is draw)
    if white_pieces == 2 && black_pieces == 2 && white_bishops == 1 && black_bishops == 1 {
        return true;
    }

    false
}

fn has_legal_moves(
    board: &Board,
    color: PieceColor,
    en_passant_target: Option<(usize, usize)>,
    castling_rights: CastlingRights,
) -> bool {
    let mut moves = Vec::new();
    for r in 0..8 {
        for c in 0..8 {
            if let Some(p) = board[r][c] {
                if p.color == color {
                    get_legal_moves(board, r, c, en_passant_target, castling_rights, &mut moves);
                    if !moves.is_empty() {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn load_svg_as_image(ctx: &mut Context, path: &str, size: u32) -> GameResult<graphics::Image> {
    let svg_data = std::fs::read(path).map_err(|e| {
        ggez::GameError::ResourceLoadError(format!("Failed to read SVG file: {}", e))
    })?;
    let options = resvg::usvg::Options::default();
    let tree = resvg::usvg::Tree::from_data(&svg_data, &options)
        .map_err(|e| ggez::GameError::ResourceLoadError(format!("Failed to parse SVG: {}", e)))?;
    let mut pixmap = tiny_skia::Pixmap::new(size, size)
        .ok_or_else(|| ggez::GameError::ResourceLoadError("Failed to create pixmap".to_string()))?;
    let scale_x = size as f32 / tree.size().width();
    let scale_y = size as f32 / tree.size().height();
    let transform = tiny_skia::Transform::from_scale(scale_x, scale_y);
    resvg::render(&tree, transform, &mut pixmap.as_mut());

    Ok(graphics::Image::from_pixels(
        ctx,
        pixmap.data(),
        graphics::ImageFormat::Rgba8UnormSrgb,
        size,
        size,
    ))
}

fn load_pieces(
    ctx: &mut Context,
    square_size: u32,
) -> GameResult<HashMap<(u8, u8), graphics::Image>> {
    use PieceColor::*;
    use PieceType::*;
    // Maps (color_id, kind_id) → file prefix
    // color: 0 = White, 1 = Black
    // kind:  0=P, 1=R, 2=N, 3=B, 4=Q, 5=K
    let pieces = [
        (White, Pawn, "wP"),
        (White, Rook, "wR"),
        (White, Knight, "wN"),
        (White, Bishop, "wB"),
        (White, Queen, "wQ"),
        (White, King, "wK"),
        (Black, Pawn, "bP"),
        (Black, Rook, "bR"),
        (Black, Knight, "bN"),
        (Black, Bishop, "bB"),
        (Black, Queen, "bQ"),
        (Black, King, "bK"),
    ];
    let mut map = HashMap::new();
    for (color, kind, prefix) in &pieces {
        let path = format!("pieces/{}.svg", prefix);
        let image = load_svg_as_image(ctx, &path, square_size)?;
        let key = (*color as u8, *kind as u8);
        map.insert(key, image);
    }
    Ok(map)
}

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("Chesstic", "OscarPastry")
        .window_setup(ggez::conf::WindowSetup::default().title("Chesstic"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 800.0))
        .build()
        .expect("Jesus christ, how did you even manage to mess this up.");
    let my_game = MyGame::new(&mut ctx).expect("Lmao, the computer says no.");
    event::run(ctx, event_loop, my_game);
}
#[derive(Clone, PartialEq, Eq)]
enum GameStatus {
    Playing,
    Checkmate(PieceColor),
    Stalemate,
    Draw(String),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct CastlingRights {
    white_kingside: bool,
    white_queenside: bool,
    black_kingside: bool,
    black_queenside: bool,
}
impl CastlingRights {
    fn new() -> Self {
        CastlingRights {
            white_kingside: true,
            white_queenside: true,
            black_kingside: true,
            black_queenside: true,
        }
    }
}

#[derive(Clone)]
struct GameStateSnapshot {
    board: Board,
    turn: PieceColor,
    en_passant_target: Option<(usize, usize)>,
    castling_rights: CastlingRights,
    halfmove_clock: u32,
    status: GameStatus,
    engine_eval: i32,
    best_move_hint: Option<((usize, usize), (usize, usize))>,
}

struct MyGame {
    board_mesh: Mesh,
    board: Board,
    pieces: HashMap<(u8, u8), graphics::Image>,
    square_size: f32,
    selected_piece: Option<(usize, usize)>,
    turn: PieceColor,
    legal_moves: Vec<(usize, usize)>,
    flash_timer: f32,
    en_passant_target: Option<(usize, usize)>,
    castling_rights: CastlingRights,
    status: GameStatus,
    promotion_pending: Option<(usize, usize, usize, usize)>,
    engine_eval: i32,
    best_move_hint: Option<((usize, usize), (usize, usize))>,
    board_flipped: bool,
    halfmove_clock: u32,
    position_history: HashMap<(Board, PieceColor, Option<(usize, usize)>, CastlingRights), u8>,
    history_stack: Vec<GameStateSnapshot>,
}
impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let (win_w, win_h) = ctx.gfx.drawable_size();
        let square_size = win_w.min(win_h) / 8.0;

        let board_mesh = create_chessboard(ctx)?;
        let board = inital_board();
        let pieces = load_pieces(ctx, square_size as u32)?;
        let mut game = MyGame {
            board_mesh,
            board,
            pieces,
            square_size,
            selected_piece: None,
            turn: PieceColor::White,
            legal_moves: Vec::new(),
            flash_timer: 0.0,
            en_passant_target: None,
            castling_rights: CastlingRights::new(),
            status: GameStatus::Playing,
            promotion_pending: None,
            engine_eval: 0,
            best_move_hint: None,
            board_flipped: false,
            halfmove_clock: 0,
            position_history: HashMap::new(),
            history_stack: Vec::new(),
        };

        game.position_history.insert(
            (
                game.board,
                game.turn,
                game.en_passant_target,
                game.castling_rights,
            ),
            1,
        );

        let (best_move, eval) = find_best_move(
            &game.board,
            5,
            game.turn,
            game.en_passant_target,
            game.castling_rights,
        );
        game.best_move_hint = best_move;
        game.engine_eval = eval;

        Ok(game)
    }

    pub fn reset_game(&mut self) {
        self.board = inital_board();
        self.selected_piece = None;
        self.turn = PieceColor::White;
        self.legal_moves.clear();
        self.flash_timer = 0.0;
        self.en_passant_target = None;
        self.castling_rights = CastlingRights::new();
        self.status = GameStatus::Playing;
        self.promotion_pending = None;
        self.engine_eval = 0;
        self.best_move_hint = None;
        self.halfmove_clock = 0;
        self.position_history.clear();
        self.history_stack.clear();

        self.position_history.insert(
            (
                self.board,
                self.turn,
                self.en_passant_target,
                self.castling_rights,
            ),
            1,
        );

        let (best_move, eval) = find_best_move(
            &self.board,
            3,
            self.turn,
            self.en_passant_target,
            self.castling_rights,
        );
        self.best_move_hint = best_move;
        self.engine_eval = eval;
    }

    pub fn undo_move(&mut self) {
        if let Some(snapshot) = self.history_stack.pop() {
            // Remove the current state from history before reverting
            let current_state_key = (
                self.board,
                self.turn,
                self.en_passant_target,
                self.castling_rights,
            );
            if let Some(count) = self.position_history.get_mut(&current_state_key) {
                *count = count.saturating_sub(1);
            }

            self.board = snapshot.board;
            self.turn = snapshot.turn;
            self.en_passant_target = snapshot.en_passant_target;
            self.castling_rights = snapshot.castling_rights;
            self.halfmove_clock = snapshot.halfmove_clock;
            self.status = snapshot.status;
            self.engine_eval = snapshot.engine_eval;
            self.best_move_hint = snapshot.best_move_hint;

            self.selected_piece = None;
            self.legal_moves.clear();
            self.promotion_pending = None;
            self.flash_timer = 0.0;
        }
    }

    fn apply_move(
        &mut self,
        from: (usize, usize),
        to: (usize, usize),
        promote_to: Option<PieceType>,
    ) {
        let (sel_row, sel_col) = from;
        let (row, col) = to;

        if let Some(mut piece) = self.board[sel_row][sel_col] {
            // Snapshot current state for undo
            self.history_stack.push(GameStateSnapshot {
                board: self.board,
                turn: self.turn,
                en_passant_target: self.en_passant_target,
                castling_rights: self.castling_rights,
                halfmove_clock: self.halfmove_clock,
                status: self.status.clone(),
                engine_eval: self.engine_eval,
                best_move_hint: self.best_move_hint,
            });

            let is_pawn = piece.kind == PieceType::Pawn;
            let is_capture = self.board[row][col].is_some()
                || (is_pawn && sel_col != col && self.board[row][col].is_none());

            if is_pawn || is_capture {
                self.halfmove_clock = 0;
            } else {
                self.halfmove_clock += 1;
            }

            // Castling rook movement
            if piece.kind == PieceType::King && (sel_col as i32 - col as i32).abs() == 2 {
                if col == 6 {
                    self.board[sel_row][5] = self.board[sel_row][7];
                    self.board[sel_row][7] = None;
                } else if col == 2 {
                    self.board[sel_row][3] = self.board[sel_row][0];
                    self.board[sel_row][0] = None;
                }
            }
            //En passant capture
            if piece.kind == PieceType::Pawn && sel_col != col && self.board[row][col].is_none() {
                self.board[sel_row][col] = None;
            }
            //En passant target for next turn
            let mut next_en_passant_target = None;
            if piece.kind == PieceType::Pawn && (sel_row as i32 - row as i32).abs() == 2 {
                next_en_passant_target = Some(((sel_row + row) / 2, col));
            }
            // Pawn promotion
            if let Some(new_kind) = promote_to {
                piece.kind = new_kind;
            }
            // Castling rights
            if piece.kind == PieceType::King {
                if piece.color == PieceColor::White {
                    self.castling_rights.white_kingside = false;
                    self.castling_rights.white_queenside = false;
                } else {
                    self.castling_rights.black_kingside = false;
                    self.castling_rights.black_queenside = false;
                }
            }
            if piece.kind == PieceType::Rook {
                if piece.color == PieceColor::White {
                    if sel_row == 7 && sel_col == 0 {
                        self.castling_rights.white_queenside = false;
                    } else if sel_row == 7 && sel_col == 7 {
                        self.castling_rights.white_kingside = false;
                    }
                } else {
                    if sel_row == 0 && sel_col == 0 {
                        self.castling_rights.black_queenside = false;
                    } else if sel_row == 0 && sel_col == 7 {
                        self.castling_rights.black_kingside = false;
                    }
                }
            }
            // Also update castling rights if a rook is captured!
            if row == 7 && col == 0 {
                self.castling_rights.white_queenside = false;
            }
            if row == 7 && col == 7 {
                self.castling_rights.white_kingside = false;
            }
            if row == 0 && col == 0 {
                self.castling_rights.black_queenside = false;
            }
            if row == 0 && col == 7 {
                self.castling_rights.black_kingside = false;
            }

            self.board[row][col] = Some(piece);
            self.board[sel_row][sel_col] = None;

            self.en_passant_target = next_en_passant_target;
            self.turn = self.turn.opposite(); // Switch turns
            self.selected_piece = None;
            self.legal_moves.clear();

            let state_key = (
                self.board,
                self.turn,
                self.en_passant_target,
                self.castling_rights,
            );
            let count = self.position_history.entry(state_key).or_insert(0);
            *count += 1;

            if !has_legal_moves(
                &self.board,
                self.turn,
                self.en_passant_target,
                self.castling_rights,
            ) {
                if is_in_check(&self.board, self.turn) {
                    self.status = GameStatus::Checkmate(self.turn.opposite());
                } else {
                    self.status = GameStatus::Stalemate;
                }
            } else if self.halfmove_clock >= 100 {
                self.status = GameStatus::Draw("50-Move Rule".to_string());
            } else if *count >= 3 {
                self.status = GameStatus::Draw("Threefold Repetition".to_string());
            } else if is_insufficient_material(&self.board) {
                self.status = GameStatus::Draw("Insufficient Material".to_string());
            } else {
                let (best_move, eval) = find_best_move(
                    &self.board,
                    3,
                    self.turn,
                    self.en_passant_target,
                    self.castling_rights,
                );
                self.best_move_hint = best_move;
                self.engine_eval = eval;
            }
        }
    }
}
impl EventHandler for MyGame {
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        if button != event::MouseButton::Left || self.flash_timer > 0.0 {
            return Ok(());
        }

        // UI Button clicks in the sidebar
        let (win_w, _win_h) = _ctx.gfx.drawable_size();
        if win_w > 800.0 && x >= 820.0 && x <= 980.0 {
            if y >= 180.0 && y <= 220.0 {
                self.board_flipped = !self.board_flipped;
                return Ok(());
            } else if y >= 240.0 && y <= 280.0 {
                if !self.history_stack.is_empty() {
                    self.undo_move();
                }
                return Ok(());
            } else if y >= 300.0 && y <= 340.0 {
                self.reset_game();
                return Ok(());
            }
        }

        if self.status != GameStatus::Playing {
            return Ok(());
        }

        if let Some((from_row, from_col, to_row, to_col)) = self.promotion_pending {
            let menu_x = from_col as f32 * self.square_size;
            let menu_y = if to_row == 0 {
                0.0
            } else {
                4.0 * self.square_size
            };

            if x >= menu_x && x < menu_x + self.square_size {
                if y >= menu_y && y < menu_y + 4.0 * self.square_size {
                    let index = ((y - menu_y) / self.square_size) as usize;
                    let choices = [
                        PieceType::Queen,
                        PieceType::Rook,
                        PieceType::Bishop,
                        PieceType::Knight,
                    ];
                    if index < choices.len() {
                        let promote_to = choices[index];
                        self.promotion_pending = None;
                        self.apply_move((from_row, from_col), (to_row, to_col), Some(promote_to));
                    }
                    return Ok(());
                }
            }
            return Ok(()); // Ignore clicks outside the menu
        }

        let mut col = (x / self.square_size) as usize;
        let mut row = (y / self.square_size) as usize;

        if col >= 8 || row >= 8 {
            return Ok(());
        }

        if self.board_flipped {
            col = 7 - col;
            row = 7 - row;
        }
        match self.selected_piece {
            Some((sel_row, sel_col)) => {
                if sel_row == row && sel_col == col {
                    self.selected_piece = None; // Deselect if clicked again
                    self.legal_moves.clear();
                } else if self.legal_moves.contains(&(row, col)) {
                    if let Some(p) = self.board[sel_row][sel_col] {
                        let promotion = if p.color == PieceColor::White { 0 } else { 7 };
                        let is_promotion = p.kind == PieceType::Pawn && row == promotion;
                        if is_promotion {
                            self.promotion_pending = Some((sel_row, sel_col, row, col));
                            self.selected_piece = None;
                            self.legal_moves.clear();
                        } else {
                            self.apply_move((sel_row, sel_col), (row, col), None);
                        }
                    }
                } else if let Some(piece) = self.board[row][col] {
                    if piece.color == self.turn {
                        self.selected_piece = Some((row, col)); // Select new piece
                        self.legal_moves.clear();
                        get_legal_moves(
                            &self.board,
                            row,
                            col,
                            self.en_passant_target,
                            self.castling_rights,
                            &mut self.legal_moves,
                        );
                    } else {
                        self.flash_timer = 0.001; // Start flash effect for invalid move
                        self.selected_piece = None; // Deselect current piece
                        self.legal_moves.clear();
                    }
                } else {
                    self.selected_piece = None; // Deselect if clicked empty square
                    self.legal_moves.clear();
                }
            }
            None => {
                if let Some(p) = self.board[row][col] {
                    // Select piece
                    if p.color == self.turn {
                        self.selected_piece = Some((row, col));
                        self.legal_moves.clear();
                        get_legal_moves(
                            &self.board,
                            row,
                            col,
                            self.en_passant_target,
                            self.castling_rights,
                            &mut self.legal_moves,
                        );
                    } else {
                        self.flash_timer = 0.001; // Start flash effect for invalid selection
                    }
                }
            }
        }
        Ok(())
    }
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.flash_timer > 0.0 {
            self.flash_timer += _ctx.time.delta().as_secs_f32();
            if self.flash_timer > 1.0 {
                self.flash_timer = 0.0; // Reset flash after 1 second
            }
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::new(0.1, 0.1, 0.1, 1.0));
        canvas.draw(&self.board_mesh, DrawParam::default());

        if let Some((sel_row, sel_col)) = self.selected_piece {
            let render_row = if self.board_flipped {
                7 - sel_row
            } else {
                sel_row
            };
            let render_col = if self.board_flipped {
                7 - sel_col
            } else {
                sel_col
            };
            let highlight = Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(
                    render_col as f32 * self.square_size,
                    render_row as f32 * self.square_size,
                    self.square_size,
                    self.square_size,
                ),
                Color::new(1.0, 1.0, 0.0, 0.5),
            )?;
            canvas.draw(&highlight, DrawParam::default());
        }
        if let Some((from, to)) = self.best_move_hint {
            for &(sq_row, sq_col) in &[from, to] {
                let render_row = if self.board_flipped {
                    7 - sq_row
                } else {
                    sq_row
                };
                let render_col = if self.board_flipped {
                    7 - sq_col
                } else {
                    sq_col
                };
                let hint = Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(
                        render_col as f32 * self.square_size,
                        render_row as f32 * self.square_size,
                        self.square_size,
                        self.square_size,
                    ),
                    Color::new(0.0, 0.6, 1.0, 0.25),
                )?;
                canvas.draw(&hint, DrawParam::default());
            }
        }
        for &(mr, mc) in &self.legal_moves {
            let render_r = if self.board_flipped { 7 - mr } else { mr };
            let render_c = if self.board_flipped { 7 - mc } else { mc };
            let dot = Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                [
                    render_c as f32 * self.square_size + self.square_size / 2.0,
                    render_r as f32 * self.square_size + self.square_size / 2.0,
                ],
                self.square_size * 0.15,
                0.5,
                Color::new(0.0, 0.0, 0.0, 0.3),
            )?;
            canvas.draw(&dot, DrawParam::default());
        }
        if self.flash_timer > 0.0 {
            let phase = (self.flash_timer / 0.25) as u32; // Flash frequency
            if phase % 2 == 0 {
                let flash = Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::stroke(10.0),
                    graphics::Rect::new(
                        0.0,
                        0.0,
                        ctx.gfx.drawable_size().0 as f32,
                        ctx.gfx.drawable_size().1 as f32,
                    ),
                    Color::new(1.0, 0.0, 0.0, 1.0),
                )?;
                canvas.draw(&flash, DrawParam::default());
            }
        }
        for row in 0..8usize {
            for col in 0..8usize {
                if let Some(piece) = self.board[row][col] {
                    let key = (piece.color as u8, piece.kind as u8);
                    if let Some(image) = self.pieces.get(&key) {
                        let render_r = if self.board_flipped { 7 - row } else { row };
                        let render_c = if self.board_flipped { 7 - col } else { col };
                        let x = render_c as f32 * self.square_size;
                        let y = render_r as f32 * self.square_size;
                        canvas.draw(image, DrawParam::default().dest([x, y]));
                    }
                }
            }
        }

        match &self.status {
            GameStatus::Playing => {}
            other_status => {
                let text_str = match other_status {
                    GameStatus::Checkmate(winner) => {
                        let winner_str = match winner {
                            PieceColor::White => "White",
                            PieceColor::Black => "Black",
                        };
                        format!("Checkmate! {} wins!", winner_str)
                    }
                    GameStatus::Stalemate => "Stalemate! It's a draw.".to_string(),
                    GameStatus::Draw(reason) => format!("Draw! {}", reason),
                    _ => "".to_string(),
                };

                let mut text = Text::new(text_str);
                text.set_scale(48.0);
                let text_dim = text.measure(ctx)?;
                let (win_w, win_h) = ctx.gfx.drawable_size();

                // Draw a semi-transparent background
                let bg = Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(0.0, win_h / 2.0 - 40.0, win_w, 80.0),
                    Color::new(0.0, 0.0, 0.0, 0.7),
                )?;
                canvas.draw(&bg, DrawParam::default());

                canvas.draw(
                    &text,
                    DrawParam::default()
                        .dest([
                            win_w / 2.0 - text_dim.x / 2.0,
                            win_h / 2.0 - text_dim.y / 2.0,
                        ])
                        .color(Color::new(1.0, 1.0, 1.0, 1.0)),
                );
            }
        }
        if let Some((_, from_col, to_row, _)) = self.promotion_pending {
            let color = self.turn;
            let choices = [
                PieceType::Queen,
                PieceType::Rook,
                PieceType::Bishop,
                PieceType::Knight,
            ];
            let menu_x = from_col as f32 * self.square_size;
            let menu_y = if to_row == 0 {
                0.0
            } else {
                4.0 * self.square_size
            };

            for (i, &piece_type) in choices.iter().enumerate() {
                let btn_y = menu_y + i as f32 * self.square_size;

                let bg = Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(menu_x, btn_y, self.square_size, self.square_size),
                    Color::new(0.95, 0.85, 0.55, 1.0),
                )?;
                canvas.draw(&bg, DrawParam::default());

                let key = (color as u8, piece_type as u8);
                if let Some(image) = self.pieces.get(&key) {
                    canvas.draw(image, DrawParam::default().dest([menu_x, btn_y]));
                }
            }
        }

        let (win_w, win_h) = ctx.gfx.drawable_size();
        if win_w > 800.0 {
            let sidebar = Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(800.0, 0.0, win_w - 800.0, win_h),
                Color::new(0.15, 0.15, 0.15, 1.0),
            )?;
            canvas.draw(&sidebar, DrawParam::default());

            let mut eval_text = Text::new(format!("Eval: {:.2}", self.engine_eval as f32 / 100.0));
            eval_text.set_scale(32.0);
            canvas.draw(&eval_text, DrawParam::default().dest([820.0, 40.0]));

            if let Some((from, to)) = self.best_move_hint {
                let files = ["a", "b", "c", "d", "e", "f", "g", "h"];
                let rank_from = 8 - from.0;
                let rank_to = 8 - to.0;
                let mut best_text = Text::new(format!(
                    "Best Move:\n{}{}{} {}",
                    files[from.1], rank_from, files[to.1], rank_to
                ));
                best_text.set_scale(24.0);
                canvas.draw(&best_text, DrawParam::default().dest([820.0, 100.0]));
            }

            let rotate_btn_y = 180.0;
            let rotate_bg = Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(820.0, rotate_btn_y, 160.0, 40.0),
                Color::new(0.25, 0.25, 0.25, 1.0),
            )?;
            canvas.draw(&rotate_bg, DrawParam::default());

            let mut rotate_text = Text::new("Rotate Board");
            rotate_text.set_scale(20.0);
            canvas.draw(
                &rotate_text,
                DrawParam::default().dest([835.0, rotate_btn_y + 10.0]),
            );

            let undo_btn_y = 240.0;
            let undo_bg = Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(820.0, undo_btn_y, 160.0, 40.0),
                if self.history_stack.is_empty() {
                    Color::new(0.15, 0.15, 0.15, 1.0)
                } else {
                    Color::new(0.25, 0.25, 0.25, 1.0)
                },
            )?;
            canvas.draw(&undo_bg, DrawParam::default());

            let mut undo_text = Text::new("Undo Move");
            undo_text.set_scale(20.0);
            canvas.draw(
                &undo_text,
                DrawParam::default().dest([845.0, undo_btn_y + 10.0]).color(
                    if self.history_stack.is_empty() {
                        Color::new(0.5, 0.5, 0.5, 1.0)
                    } else {
                        Color::new(1.0, 1.0, 1.0, 1.0)
                    },
                ),
            );

            let reset_btn_y = 300.0;
            let reset_bg = Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(820.0, reset_btn_y, 160.0, 40.0),
                Color::new(0.5, 0.15, 0.15, 1.0),
            )?;
            canvas.draw(&reset_bg, DrawParam::default());

            let mut reset_text = Text::new("Reset Game");
            reset_text.set_scale(20.0);
            canvas.draw(
                &reset_text,
                DrawParam::default().dest([845.0, reset_btn_y + 10.0]),
            );
        }

        canvas.finish(ctx)
    }
}
fn create_chessboard(ctx: &mut Context) -> GameResult<Mesh> {
    let (win_w, win_h) = ctx.gfx.drawable_size();
    let square_size = win_w.min(win_h) / 8.0; // Ensure squares fit within the window
    let board_size = 8;
    let mut mesh_builder = MeshBuilder::new();
    for row in 0..board_size {
        for col in 0..board_size {
            let x = col as f32 * square_size;
            let y = row as f32 * square_size;
            // Alternate between white and dark squares
            let is_dark = (row + col) % 2 == 1;
            let color = if is_dark {
                Color::new(0.451, 0.584, 0.322, 1.0) // Dark
            } else {
                Color::new(0.922, 0.925, 0.816, 1.0) // Light
            };
            // Draw a rectangle for each square
            mesh_builder.rectangle(
                ggez::graphics::DrawMode::fill(),
                ggez::graphics::Rect::new(x, y, square_size, square_size),
                color,
            )?;
        }
    }
    Ok(Mesh::from_data(ctx, mesh_builder.build()))
}

fn apply_move_pure(
    mut board: Board,
    from: (usize, usize),
    to: (usize, usize),
    mut castling_rights: CastlingRights,
    promote_to: Option<PieceType>,
) -> (Board, Option<(usize, usize)>, CastlingRights) {
    let (sel_row, sel_col) = from;
    let (row, col) = to;
    if let Some(mut piece) = board[sel_row][sel_col] {
        let mut next_en_passant_target = None;
        if piece.kind == PieceType::King && (sel_col as i32 - col as i32).abs() == 2 {
            if col == 6 {
                board[sel_row][5] = board[sel_row][7];
                board[sel_row][7] = None;
            } else if col == 2 {
                board[sel_row][3] = board[sel_row][0];
                board[sel_row][0] = None;
            }
        }
        if piece.kind == PieceType::Pawn && sel_col != col && board[row][col].is_none() {
            board[sel_row][col] = None;
        }
        if piece.kind == PieceType::Pawn && (sel_row as i32 - row as i32).abs() == 2 {
            next_en_passant_target = Some(((sel_row + row) / 2, col));
        }
        if let Some(new_kind) = promote_to {
            piece.kind = new_kind;
        }
        if piece.kind == PieceType::King {
            if piece.color == PieceColor::White {
                castling_rights.white_kingside = false;
                castling_rights.white_queenside = false;
            } else {
                castling_rights.black_kingside = false;
                castling_rights.black_queenside = false;
            }
        }
        if piece.kind == PieceType::Rook {
            if piece.color == PieceColor::White {
                if sel_row == 7 && sel_col == 0 {
                    castling_rights.white_queenside = false;
                } else if sel_row == 7 && sel_col == 7 {
                    castling_rights.white_kingside = false;
                }
            } else {
                if sel_row == 0 && sel_col == 0 {
                    castling_rights.black_queenside = false;
                } else if sel_row == 0 && sel_col == 7 {
                    castling_rights.black_kingside = false;
                }
            }
        }
        let check_square =
            |r, c| -> bool { (sel_row == r && sel_col == c) || (row == r && col == c) };
        if check_square(7, 0) {
            castling_rights.white_queenside = false;
        }
        if check_square(7, 7) {
            castling_rights.white_kingside = false;
        }
        if check_square(0, 0) {
            castling_rights.black_queenside = false;
        }
        if check_square(0, 7) {
            castling_rights.black_kingside = false;
        }

        board[row][col] = Some(piece);
        board[sel_row][sel_col] = None;
        return (board, next_en_passant_target, castling_rights);
    }
    (board, None, castling_rights)
}

//--- Engine logic below---------

fn evaluate(board: &Board) -> i32 {
    let mut score = 0;
    for r in 0..8 {
        for c in 0..8 {
            if let Some(p) = board[r][c] {
                let positional = piece_square_value(p.kind, p.color, r, c);
                let piece_score = p.kind.value() + positional;
                if p.color == PieceColor::White {
                    score += piece_score;
                } else {
                    score -= piece_score;
                }
            }
        }
    }
    score
}

//---- Move generation with ordering ----

fn move_score(
    board: &Board,
    from: (usize, usize),
    to: (usize, usize),
    promote: Option<PieceType>,
) -> i32 {
    let mut score = 0i32;

    // Reward captures based on piece value
    if let Some(victim) = board[to.0][to.1] {
        let attacker_val = board[from.0][from.1].map_or(0, |att| att.kind.value());
        score += 10 * victim.kind.value() - attacker_val;
    }

    // Reward promotions
    if let Some(promote_to) = promote {
        score += promote_to.value();
    }

    score
}

fn get_all_legal_moves(
    board: &Board,
    color: PieceColor,
    ep_target: Option<(usize, usize)>,
    castling: CastlingRights,
) -> Vec<((usize, usize), (usize, usize), Option<PieceType>)> {
    let mut moves = Vec::with_capacity(64);
    let mut piece_moves = Vec::with_capacity(32);
    for r in 0..8 {
        for c in 0..8 {
            if let Some(p) = board[r][c] {
                if p.color == color {
                    piece_moves.clear();
                    get_legal_moves(board, r, c, ep_target, castling, &mut piece_moves);
                    for &(nr, nc) in &piece_moves {
                        let promotion_row = if p.color == PieceColor::White { 0 } else { 7 };
                        if p.kind == PieceType::Pawn && nr == promotion_row {
                            moves.push(((r, c), (nr, nc), Some(PieceType::Queen)));
                            moves.push(((r, c), (nr, nc), Some(PieceType::Knight)));
                            moves.push(((r, c), (nr, nc), Some(PieceType::Rook)));
                            moves.push(((r, c), (nr, nc), Some(PieceType::Bishop)));
                        } else {
                            moves.push(((r, c), (nr, nc), None));
                        }
                    }
                }
            }
        }
    }
    moves.sort_by_key(|&(from, to, promote)| -move_score(board, from, to, promote));
    moves
}

fn get_all_legal_captures(
    board: &Board,
    color: PieceColor,
    ep_target: Option<(usize, usize)>,
    castling: CastlingRights,
) -> Vec<((usize, usize), (usize, usize), Option<PieceType>)> {
    let mut moves = Vec::with_capacity(16);
    let mut piece_moves = Vec::with_capacity(32);

    // Find king position once per color
    let mut king_pos = (0, 0);
    'find: for r in 0..8 {
        for c in 0..8 {
            if let Some(p) = board[r][c] {
                if p.color == color && p.kind == PieceType::King {
                    king_pos = (r, c);
                    break 'find;
                }
            }
        }
    }

    for r in 0..8 {
        for c in 0..8 {
            if let Some(p) = board[r][c] {
                if p.color == color {
                    piece_moves.clear();
                    get_pseudo_moves(board, r, c, ep_target, castling, &mut piece_moves);
                    for &(nr, nc) in &piece_moves {
                        let is_capture = board[nr][nc].is_some()
                            || (p.kind == PieceType::Pawn && ep_target == Some((nr, nc)));
                        let is_promotion = p.kind == PieceType::Pawn && (nr == 0 || nr == 7);
                        if !is_capture && !is_promotion {
                            continue;
                        }

                        let mut temp_board = *board;
                        temp_board[nr][nc] = temp_board[r][c];
                        temp_board[r][c] = None;

                        if let Some(piece) = temp_board[nr][nc] {
                            if piece.kind == PieceType::Pawn && nc != c && board[nr][nc].is_none() {
                                temp_board[r][nc] = None;
                            }
                        }

                        let current_king_pos = if p.kind == PieceType::King {
                            (nr, nc)
                        } else {
                            king_pos
                        };

                        if !is_attacked(
                            &temp_board,
                            current_king_pos.0,
                            current_king_pos.1,
                            color.opposite(),
                        ) {
                            if is_promotion {
                                moves.push(((r, c), (nr, nc), Some(PieceType::Queen)));
                                moves.push(((r, c), (nr, nc), Some(PieceType::Knight)));
                                moves.push(((r, c), (nr, nc), Some(PieceType::Rook)));
                                moves.push(((r, c), (nr, nc), Some(PieceType::Bishop)));
                            } else {
                                moves.push(((r, c), (nr, nc), None));
                            }
                        }
                    }
                }
            }
        }
    }
    moves.sort_by_key(|&(from, to, promote)| -move_score(board, from, to, promote));
    moves
}

//--- Minimax with alpha-beta pruning ---

fn minimax(
    board: &Board,
    depth: u8,
    mut alpha: i32,
    mut beta: i32,
    maximizing_player: bool,
    color: PieceColor,
    ep_target: Option<(usize, usize)>,
    castling: CastlingRights,
) -> i32 {
    if depth == 0 {
        return quiescence(
            board,
            alpha,
            beta,
            maximizing_player,
            color,
            ep_target,
            castling,
        );
    }
    let moves = get_all_legal_moves(board, color, ep_target, castling);
    if moves.is_empty() {
        if is_in_check(board, color) {
            return if maximizing_player { -99999 } else { 99999 };
        } else {
            0 // Stalemate
        };
    }
    if maximizing_player {
        let mut max_eval = -100000;
        for (from, to, promote) in moves {
            let (new_board, new_ep, new_castling) =
                apply_move_pure(*board, from, to, castling, promote);
            let eval = minimax(
                &new_board,
                depth - 1,
                alpha,
                beta,
                false,
                color.opposite(),
                new_ep,
                new_castling,
            );
            max_eval = max_eval.max(eval);
            alpha = alpha.max(eval);
            if beta <= alpha {
                break;
            }
        }
        max_eval
    } else {
        let mut min_eval = 100000;
        for (from, to, promote) in moves {
            let (new_board, new_ep, new_castling) =
                apply_move_pure(*board, from, to, castling, promote);
            let eval = minimax(
                &new_board,
                depth - 1,
                alpha,
                beta,
                true,
                color.opposite(),
                new_ep,
                new_castling,
            );
            min_eval = min_eval.min(eval);
            beta = beta.min(eval);
            if beta <= alpha {
                break;
            }
        }
        min_eval
    }
}
// --- Quiescence search to reduce horizon effect ---

fn quiescence(
    board: &Board,
    mut alpha: i32,
    mut beta: i32,
    maximizing_player: bool,
    color: PieceColor,
    ep_target: Option<(usize, usize)>,
    castling: CastlingRights,
) -> i32 {
    let stand_pat = evaluate(board);

    if maximizing_player {
        if stand_pat >= beta {
            return beta;
        }
        alpha = alpha.max(stand_pat);
    } else {
        if stand_pat <= alpha {
            return alpha;
        }
        beta = beta.min(stand_pat);
    }

    // Only search captures
    let captures = get_all_legal_captures(board, color, ep_target, castling);

    if maximizing_player {
        let mut max_eval = stand_pat;
        for (from, to, promote) in captures {
            let (new_board, new_ep, new_castling) =
                apply_move_pure(*board, from, to, castling, promote);
            let eval = quiescence(
                &new_board,
                alpha,
                beta,
                false,
                color.opposite(),
                new_ep,
                new_castling,
            );
            max_eval = max_eval.max(eval);
            alpha = alpha.max(eval);
            if beta <= alpha {
                break;
            }
        }
        max_eval
    } else {
        let mut min_eval = stand_pat;
        for (from, to, promote) in captures {
            let (new_board, new_ep, new_castling) =
                apply_move_pure(*board, from, to, castling, promote);
            let eval = quiescence(
                &new_board,
                alpha,
                beta,
                true,
                color.opposite(),
                new_ep,
                new_castling,
            );
            min_eval = min_eval.min(eval);
            beta = beta.min(eval);
            if beta <= alpha {
                break;
            }
        }
        min_eval
    }
}

fn find_best_move(
    board: &Board,
    depth: u8,
    color: PieceColor,
    ep_target: Option<(usize, usize)>,
    castling: CastlingRights,
) -> (Option<((usize, usize), (usize, usize))>, i32) {
    let moves = get_all_legal_moves(board, color, ep_target, castling);
    let mut best_move = None;
    let mut best_val = if color == PieceColor::White {
        -100000
    } else {
        100000
    };

    for (from, to, promote) in moves {
        let (new_board, new_ep, new_castling) =
            apply_move_pure(*board, from, to, castling, promote);
        let eval = minimax(
            &new_board,
            depth - 1,
            -100000,
            100000,
            color == PieceColor::Black,
            color.opposite(),
            new_ep,
            new_castling,
        );

        if color == PieceColor::White {
            if eval > best_val {
                best_val = eval;
                best_move = Some((from, to));
            }
        } else {
            if eval < best_val {
                best_val = eval;
                best_move = Some((from, to));
            }
        }
    }
    (best_move, best_val)
}
