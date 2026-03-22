use ggez::graphics::Color;

pub fn gen_colors(count: usize) -> Vec<Color> {
    let h_to_rgb = |h: f32| {
        let h6 = h * 6.0;
        let sector = h6 as u32;
        let frac = h6 - sector as f32;

        let (r, g, b) = match sector % 6 {
            0 => (1.0, frac, 0.0),
            1 => (1.0 - frac, 1.0, 0.0),
            2 => (0.0, 1.0, frac),
            3 => (0.0, 1.0 - frac, 1.0),
            4 => (frac, 0.0, 1.0),
            _ => (1.0, 0.0, 1.0 - frac),
        };

        Color::new(r, g, b, 1.)
    };

    (0..count)
        .map(|i| i as f32 * (1. / count as f32))
        .map(h_to_rgb)
        .collect()
}

pub fn distance2(p1: &(f32, f32), p2: &(f32, f32)) -> f32 {
    let x = p1.0 - p2.0;
    let y = p1.1 - p2.1;

    x * x + y * y
}

pub fn nearest_k(ks: &[(f32, f32)], point: &(f32, f32)) -> usize {
    let mut smallest_index = 0;
    let mut smallest_val = 2.;

    for (i, k) in ks.iter().enumerate() {
        let d = distance2(point, k);
        if d < smallest_val {
            smallest_index = i;
            smallest_val = d;
        }
    }
    smallest_index
}
