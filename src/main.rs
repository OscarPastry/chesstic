use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam, Mesh, MeshBuilder};
use ggez::{Context, ContextBuilder, GameResult};
fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("Chesstic", "OscarPastry")
        .window_setup(ggez::conf::WindowSetup::default().title("Chesstic"))
        .build()
        .expect("Failed to create context");
    let my_game = MyGame::new(&mut ctx).expect("Failed to create game");
    event::run(ctx, event_loop, my_game);
}
struct MyGame {
    board_mesh: Mesh,
}
impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let board_mesh = create_chessboard(ctx)?;
        Ok(MyGame { board_mesh })
    }
}
impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::new(0.1, 0.1, 0.1, 1.0));
        canvas.draw(&self.board_mesh, DrawParam::default());
        canvas.finish(ctx)
    }
}
fn create_chessboard(ctx: &mut Context) -> GameResult<Mesh> {
    let (win_w, win_h) = ctx.gfx.drawable_size();
    println!("Window size: {}x{}", win_w, win_h);
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
                Color::new(0.2, 0.2, 0.2, 1.0) // Dark
            } else {
                Color::new(0.9, 0.9, 0.9, 1.0) // Light
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
