use std::{self, io::Write};
use tempfile;

macro_rules! stringStrings {
    ($_t:tt $e:expr) => {$e};
    ($($s:expr)+) => {
            format!(concat!($(stringStrings!($s "{}")),+), $($s),+)
    };
}

trait Element {
    // fn position(&self) -> (f32, f32);
    // mayb setsize and stuff here
    // pub fn add(&mut self, offset: (f32, f32)) -> Self;
    fn opacity(&self) -> f32 { 1.0 }

    fn sdf(&self) -> String;
}

pub struct Builder {
    shader: tempfile::NamedTempFile,
    elements: Vec<Box<dyn Element>>,
}

impl Builder {
    pub fn new() -> std::io::Result<Self> {
        Ok(Self {
            shader: tempfile::NamedTempFile::new()?,
            elements: Vec::new(),
        })
    }

    pub fn with_element<T: Element + 'static>(mut self, element: T) -> Self {
        self.elements.push(Box::new(element));
        self
    }

    pub fn sync(&mut self) -> std::io::Result<()> {
        self.shader = tempfile::NamedTempFile::new()?;
        let src = self.build();
        write!(self.shader.as_file_mut(), "{src}")
    }

    fn build(&self) -> String {
        stringStrings!(
            r#"
            // thanks Inigo Quilez
            //!HOOK MAIN
            //!BIND HOOKED

            float sharpen(in float a) {
                return smoothstep(0.01, 0.01+0.001, a);
            }

            float sdBox( in vec2 p, in vec2 b ) {
                vec2 d = abs(p)-b;
                float dd = length(max(d,0.0)) + min(max(d.x,d.y),0.0);
                return sharpen(dd);
            }

            float sdCircle(in vec2 p, in float r) {
                float d = length(p) - r;
                return sharpen(d);
            }

            vec4 hook() {
                vec2 pos = HOOKED_pos;
                pos.x *= HOOKED_size.x/HOOKED_size.y;

                vec4 colour = HOOKED_tex(HOOKED_pos);
            "#

            self.elements
                .iter()
                .map(|e| e.sdf())
                .reduce(|a, e| a + "\n" + &e)
                .unwrap_or_default()

            r#"
                // const vec2 pos[4] = vec2[4](
                //     (REGION.xy - REGION.zw),
                //     (REGION.xy + REGION.zw),
                //     (REGION.xy - vec2(REGION.z, -REGION.w)),
                //     (REGION.xy + vec2(REGION.z, -REGION.w))
                // );
                // for(
                //     int i = 0;
                //     i < 4;
                //     i++
                // ) {
                //     float circle = sdCircle(pos[i] - cpos, 0.01);
                //     colour += (1 - circle)*0.5;
                // }
                //
                // float box = sdBox(REGION.xy - cpos, REGION.zw);
                // colour += (1 - box)*0.1;

                return colour;
            }
        "#)
        .to_owned()
    }

    pub fn shaderpath(&self) -> std::io::Result<String> {
        // hacky error conversion but o well
        Ok(self
            .shader
            .path()
            .to_str()
            .ok_or(std::io::ErrorKind::Other)?
            .to_owned())
    }
}

/// NOTE: x,y,w,h r normalised!
pub struct Rectangle {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

pub struct Circle {
    x: f32,
    y: f32,
    r: f32
}

impl Rectangle {
    pub fn new() -> Self {
        Self { x: 0., y: 0., w: 0., h: 0. }
    }

    pub fn pos(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn size(mut self, w: f32, h: f32) -> Self {
        self.w = w;
        self.h = h;
        self
    }
}

impl Circle {
    pub fn new() -> Self {
        Self { x: 0., y: 0., r: 0. }
    }

    pub fn pos(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn rad(mut self, r: f32) -> Self {
        self.r = r;
        self
    }
}

impl Element for Rectangle {
    fn opacity(&self) -> f32 {
        0.1
    }

    fn sdf(&self) -> String {
        format!(r#"{{
            float box = sdBox(vec2({}, {}) - pos, vec2({}, {}));
            colour += (1 - box)*0.1;
        }}"#, self.x, self.y, self.w, self.h)
    }
}

impl Element for Circle {
    fn opacity(&self) -> f32 {
        0.5
    }

    fn sdf(&self) -> String {
        format!(r#"{{
            float circle = sdCircle(vec2({}, {}) - pos, {});
            colour += (1 - circle)*0.5;
        }}"#, self.x, self.y, self.r)
    }
}
