use crate::utils::{distance2, gen_colors, nearest_k};
use ggez::event::EventHandler;
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect, Text};
use ggez::input::keyboard::{Key, KeyInput};
use ggez::input::mouse::MouseButton;
use ggez::winit::keyboard::NamedKey;
use ggez::{Context, GameError};
use rand::{RngExt, rng};

pub struct KMeans {
    points: Vec<(f32, f32)>,
    ks: Vec<(f32, f32)>,
}

impl KMeans {
    pub fn new(_ctx: &mut Context) -> Self {
        Self {
            points: Vec::new(),
            ks: Vec::new(),
        }
    }

    pub fn random(&mut self, points: usize, ks: usize) {
        let mut rng = rng();
        self.points.resize_with(self.points.len() + points, || {
            (rng.random_range(0.0..1.0), rng.random_range(0.0..1.0))
        });
        self.ks.resize_with(self.ks.len() + ks, || {
            (rng.random_range(0.0..1.0), rng.random_range(0.0..1.0))
        });
    }

    fn iterate(&mut self) {
        let mut k_sum: Vec<(f32, f32, f32)> = vec![(0., 0., 0.); self.ks.len()];
        for point in self.points.iter() {
            let k = nearest_k(&self.ks, point);
            let (x, y, n) = k_sum[k];
            k_sum[k] = (x + point.0, y + point.1, n + 1.);
        }

        for (i, (x, y, n)) in k_sum.iter().enumerate() {
            let x = x / n;
            let y = y / n;
            self.ks[i] = (x, y);
        }
    }

    fn build_mesh(&self, ctx: &mut Context, size: f32) -> Mesh {
        let mut mb = MeshBuilder::new();

        mb.rectangle(
            DrawMode::stroke(10.),
            Rect::new(5., 5., size - 5., size - 5.),
            Color::WHITE,
        )
        .expect("Failed to draw outline");
        let size = size - 10.;

        let colors = gen_colors(self.ks.len());

        for point in self.points.iter() {
            let k_i = nearest_k(&self.ks, point);

            mb.circle(
                DrawMode::stroke(1.0),
                Vec2::new(point.0 * size, point.1 * size),
                size * 0.005,
                1.0,
                colors[k_i],
            )
            .expect("Failed to paint Point");
        }

        for (i, point) in self.ks.iter().enumerate() {
            mb.circle(
                DrawMode::fill(),
                Vec2::new(point.0 * size, point.1 * size),
                size * 0.005,
                1.0,
                colors[i],
            )
            .expect("Failed to paint Point");
        }

        Mesh::from_data(ctx, mb.build())
    }
}

impl EventHandler for KMeans {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        let (w, h) = ctx.gfx.size();
        let size = w.min(h);

        let fps_text = format!("FPS: {:.0}", ctx.time.fps());
        canvas.draw(Text::new(fps_text).set_scale(48.), Vec2::new(10., 10.));

        let mesh = self.build_mesh(ctx, size);
        canvas.draw(&mesh, DrawParam::default());

        canvas.finish(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> Result<(), GameError> {
        let (w, h) = ctx.gfx.size();
        let size = w.min(h) - 10.;
        let click = (x / size, y / size);
        let radius2 = 0.005 * 0.005;

        match button {
            MouseButton::Left => self.points.push(click),
            MouseButton::Middle => self.ks.push(click),
            MouseButton::Right => {
                self.points.retain(|p| distance2(&click, p) > radius2);
                self.ks.retain(|p| distance2(&click, p) > radius2);
            }
            _ => {}
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> Result<(), GameError> {
        match input.event.logical_key {
            Key::Named(NamedKey::Enter) => self.iterate(),
            Key::Named(NamedKey::Escape) => ctx.request_quit(),
            Key::Named(NamedKey::Backspace) => {
                self.ks.clear();
                self.points.clear();
            }
            Key::Character(c) if c == "r" => self.random(100, 6),
            _ => {}
        }
        Ok(())
    }

    fn quit_event(&mut self, _ctx: &mut Context) -> Result<bool, GameError> {
        Ok(false)
    }
}
