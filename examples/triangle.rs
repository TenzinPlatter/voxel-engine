use beryllium::*;
use bytemuck::cast_slice;
use gl33::{global_loader::*, *};

use voxel_engine::{Buffer, BufferType, ShaderProgram, VertexArray, buffer_data, clear_color};

type Vertex = [f32; 3];

const WINDOW_TITLE: &str = "Hello, Beryllium! (float)";
const VERTICES: [Vertex; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];

const VERT_SHADER: &str = r#"#version 330 core
  layout (location = 0) in vec3 pos;
  void main() {
      gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
  }
"#;

const FRAG_SHADER: &str = r#"#version 330 core
  out vec4 final_color;

  void main() {
    final_color = vec4(1.0, 0.5, 0.2, 1.0);
  }
"#;

fn main() {
    let sdl = Sdl::init(init::InitFlags::EVERYTHING);

    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_minor_version(3).unwrap();
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();

    #[cfg(target_os = "macos")]
    sdl.set_gl_context_flags(video::GlContextFlags::FORWARD_COMPATIBLE)
        .unwrap();

    let win_args = video::CreateWinArgs {
        title: WINDOW_TITLE,
        width: 800,
        height: 600,
        allow_high_dpi: true,
        borderless: false,
        resizable: false,
    };

    let win = sdl
        .create_gl_window(win_args)
        .expect("Failed to create window");
    win.set_swap_interval(video::GlSwapInterval::Vsync).unwrap();

    unsafe { load_global_gl(&|p_name| win.get_proc_address(p_name)) };

    let vao = VertexArray::new().expect("Failed to create VAO");
    vao.bind();

    // generate and bind vertex buffer object
    let vbo = Buffer::new().expect("Failed to create VBO");
    vbo.bind(BufferType::Array);

    // send the data to buffer
    buffer_data(BufferType::Array, cast_slice(&VERTICES), GL_STATIC_DRAW);

    // not a clue what this does
    unsafe {
        glVertexAttribPointer(
            0,
            3,
            GL_FLOAT,
            GL_FALSE.0 as u8,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );

        glEnableVertexAttribArray(0);
    }

    let shader_program = ShaderProgram::from_vert_frag(VERT_SHADER, FRAG_SHADER).unwrap();
    shader_program.use_program();

    clear_color(0.2, 0.3, 0.3, 1.0);

    'main_loop: loop {
        unsafe {
            glClear(GL_COLOR_BUFFER_BIT);

            glDrawArrays(GL_TRIANGLES, 0, 3);
        }

        while let Some(event) = sdl.poll_events() {
            if let (events::Event::Quit, _) = event {
                break 'main_loop;
            }
        }

        win.swap_window();
    }
}
