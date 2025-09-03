use std::io::Write;
use tempfile;

pub struct Builder {
    pub shader: tempfile::NamedTempFile,
    pub x: f32,
    pub y: f32,
}

impl Builder {
    pub fn new() -> std::io::Result<Self> {
        Ok(Self {
            shader: tempfile::NamedTempFile::new()?,
            x: 0.,
            y: 0.
        })
    }

    pub fn add(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }

    fn build(&self) -> String {
        String::from(r#"
            //!HOOK MAIN
            //!BIND HOOKED

            float sdBox( in vec2 p, in vec2 b ) {
                vec2 d = abs(p)-b;
                return length(max(d,0.0)) + min(max(d.x,d.y),0.0);
            }

            vec4 hook() {
                vec4 colour = HOOKED_tex(HOOKED_pos);

                float box = sdBox(HOOKED_pos, vec2(0.1));
                box = smoothstep(0.001, 0.001+0.0001, box);
                return colour * box;
            }
        "#)
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
