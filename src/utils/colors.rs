use iced::Color;
use rand::{distributions::Uniform, prelude::Distribution};

pub fn generate_color() -> Color {
    let step = Uniform::new(0, 256);
    let mut rng = rand::thread_rng();
    let red = step.sample(&mut rng) as u8;
    let green = step.sample(&mut rng) as u8;
    let blue = step.sample(&mut rng) as u8;

    Color::from_rgb8(red, green, blue)
}
