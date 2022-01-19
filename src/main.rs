use glium::{
  glutin::{
    event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder, NotCurrent,
  },
  implement_vertex,
  index::PrimitiveType::TrianglesList,
  // texture,
  uniform,
  Display,
  IndexBuffer,
  Program,
  Surface,
  VertexBuffer,
};
// use image::{ImageBuffer, ImageFormat, Rgb};
use std::time::{Duration, Instant};

mod teapot;

fn main() {
  // let image: ImageBuffer<Rgb<u8>, Vec<u8>> = image::load(
  //   Cursor::new(&include_bytes!("/Users/jm/Desktop/use-as-texture.png")),
  //   ImageFormat::Png,
  // )
  // .unwrap()
  // .to_rgb8();
  // let image_dimensions = image.dimensions();
  // let composed_image: texture::RawImage2d<u8> =
  //   texture::RawImage2d::from_raw_rgb_reversed(
  //     &image.into_raw(),
  //     image_dimensions,
  //   );

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

  let positions: VertexBuffer<teapot::Vertex> =
    VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
  let normals: VertexBuffer<teapot::Normal> =
    VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
  let indices: IndexBuffer<u16> =
    IndexBuffer::new(&display, TrianglesList, &teapot::INDICES).unwrap();

  // let texture = texture::SrgbTexture2d::new(&display, composed_image).unwrap();

  let shader_v = r#"
    #version 140

    in vec3 position;
    in vec3 normal;

    uniform mat4 matrix;

    void main() {
      gl_Position = matrix * vec4(position, 1.0);
    }
  "#;

  let shader_f = r#"
    #version 140

    out vec4 color;

    void main() {
      color = vec4(1.0, 0.0, 0.0, 1.0);
    }
  "#;

  let program =
    Program::from_source(&display, shader_v, shader_f, None).unwrap();

  // let mut offset: f32 = 0.0;
  // let mut op = '-';

  e_loop.run(move |ev, _, control_flow| {
    // if offset > 1.0 {
    //   op = '-'
    // } else if offset < -1.0 {
    //   op = '+'
    // };
    // if op == '-' {
    //   offset -= 0.002;
    // } else if op == '+' {
    //   offset += 0.002;
    // }

    // let uniforms = uniform! {
    //   matrix: [
    //     [offset.cos(), offset.sin(), 0.0, 0.0],
    //     [-offset.sin(), offset.cos(), 0.0, 0.0],
    //     [0.0, 0.0, 1.0, 0.0],
    //     [0.0 , 0.0, 0.0, 1.0]
    //   ],
    //   tex: &texture
    // };

    let matrix: [[f32; 4]; 4] = [
      [0.01, 0.0, 0.0, 0.0],
      [0.0, 0.01, 0.0, 0.0],
      [0.0, 0.0, 0.01, 0.0],
      [0.0, 0.0, 0.0, 1.0],
    ];

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);
    target
      .draw(
        (&positions, &normals),
        &indices,
        &program,
        &uniform! { matrix: matrix },
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
