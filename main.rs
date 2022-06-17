const WIDTH: usize = 500;
const HEIGHT: usize = 320;
const EXPORT_ICON: [u32; 400] = [
    16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215,
    16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215,
    16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215,
    16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215,
    16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215,
    16777215, 0, 0, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215,
    16777215, 0, 0, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 0,
    16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215,
    16777215, 16777215, 0, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 0,
    16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215,
    16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215,
    16777215, 0, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 0,
    0, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 0,
    16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 0, 0, 0,
    16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 0, 16777215,
    16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 0, 0, 0,
    16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 0, 16777215, 16777215,
    16777215, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16777215, 16777215, 16777215, 16777215, 16777215,
    16777215, 0, 16777215, 16777215, 16777215, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16777215, 16777215,
    16777215, 16777215, 16777215, 16777215, 0, 16777215, 16777215, 16777215, 16777215, 16777215,
    16777215, 16777215, 16777215, 16777215, 0, 0, 0, 16777215, 16777215, 16777215, 16777215,
    16777215, 16777215, 16777215, 0, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215,
    16777215, 16777215, 0, 0, 0, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215,
    16777215, 16777215, 0, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215,
    16777215, 0, 0, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215,
    16777215, 0, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215,
    16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215,
    16777215, 16777215, 0, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215,
    16777215, 16777215, 16777215, 16777215, 0, 16777215, 16777215, 16777215, 16777215, 16777215,
    16777215, 16777215, 0, 0, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215,
    16777215, 16777215, 0, 0, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215,
    16777215, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16777215, 16777215, 16777215, 16777215, 16777215,
    16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215,
    16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215,
    16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215,
    16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215, 16777215,
    16777215, 16777215, 16777215, 16777215,
];

extern crate minifb;

use minifb::{MouseButton, MouseMode, Scale, ScaleMode, Window, WindowOptions};
use std::time::{Duration, Instant};

fn main() {
    let mut buffer: Vec<u32>;

    let windowoptions = WindowOptions {
        borderless: false,
        title: true,
        resize: false,
        scale: Scale::X2,
        scale_mode: ScaleMode::AspectRatioStretch,
        topmost: false,
        transparency: false,
        none: false,
    };

    let mut window = Window::new("PixelArt", WIDTH, HEIGHT, windowoptions).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(33200))); // limit fps to 30

    let mut canvas = ui::Canvas::new();
    canvas.add_rectangle(ui::Rectangle::new([0, 12], [20, 300], 0xffffff)); // backdrop for the colour selection
    for i in 0..14 {
        // place buttons on left of the screen
        canvas.add_button(ui::Button::new(
            [2, 18 * i + 14],
            [16, 16],
            ui::Event::Select,
            0xff0000,
        ));
    }

    canvas.add_rectangle(ui::Rectangle::new([1, 13], [18, 18], 0)); // select indicator (black outline)
    canvas.add_rectangle(ui::Rectangle::new([0, 0], [0, 0], 0xff000000)); // menu backdrop
    canvas.add_rectangle(ui::Rectangle::new(
        [WIDTH as u32 - 20, 12],
        [20, 300],
        0xffffff,
    ));
    canvas.add_icon(ui::Icon::new([480, 290], [20, 20]));
    canvas.icons[0].set_pixels(EXPORT_ICON.to_vec());
    canvas.add_button(ui::Button::new(
        [480, 290],
        [20, 20],
        ui::Event::Export,
        0xffffff,
    ));

    for _i in 0..3 {
        // create the sliders in the menu
        canvas.add_slider(ui::Slider::new([0, 0], 70, 9, 255, 0xff000000));
    }

    let mut tilecolour: u32; // the colour of the tile that should be placed
    for y in 0..20 {
        // the drawing space
        for x in 0..40 {
            if (x + y) % 2 == 0 {
                tilecolour = 0xffffff;
            } else {
                tilecolour = 0xbbbbbb;
            }
            canvas.add_button(ui::Button::new(
                [110 + 7 * x, 30 + 7 * y],
                [7, 7],
                ui::Event::Draw,
                tilecolour,
            ));
        }
    }

    let first_button = canvas.check_click([110, 30]).3 as usize;

    let mut draw_colour: u32 = 0xffffff; // the current drawing colour
    let mut selected_button: usize = 0; // the colour select button with the outline
    let mut elapsed_time_since_click: Instant = Instant::now();

    let mut image = storage::Image::new(40, 20, vec![0xff000000; 20 * 40]);

    while window.is_open() {
        buffer = vec![0x8e8e8e; WIDTH * HEIGHT]; // create background

        // draw all objects in canvas to buffer
        let canvas_buffer = canvas.to_buffer();
        for i in 0..buffer.len() {
            if canvas_buffer[i] >> 24 == 0 {
                buffer[i] = canvas_buffer[i];
            }
        }
        if window.get_mouse_down(MouseButton::Left) {
            window.get_mouse_pos(MouseMode::Clamp).map(|mouse| {
                let click = canvas.check_click([mouse.0 as u32, mouse.1 as u32]); // check whether a button or slider has been clicked

                if click.0 == ui::Event::Select {
                    canvas.rectangles[1].position[1] = click.1[1] - 1;
                    draw_colour = click.2;
                } else if click.0 == ui::Event::Draw {
                    if draw_colour >> 24 == 0 {
                        canvas.buttons[click.3 as usize].colour = draw_colour;
                        image.set(click.3 as usize - first_button, draw_colour);
                    }
                } else if click.0 == ui::Event::SetSlider {
                    canvas.sliders[click.2 as usize].value =
                        (click.1[1] as f32 / canvas.sliders[click.2 as usize].max_width as f32
                            * 255.0)
                            .round() as u32;
                } else if click.0 == ui::Event::Export {
                    image.save(r"./image.png");
                }
            });
        } else if window.get_mouse_down(MouseButton::Right) {
            window.get_mouse_pos(MouseMode::Clamp).map(|mouse| {
                let click = canvas.check_click([mouse.0 as u32, mouse.1 as u32]);
                if click.0 == ui::Event::Select
                    && elapsed_time_since_click.elapsed() > Duration::from_millis(250)
                {
                    // do not allow very fast clicking to prevent flickering of the menu
                    elapsed_time_since_click = Instant::now();
                    if click.1[1] - 5 == canvas.rectangles[2].position[1]
                        && canvas.rectangles[2].colour == 0xffffff
                    {
                        canvas.rectangles[2].colour = 0xff000000;
                        for i in canvas.sliders.iter_mut() {
                            i.colour = 0xff000000;
                        }
                    } else {
                        canvas.rectangles[2].colour = 0xffffff;
                        canvas.rectangles[2].position = [click.1[0] + 20, click.1[1] - 5];
                        canvas.rectangles[2].size = [70, 27];
                        canvas.sliders[0].position = [click.1[0] + 20, click.1[1] - 5];
                        canvas.sliders[0].colour = 0xff0000;
                        canvas.sliders[1].position = [click.1[0] + 20, click.1[1] + 4];
                        canvas.sliders[1].colour = 0x00ff00;
                        canvas.sliders[2].position = [click.1[0] + 20, click.1[1] + 13];
                        canvas.sliders[2].colour = 0x0000ff;
                        selected_button = click.3 as usize;
                        for i in 0..3 {
                            canvas.sliders[i].value = (canvas.buttons[selected_button].colour
                                >> 8 * (2 - i))
                                as u8 as u32; // set the sliders to the rgb value of the button
                        }
                    }
                }
            });
        }
        canvas.buttons[selected_button].colour =
            canvas.sliders[0].value << 16 | canvas.sliders[1].value << 8 | canvas.sliders[2].value; // set the colour of the button to the value of the sliders
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

mod ui {
    #[derive(Clone, PartialEq)]
    pub enum Event {
        None,
        Select,
        Draw,
        SetSlider,
        Export,
    }
    pub struct Canvas {
        pub buttons: Vec<Button>,
        pub rectangles: Vec<Rectangle>,
        pub sliders: Vec<Slider>,
        pub icons: Vec<Icon>,
    }
    impl Canvas {
        pub fn new() -> Canvas {
            Canvas {
                buttons: Vec::new(),
                rectangles: Vec::new(),
                sliders: Vec::new(),
                icons: Vec::new(),
            }
        }
        pub fn add_button(&mut self, button: Button) {
            self.buttons.push(button);
        }
        pub fn add_rectangle(&mut self, rect: Rectangle) {
            self.rectangles.push(rect);
        }
        pub fn add_slider(&mut self, slid: Slider) {
            self.sliders.push(slid);
        }
        pub fn add_icon(&mut self, icon: Icon) {
            self.icons.push(icon);
        }

        pub fn to_buffer(&self) -> Vec<u32> {
            let mut buffer: Vec<u32> = vec![0xFF000000; crate::WIDTH * crate::HEIGHT];
            for block in &self.rectangles {
                for height in 0..block.size[1] {
                    for width in 0..block.size[0] {
                        let pixel = block.position[0] as usize
                            + block.position[1] as usize * crate::WIDTH
                            + width as usize
                            + height as usize * crate::WIDTH; // same as below comment
                        if pixel < buffer.len() {
                            buffer[pixel] = block.colour;
                        }
                    }
                }
            }

            for button in &self.buttons {
                for height in 0..button.size[1] {
                    for width in 0..button.size[0] {
                        let pixel = button.position[0] as usize
                            + button.position[1] as usize * crate::WIDTH
                            + width as usize
                            + height as usize * crate::WIDTH; // the index of the current pixel being processed
                        if pixel < buffer.len() {
                            buffer[pixel] = button.colour;
                        }
                    }
                }
            }

            for slider in &self.sliders {
                for height in 0..slider.height {
                    for width in 0..(slider.max_width as f32 * (slider.value as f32 / 255.0))
                        .round() as usize
                    {
                        let pixel = slider.position[0] as usize
                            + slider.position[1] as usize * crate::WIDTH
                            + width as usize
                            + height as usize * crate::WIDTH;
                        if pixel < buffer.len() {
                            buffer[pixel] = slider.colour;
                        }
                    }
                }
            }

            for icon in &self.icons {
                for height in 0..icon.size[1] {
                    for width in 0..icon.size[0] {
                        let pixel = icon.position[0] as usize
                            + icon.position[1] as usize * crate::WIDTH
                            + width as usize
                            + height as usize * crate::WIDTH;
                        let colour = icon.pixels[(width + height * icon.size[0]) as usize];
                        if pixel < buffer.len() && colour >> 24 == 0 {
                            buffer[pixel] = colour;
                        }
                    }
                }
            }
            buffer
        }

        pub fn check_click(&self, pos: [u32; 2]) -> (Event, [u32; 2], u32, u32) {
            let mut ret: (Event, [u32; 2], u32, u32) = (Event::None, [0; 2], 0xff000000, 0);
            let mut iter: u32 = 0;
            for button in &self.buttons {
                if pos[0] >= button.position[0]
                    && pos[0] < button.position[0] + button.size[0]
                    && pos[1] >= button.position[1]
                    && pos[1] < button.position[1] + button.size[1]
                {
                    // if the pointer is inside the button
                    ret.0 = button.onclick.clone();
                    ret.1 = button.position;
                    ret.2 = button.colour;
                    ret.3 = iter;
                    return ret;
                }
                iter += 1;
            }

            iter = 0;
            for slider in &self.sliders {
                if slider.colour != 0xff000000 {
                    // check if slider is visible
                    if pos[0] >= slider.position[0]
                        && pos[0] < slider.position[0] + slider.max_width
                        && pos[1] >= slider.position[1]
                        && pos[1] < slider.position[1] + slider.height
                    {
                        ret.0 = Event::SetSlider;
                        ret.1 = [slider.value, pos[0] - slider.position[0]];
                        ret.2 = iter;
                        return ret;
                    } else if pos[0] >= slider.position[0] + slider.max_width
                        && pos[0] < slider.position[0] + slider.max_width + 10
                        && pos[1] >= slider.position[1]
                        && pos[1] < slider.position[1] + slider.height
                    {
                        ret.0 = Event::SetSlider;
                        ret.1 = [slider.value, slider.max_width];
                        ret.2 = iter;
                        return ret;
                    } else if pos[0] >= slider.position[0] - 10
                        && pos[0] < slider.position[0] + slider.max_width
                        && pos[1] >= slider.position[1]
                        && pos[1] < slider.position[1] + slider.height
                    {
                        ret.0 = Event::SetSlider;
                        ret.1 = [slider.value, 0];
                        ret.2 = iter;
                        return ret;
                    }
                    iter += 1;
                }
            }
            ret
        }
    }

    pub struct Icon {
        position: [u32; 2],
        size: [u32; 2],
        pixels: Vec<u32>,
    }

    impl Icon {
        pub fn new(position: [u32; 2], size: [u32; 2]) -> Icon {
            Icon {
                position,
                size,
                pixels: vec![0; (size[0] * size[1]) as usize],
            }
        }

        pub fn set_pixels(&mut self, data: Vec<u32>) {
            if data.len() == self.pixels.len() {
                for i in 0..self.pixels.len() {
                    self.pixels[i] = data[i]
                }
            }
        }
    }

    pub struct Button {
        position: [u32; 2],
        size: [u32; 2],
        pub onclick: Event,
        pub colour: u32,
    }

    impl Button {
        pub fn new(position: [u32; 2], size: [u32; 2], onclick: Event, colour: u32) -> Button {
            Button {
                position,
                size,
                onclick,
                colour,
            }
        }
    }
    pub struct Rectangle {
        pub position: [u32; 2],
        pub size: [u32; 2],
        pub colour: u32,
    }

    impl Rectangle {
        pub fn new(position: [u32; 2], size: [u32; 2], colour: u32) -> Rectangle {
            Rectangle {
                position,
                size,
                colour,
            }
        }
    }

    pub struct Slider {
        pub position: [u32; 2],
        pub max_width: u32,
        height: u32,
        pub value: u32,
        pub colour: u32,
    }

    impl Slider {
        pub fn new(
            position: [u32; 2],
            max_width: u32,
            height: u32,
            value: u32,
            colour: u32,
        ) -> Slider {
            Slider {
                position,
                max_width,
                height,
                value,
                colour,
            }
        }
    }
}

mod storage {
    use std::fs::File;
    use std::io::BufWriter;
    use std::path::Path;

    pub struct Image {
        width: u32,
        height: u32,
        pixels: Vec<u32>,
    }

    impl Image {
        pub fn new(width: u32, height: u32, pixels: Vec<u32>) -> Image {
            Image {
                width,
                height,
                pixels,
            }
        }

        pub fn set(&mut self, pixel: usize, colour: u32) {
            self.pixels[pixel] = colour;
        }

        pub fn save(&self, path: &str) {
            let mut write_data: Vec<u8> = vec![0; self.width as usize * self.height as usize * 4];
            for i in 0..self.pixels.len() {
                write_data[i * 4] = (&self.pixels[i] >> 16) as u8;
                write_data[i * 4 + 1] = (&self.pixels[i] >> 8) as u8;
                write_data[i * 4 + 2] = (&self.pixels[i] >> 0) as u8;
                write_data[i * 4 + 3] = 255 - (&self.pixels[i] >> 24) as u8;
            }

            let path = Path::new(path);
            let file = File::create(path).unwrap();
            let ref mut w = BufWriter::new(file);
            let mut encoder = png::Encoder::new(w, self.width, self.height);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            encoder.set_trns(vec![0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8]);
            encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
            encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
            let source_chromaticities = png::SourceChromaticities::new(
                // Using unscaled instantiation here
                (0.31270, 0.32900),
                (0.64000, 0.33000),
                (0.30000, 0.60000),
                (0.15000, 0.06000),
            );
            encoder.set_source_chromaticities(source_chromaticities);
            let mut writer = encoder.write_header().unwrap();

            writer.write_image_data(&write_data).unwrap(); // Save
        }
    }
}
