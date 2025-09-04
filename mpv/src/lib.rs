use mpv_client::{Event, Handle, mpv_handle};
mod shader;

static PROP_SHADER: &str = "glsl-shaders";

#[unsafe(no_mangle)]
extern "C" fn mpv_open_cplugin(handle: *mut mpv_handle) -> std::os::raw::c_int {
    println!("mappiiiiiiiiiiiiiiiiiiiiiiiiiiii");

    let client = Handle::from_ptr(handle);

    let mut shader = shader::Builder::new().unwrap()
        .with_element(shader::Rectangle::new().pos(0.4, 0.4).size(0.2, 0.2))
        .with_element(shader::Circle::new().pos(0.4+0.2, 0.4+0.2).rad(0.01))
        .with_element(shader::Circle::new().pos(0.4-0.2, 0.4-0.2).rad(0.01))
        .with_element(shader::Circle::new().pos(0.4+0.2, 0.4-0.2).rad(0.01))
        .with_element(shader::Circle::new().pos(0.4-0.2, 0.4+0.2).rad(0.01));
    shader.sync().unwrap();
    _ = client.set_property(PROP_SHADER, shader.shaderpath().unwrap());
    loop {
        match client.wait_event(-1.) {
            Event::Shutdown => return 0,
            event => println!("Got event: {}", event),
        }

        // let a = client.get_property(NAME_PROP_SHADER).unwrap();
        // println!("{a}");
    }
}
