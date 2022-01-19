use glium::{
  glutin::{
    event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder, NotCurrent,
  },
  implement_vertex,
  index::{NoIndices, PrimitiveType::TrianglesList},
  texture, uniform, Display, Program, Surface, VertexBuffer,
};
use image::{ImageBuffer, ImageFormat, Rgb};
use std::{
  io::Cursor,
  time::{Duration, Instant},
};

mod teapot;

fn main() {
  let image: ImageBuffer<Rgb<u8>, Vec<u8>> = image::load(
    Cursor::new(&include_bytes!("/Users/jm/Desktop/use-as-texture.png")),
    ImageFormat::Png,
  )
  .unwrap()
  .to_rgb8();
  let image_dimensions = image.dimensions();
  let composed_image: texture::RawImage2d<u8> =
    texture::RawImage2d::from_raw_rgb_reversed(
      &image.into_raw(),
      image_dimensions,
    );

  #[derive(Copy, Clone)]
  struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
  }

  implement_vertex!(Vertex, position, tex_coords);

  let e_loop: EventLoop<()> = EventLoop::new();
  let w_build: WindowBuilder = WindowBuilder::new();
  let c_build: ContextBuilder<NotCurrent> = ContextBuilder::new();

  let display: Display = Display::new(w_build, c_build, &e_loop).unwrap();

  let indices: NoIndices = NoIndices(TrianglesList);

  let positions = VertexBuffer::new(&display, &teapot::VERTICIES).unwrap();

  // let texture = texture::SrgbTexture2d::new(&display, composed_image).unwrap();

  let shader_v = r#"
    #version 140

    in vec2 position;
    in vec2 tex_coords;
    out vec2 v_tex_coords;

    uniform mat4 matrix;

    void main() {
      v_tex_coords = tex_coords;
      gl_Position = matrix * vec4(position, 0.0, 1.0);
    }
  "#;

  let shader_p = r#"
    #version 140

    in vec2 v_tex_coords;
    out vec4 color;

    uniform sampler2D tex;

    void main() {
      color = texture(tex, v_tex_coords);
    }
  "#;

  let program =
    Program::from_source(&display, shader_v, shader_p, None).unwrap();

  let mut offset: f32 = 0.0;
  let mut op = '-';

  e_loop.run(move |ev, _, control_flow| {
    if offset > 1.0 {
      op = '-'
    } else if offset < -1.0 {
      op = '+'
    };
    if op == '-' {
      offset -= 0.002;
    } else if op == '+' {
      offset += 0.002;
    }

    let uniforms = uniform! {
      matrix: [
        [offset.cos(), offset.sin(), 0.0, 0.0],
        [-offset.sin(), offset.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0 , 0.0, 0.0, 1.0]
      ],
      tex: &texture
    };

    let mut target = display.draw();
    target.clear_color(1.0, 0.0, 0.0, 1.0);
    target
      .draw(
        &buffer_v,
        &indices,
        &program,
        &uniforms,
        &Default::default(),
      )
      .unwrap();
    target.finish().unwrap();

    let next_frame = Instant::now() + Duration::from_nanos(16666667);

    *control_flow = ControlFlow::WaitUntil(next_frame);

    match ev {
      event::Event::WindowEvent { event, .. } => match event {
        event::WindowEvent::CloseRequested => {
          *control_flow = ControlFlow::Exit;
          return;
        }
        _ => return,
      },
      _ => (),
    }
  })
}
