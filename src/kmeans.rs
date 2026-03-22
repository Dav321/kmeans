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

    pub fn random(&mut self, points: u32, ks: u32) {
        let mut rng = rng();

        for _ in 0..points {
            let coords = (rng.random_range(0.0..1.0), rng.random_range(0.0..1.0));
            self.points.push(coords);
        }

        for _ in 0..ks {
            let coords = (rng.random_range(0.0..1.0), rng.random_range(0.0..1.0));
            self.ks.push(coords);
        }
    }

    fn distance2(p1: &(f32, f32), p2: &(f32, f32)) -> f32 {
        let x = p1.0 - p2.0;
        let y = p1.1 - p2.1;

        x * x + y * y
    }

    fn nearest_k(&self, point: &(f32, f32)) -> usize {
        let mut smallest_index = 0;
        let mut smallest_val = 2.;

        for (i, k) in self.ks.iter().enumerate() {
            let d = Self::distance2(point, k);
            if d < smallest_val {
                smallest_index = i;
                smallest_val = d;
            }
        }
        smallest_index
    }

    fn iterate(&mut self) {
        let mut k_sum: Vec<(f32, f32, f32)> = vec![(0., 0., 0.); self.ks.len()];
        for point in self.points.iter() {
            let k = self.nearest_k(point);
            let (x, y, n) = k_sum[k];
            k_sum[k] = (x + point.0, y + point.1, n + 1.);
        }

        for (i, (x, y, n)) in k_sum.iter().enumerate() {
            let x = x / n;
            let y = y / n;
            self.ks[i] = (x, y);
        }
    }

    fn color(i: usize) -> Color {
        const COLORS: [Color; 6] = [
            Color::GREEN,
            Color::BLUE,
            Color::RED,
            Color::CYAN,
            Color::MAGENTA,
            Color::YELLOW,
        ];

        if i >= COLORS.len() {
            Color::WHITE
        } else {
            COLORS[i]
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

        for i in 0..self.points.len() {
            let point = self.points[i];
            let k_i = self.nearest_k(&point);

            mb.circle(
                DrawMode::stroke(1.0),
                Vec2::new(point.0 * size, point.1 * size),
                size * 0.005,
                1.0,
                Self::color(k_i),
            )
            .expect("Failed to paint Point");
        }

        for (i, point) in self.ks.iter().enumerate() {
            mb.circle(
                DrawMode::fill(),
                Vec2::new(point.0 * size, point.1 * size),
                size * 0.005,
                1.0,
                Self::color(i),
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
            Key::Character(c) => match &*c {
                "r" => self.random(100, 6),
                _ => {}
            },
            _ => {}
        }
        Ok(())
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
        println!("Click: {}|{} = {:?}", x, y, click);
        let radius = 0.005;
        let radius2 = radius * radius;

        match button {
            MouseButton::Left => self.points.push(click),
            MouseButton::Middle => self.ks.push(click),
            MouseButton::Right => {
                self.points = self
                    .points
                    .iter()
                    .filter(|p| Self::distance2(&click, p) > radius2)
                    .map(|p| *p)
                    .collect();
                self.ks = self
                    .ks
                    .iter()
                    .filter(|p| Self::distance2(&click, p) > radius2)
                    .map(|p| *p)
                    .collect();
            }
            _ => {}
        }

        Ok(())
    }

    fn quit_event(&mut self, _ctx: &mut Context) -> Result<bool, GameError> {
        Ok(false)
    }
}
