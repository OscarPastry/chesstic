use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam, Mesh, MeshBuilder};
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
#[derive(Clone, Copy)]
struct Piece {
    color: PieceColor,
    kind: PieceType,
}
type Board = [[Option<Piece>; 8]; 8];

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
) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();
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
    moves
}

fn get_pseudo_moves(
    board: &Board,
    row: usize,
    col: usize,
    en_passant_target: Option<(usize, usize)>,
    castling_rights: CastlingRights,
) -> Vec<(usize, usize)> {
    let piece = match board[row][col] {
        Some(p) => p,
        None => return Vec::new(),
    };
    let color = piece.color;
    let mut moves = Vec::new();

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
            moves.extend(slide(
                board,
                row,
                col,
                color,
                &[(-1, -1), (-1, 1), (1, -1), (1, 1)],
            ));
        }
        PieceType::Rook => {
            moves.extend(slide(
                board,
                row,
                col,
                color,
                &[(-1, 0), (1, 0), (0, -1), (0, 1)],
            ));
        }
        PieceType::Queen => {
            moves.extend(slide(
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
            ));
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
            // King must be on its original square and not currently in check
            if row == king_row && col == 4 && !is_attacked(board, row, col, enemy_color) {
                // Kingside castling
                if (color == PieceColor::White && castling_rights.white_kingside)
                    || (color == PieceColor::Black && castling_rights.black_kingside)
                {
                    if board[king_row][5].is_none()
                        && board[king_row][6].is_none()
                        && !is_attacked(board, king_row, 5, enemy_color)
                        && !is_attacked(board, king_row, 6, enemy_color)
                    {
                        moves.push((king_row, 6));
                    }
                }
                // Queenside castling
                if (color == PieceColor::White && castling_rights.white_queenside)
                    || (color == PieceColor::Black && castling_rights.black_queenside)
                {
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
    moves
}
fn is_attacked(board: &Board, row: usize, col: usize, by_color: PieceColor) -> bool {
    for r in 0..8 {
        for c in 0..8 {
            if let Some(p) = board[r][c] {
                if p.color == by_color
                    && get_pseudo_moves(board, r, c, None, CastlingRights::none())
                        .contains(&(row, col))
                {
                    return true;
                }
            }
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
    for r in 0..8 {
        for c in 0..8 {
            if let Some(p) = board[r][c] {
                if p.color != color && get_pseudo_moves(board, r, c).contains(&king) {
                    return true;
                }
            }
        }
    }
    false
}

fn get_legal_moves(board: &Board, row: usize, col: usize) -> Vec<(usize, usize)> {
    let color = match board[row][col] {
        Some(p) => p.color,
        None => return Vec::new(),
    };
    get_pseudo_moves(board, row, col, None, CastlingRights::new())
        .into_iter()
        .filter(|&(nr, nc)| {
            let mut temp_board = *board;
            temp_board[nr][nc] = temp_board[row][col];
            temp_board[row][col] = None;
            //Keep the moves only if King is not in check after it
            !is_in_check(&temp_board, color)
        })
        .collect()
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
        .expect("Failed to create context");
    let my_game = MyGame::new(&mut ctx).expect("Failed to create game");
    event::run(ctx, event_loop, my_game);
}
#[derive(Clone, Copy)]
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
    fn none() -> Self {
        CastlingRights {
            white_kingside: false,
            white_queenside: false,
            black_kingside: false,
            black_queenside: false,
        }
    }
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
    castling_rights: [(bool, bool); 2], // [(white_kingside, white_queenside), (black_kingside, black_queenside)]
}
impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let (win_w, win_h) = ctx.gfx.drawable_size();
        let square_size = win_w.min(win_h) / 8.0;

        let board_mesh = create_chessboard(ctx)?;
        let board = inital_board();
        let pieces = load_pieces(ctx, square_size as u32)?;
        Ok(MyGame {
            board_mesh,
            board,
            pieces,
            square_size,
            selected_piece: None,
            turn: PieceColor::White,
            legal_moves: Vec::new(),
            flash_timer: 0.0,
            en_passant_target: None,
            castling_rights: [(true, true), (true, true)],
        })
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
        let col = (x / self.square_size) as usize;
        let row = (y / self.square_size) as usize;

        if col >= 8 || row >= 8 {
            return Ok(());
        }
        match self.selected_piece {
            Some((sel_row, sel_col)) => {
                if sel_row == row && sel_col == col {
                    self.selected_piece = None; // Deselect if clicked again
                    self.legal_moves.clear();
                } else if self.legal_moves.contains(&(row, col)) {
                    // Attempt to move piece
                    if let Some(piece) = self.board[sel_row][sel_col] {
                        self.board[row][col] = Some(piece);
                        self.board[sel_row][sel_col] = None;
                        self.turn = self.turn.opposite(); // Switch turns
                        self.selected_piece = None;
                        self.legal_moves.clear();
                    }
                } else if let Some(piece) = self.board[row][col] {
                    if piece.color == self.turn {
                        self.selected_piece = Some((row, col)); // Select new piece
                        self.legal_moves = get_legal_moves(&self.board, row, col);
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
                        self.legal_moves = get_legal_moves(&self.board, row, col);
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
            let x = sel_col as f32 * self.square_size;
            let y = sel_row as f32 * self.square_size;
            let highlight = Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(x, y, self.square_size, self.square_size),
                Color::new(1.0, 1.0, 0.0, 0.4),
            )?;
            canvas.draw(&highlight, DrawParam::default());
        }
        for &(mr, mc) in &self.legal_moves {
            let dot = Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                [
                    mc as f32 * self.square_size + self.square_size / 2.0,
                    mr as f32 * self.square_size + self.square_size / 2.0,
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
                        let x = col as f32 * self.square_size;
                        let y = row as f32 * self.square_size;
                        canvas.draw(image, DrawParam::default().dest([x, y]));
                    }
                }
            }
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
