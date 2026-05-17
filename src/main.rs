use std::collections::HashMap;

use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam, Mesh, MeshBuilder};
use ggez::{Context, ContextBuilder, GameResult};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum PieceColor {
    White,
    Black,
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
struct MyGame {
    board_mesh: Mesh,
    board: Board,
    pieces: HashMap<(u8, u8), graphics::Image>,
    square_size: f32,
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
        })
    }
}
impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::new(0.1, 0.1, 0.1, 1.0));
        canvas.draw(&self.board_mesh, DrawParam::default());
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
