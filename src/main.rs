use river_layout_toolkit::{run, GeneratedLayout, Layout, Rectangle};
use std::convert::Infallible;

fn main() {
    let layout = Riversnap::default();
    run(layout).unwrap();
}

#[derive(Default)]
enum Mode {
    #[default]
    Tiled,
    Left,
    Right,
    Full,
}

struct Riversnap {
    mode: Mode,
    padding: u32,
}

impl Default for Riversnap {
    fn default() -> Self {
        Self { mode: Mode::default(), padding: 10 }
    }
}

impl Layout for Riversnap {
    type Error = Infallible;
    const NAMESPACE: &'static str = "riversnap";

    fn user_cmd(&mut self, cmd: String, _tags: Option<u32>, _output: &str) -> Result<(), Self::Error> {
        let cmd = cmd.trim();
        if let Some(n) = cmd.strip_prefix("padding ") {
            if let Ok(n) = n.trim().parse() {
                self.padding = n;
            }
        } else {
            match cmd {
                "left"  => self.mode = Mode::Left,
                "right" => self.mode = Mode::Right,
                "full"  => self.mode = Mode::Full,
                "tiled" => self.mode = Mode::Tiled,
                _       => {}
            }
        }
        Ok(())
    }

    fn generate_layout(
        &mut self,
        view_count: u32,
        usable_width: u32,
        usable_height: u32,
        _tags: u32,
        _output: &str,
    ) -> Result<GeneratedLayout, Self::Error> {
        let mut views = Vec::with_capacity(view_count as usize);
        let p = self.padding;
        let p2 = p * 2;

        // Single window always takes full screen regardless of mode
        if view_count <= 1 {
            views.push(Rectangle { x: p as i32, y: p as i32, width: usable_width.saturating_sub(p2), height: usable_height.saturating_sub(p2) });
            return Ok(GeneratedLayout { layout_name: "riversnap".into(), views });
        }

        let half_w = usable_width / 2;
        let rest = view_count - 1;

        match self.mode {
            // view[0] = left half, rest stack evenly on right
            Mode::Left | Mode::Tiled => {
                views.push(Rectangle { x: p as i32, y: p as i32, width: half_w.saturating_sub(p + p / 2), height: usable_height.saturating_sub(p2) });
                for i in 0..rest {
                    let cell_h = usable_height / rest;
                    views.push(Rectangle {
                        x: (half_w + p / 2) as i32,
                        y: (cell_h * i + p) as i32,
                        width: half_w.saturating_sub(p + p / 2),
                        height: cell_h.saturating_sub(p),
                    });
                }
            }
            // view[0] = right half, rest stack evenly on left
            Mode::Right => {
                views.push(Rectangle { x: (half_w + p / 2) as i32, y: p as i32, width: half_w.saturating_sub(p + p / 2), height: usable_height.saturating_sub(p2) });
                for i in 0..rest {
                    let cell_h = usable_height / rest;
                    views.push(Rectangle {
                        x: p as i32,
                        y: (cell_h * i + p) as i32,
                        width: half_w.saturating_sub(p + p / 2),
                        height: cell_h.saturating_sub(p),
                    });
                }
            }
            // view[0] = full screen, rest overlap behind it
            Mode::Full => {
                for _ in 0..view_count {
                    views.push(Rectangle { x: p as i32, y: p as i32, width: usable_width.saturating_sub(p2), height: usable_height.saturating_sub(p2) });
                }
            }
        }

        let name = match self.mode {
            Mode::Left | Mode::Tiled => "[]=",
            Mode::Right              => "=[]",
            Mode::Full               => "[ ]",
        };

        Ok(GeneratedLayout { layout_name: name.into(), views })
    }
}
