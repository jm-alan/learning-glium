use glium::{
  glutin::{
    event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder,
  },
  implement_vertex,
  index::{NoIndices, PrimitiveType::TrianglesList},
  uniform, Display, Program, Surface, VertexBuffer,
};
use std::time::{Duration, Instant};

fn main() {
  #[derive(Copy, Clone)]
  struct Vertex {
    position: [f32; 2],
  }

  implement_vertex!(Vertex, position);

  let e_loop = EventLoop::new();
  let w_build = WindowBuilder::new();
  let c_build = ContextBuilder::new();

  let display = Display::new(w_build, c_build, &e_loop).unwrap();

  let indices = NoIndices(TrianglesList);

  let shader_v = r#"
    #version 140

    in vec2 position;

    uniform float t;

    void main() {
      vec2 pos = position;
      pos.x += t;
      pos.y -= t;
      gl_Position = vec4(pos, 0.0, 1.0);
    }
  "#;

  let shader_p = r#"
    #version 140

    out vec4 color;

    void main() {
      color = vec4(0.0, 0.0, 1.0, 1.0);
    }
  "#;

  let vert_one = Vertex {
    position: [0.5, 0.5],
  };
  let vert_two = Vertex {
    position: [0.5, -0.5],
  };
  let vert_three = Vertex {
    position: [-0.5, 0.5],
  };
  let triangle = vec![vert_one, vert_two, vert_three];
  let buffer_v = VertexBuffer::new(&display, &triangle).unwrap();

  let program = Program::from_source(&display, shader_v, shader_p, None).unwrap();

  let mut offset: f32 = 0.0;
  let mut op = '-';

  e_loop.run(move |ev, _, control_flow| {
    if offset > 0.5 {
      op = '-'
    } else if offset < -0.5 {
      op = '+'
    };
    if op == '-' {
      offset -= 0.002;
    } else if op == '+' {
      offset += 0.002;
    }

    let mut target = display.draw();
    target.clear_color(1.0, 0.0, 0.0, 1.0);
    target
      .draw(
        &buffer_v,
        &indices,
        &program,
        &uniform! {t: offset},
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
