use mpv_client::{mpv_handle, Event, Handle};
mod shader;

static NAME_PROP_SHADER: &str = "glsl-shaders";

#[unsafe(no_mangle)]
extern "C" fn mpv_open_cplugin(handle: *mut mpv_handle) -> std::os::raw::c_int {
    let client = Handle::from_ptr(handle);

    let mut shader = shader::Builder::new().unwrap();
    shader.refresh().unwrap();
    loop {
        _ = client.set_property(NAME_PROP_SHADER, shader.shader().unwrap());

        match client.wait_event(-1.) {
            Event::Shutdown => return 0,
            event => println!("Got event: {}", event),
        }

        // let a = client.get_property(NAME_PROP_SHADER).unwrap();
        // println!("{a}");
    }
}
