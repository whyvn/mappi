use std::io::Write;
use tempfile;

macro_rules! stringStrings {
    ($_t:tt $e:expr) => {$e};
    ($($s:expr)+) => {
            format!(concat!($(stringStrings!($s "{}")),+), $($s),+)
    };
}

/// NOTE: x,y,w,h r normalised!
pub struct Builder {
    shader: tempfile::NamedTempFile,
    x: f32,
    y: f32,
    w: f32,
    h: f32
}

impl Builder {
    pub fn new() -> std::io::Result<Self> {
        Ok(Self {
            shader: tempfile::NamedTempFile::new()?,
            x: 0.4,
            y: 0.4,
            w: 0.2,
            h: 0.2,
        })
    }

    pub fn add(&mut self, offset: (f32, f32)) {
        self.x += offset.0;
        self.y += offset.1;
    }

    fn build(&self) -> String {
        stringStrings!(
            r#"
            // thanks Inigo Quilez
            //!HOOK MAIN
            //!BIND HOOKED
            "#

            format!(
                "const vec4 REGION = vec4({}, {}, {}, {});",
                self.x, self.y, self.w, self.h
            )

            r#"
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
                vec2 cpos = HOOKED_pos;
                cpos.x *= HOOKED_size.x/HOOKED_size.y;

                vec4 colour = HOOKED_tex(HOOKED_pos);

                const vec2 pos[4] = vec2[4](
                    (REGION.xy - REGION.zw),
                    (REGION.xy + REGION.zw),
                    (REGION.xy - vec2(REGION.z, -REGION.w)),
                    (REGION.xy + vec2(REGION.z, -REGION.w))
                );
                for(
                    int i = 0;
                    i < 4;
                    i++
                ) {
                    float circle = sdCircle(pos[i] - cpos, 0.01);
                    colour += (1 - circle)*0.5;
                }

                float box = sdBox(REGION.xy - cpos, REGION.zw);
                colour += (1 - box)*0.1;

                return colour;
            }
        "#).to_owned()
    }

    pub fn refresh(&mut self) -> std::io::Result<()> {
        self.shader = tempfile::NamedTempFile::new()?;
        let src = self.build();
        write!(self.shader.as_file_mut(), "{src}")?;
        Ok(())
    }

    pub fn shader(&self) -> std::io::Result<String> {
        Ok(self.shader.path().to_str().ok_or(std::io::ErrorKind::Other)?.to_owned())
    }
}
