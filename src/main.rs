use glium::{
  // draw_parameters::BackfaceCullingMode,
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
  Depth,
  DepthTest,
  Display,
  DrawParameters,
  IndexBuffer,
  Program,
  Surface,
  VertexBuffer,
};
// use image::{ImageBuffer, ImageFormat, Rgb};
use std::time::{Duration, Instant};

mod teapot;

fn view_matrix(pos: &[f32; 3], dir: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
  let f_len: f32 =
    (dir[0].powf(2.0) + dir[1].powf(2.0) + dir[2].powf(2.0)).sqrt();

  let f: [f32; 3] = [dir[0] / f_len, dir[1] / f_len, dir[2] / f_len];

  let s: [f32; 3] = [
    (up[1] * f[2]) - (up[2] * f[1]),
    (up[2] * f[0]) - (up[0] * f[2]),
    (up[0] * f[1]) - (up[1] * f[0]),
  ];

  let s_len: f32 = (s[0].powf(2.0) + s[1].powf(2.0) + s[2].powf(2.0)).sqrt();

  let s_norm: [f32; 3] = [s[0] / s_len, s[1] / s_len, s[2] / s_len];

  let u = [
    (f[1] * s_norm[2]) - (f[2] * s_norm[1]),
    (f[2] * s_norm[0]) - (f[0] * s_norm[2]),
    (f[0] * s_norm[1]) - (f[1] * s_norm[0]),
  ];

  let p = [
    -(pos[0] * s_norm[0]) - (pos[1] * s_norm[1]) - (pos[2] * s_norm[2]),
    -(pos[0] * u[0]) - (pos[1] * u[1]) - (pos[2] * u[2]),
    -(pos[0] * f[0]) - (pos[1] * f[1]) - (pos[2] * f[2]),
  ];

  [
    [s_norm[0], u[0], f[0], 0.0],
    [s_norm[1], u[1], f[1], 0.0],
    [s_norm[2], u[2], f[2], 0.0],
    [p[0], p[1], p[2], 1.0],
  ]
}
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
  let c_build: ContextBuilder<NotCurrent> =
    ContextBuilder::new().with_depth_buffer(24);

  let display: Display = Display::new(w_build, c_build, &e_loop).unwrap();

  let positions: VertexBuffer<teapot::Vertex> =
    VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
  let normals: VertexBuffer<teapot::Normal> =
    VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
  let indices: IndexBuffer<u16> =
    IndexBuffer::new(&display, TrianglesList, &teapot::INDICES).unwrap();

  // let texture = texture::SrgbTexture2d::new(&display, composed_image).unwrap();

  const SHADER_V: &str = r#"
    #version 150

    in vec3 position;
    in vec3 normal;

    out vec3 v_normal;

    uniform mat4 perspective;
    uniform mat4 view;
    uniform mat4 model;

    void main() {
      mat4 modelview = view * model;
      v_normal = transpose(inverse(mat3(modelview))) * normal;
      gl_Position = perspective * modelview * vec4(position, 1.0);
    }
  "#;

  const SHADER_F: &str = r#"
    #version 140

    in vec3 v_normal;
    out vec4 color;
    uniform vec3 u_light;

    void main() {
      float brightness = dot(normalize(v_normal), normalize(u_light));
      vec3 dark_color = vec3(0.6, 0.0, 0.0);
      vec3 regular_color = vec3(1.0, 0.0, 0.0);
      color = vec4(mix(dark_color, regular_color, brightness), 1.0);
    }
  "#;

  let program: Program =
    Program::from_source(&display, SHADER_V, SHADER_F, None).unwrap();

  let mut offset: f32 = 0.0;
  let mut op = '-';
  const OFFSET_BOUND: f32 = 1.0;
  const OFFSET_INCR: f32 = 0.005;

  e_loop.run(move |ev, _, control_flow| {
    if offset > OFFSET_BOUND {
      op = '-'
    } else if offset < -OFFSET_BOUND {
      op = '+'
    };
    if op == '-' {
      offset -= OFFSET_INCR;
    } else if op == '+' {
      offset += OFFSET_INCR;
    }

    // let uniforms = uniform! {
    //   matrix: [
    //     [offset.cos(), offset.sin(), 0.0, 0.0],
    //     [-offset.sin(), offset.cos(), 0.0, 0.0],
    //     [0.0, 0.0, 1.0, 0.0],
    //     [0.0 , 0.0, 0.0, 1.0]
    //   ],
    //   tex: &texture
    // };

    let position_matrix: [[f32; 4]; 4] = [
      [0.01, 0.0, 0.0, 0.0],
      [0.0, 0.01, 0.0, 0.0],
      [0.0, 0.0, 0.01, 0.0],
      [0.5, -0.5, 1.5, 1.0],
    ];

    let light: [f32; 3] = [1.0, 1.0, 0.5];

    let mut target = display.draw();

    target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

    let perspective_matrix: [[f32; 4]; 4] = {
      let (width, height) = target.get_dimensions();
      let aspect_ratio = height as f32 / width as f32;
      let fov: f32 = 3.1415926535897932384626 / 3.0;
      let z_far = (2 ^ 10) as f32;
      let z_near: f32 = 0.1;
      let f: f32 = (fov / 2.0).tan();

      [
        [f * aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, ((z_far + z_near) / (z_far - z_near)), 1.0],
        [0.0, 0.0, (-(2.0 * z_far * z_near) / (z_far - z_near)), 0.0],
      ]
    };

    let params: DrawParameters = DrawParameters {
      depth: Depth {
        test: DepthTest::IfLess,
        write: true,
        ..Default::default()
      },
      // backface_culling: BackfaceCullingMode::CullClockwise,
      ..Default::default()
    };

    let view_pos: [f32; 3] = [2.0 + offset, -0.5, 1.0];
    let view_dir: [f32; 3] = [-2.0, 1.0, 1.0];
    let view_up: [f32; 3] = [0.0, 1.0, offset];
    let view: [[f32; 4]; 4] = view_matrix(&view_pos, &view_dir, &view_up);

    target
      .draw(
        (&positions, &normals),
        &indices,
        &program,
        &uniform! {
          view: view,
          model: position_matrix,
          u_light: light,
          perspective: perspective_matrix
        },
        &params,
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
