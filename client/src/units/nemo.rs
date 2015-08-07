use glium::{VertexBuffer, Program, Frame, DrawError};
use glium::index::{NoIndices, PrimitiveType};
use glium::backend::Facade;
use xmath::Matrix;
use traits::{Object, Move};
use error::CreationError;
use super::{vec, Unit};

pub struct Nemo {
    unit: Unit,
    state: State,
}

enum State {
    /// Nemo is stopped
    Stopped,
    /// Nemo is moving
    Moving { dest: (f32, f32) },
    /// Nemo is using Q (0 <= t < 1)
    QSkill { t: f32 }
}

impl Nemo {
    pub fn new<F: Facade>(facade: &F) -> Result<Self, CreationError> {
        let unit = Unit {
            vb: try!(VertexBuffer::new(facade, &{
                vec![
                    vec(  4.0,  0.0 ),
                    vec( -4.0,  1.5 ),
                    vec( -4.0, -1.5 ),
                ]
            })),
            ib: NoIndices(PrimitiveType::TriangleStrip),
            program: try!(Program::from_source(facade, r#"
                #version 410
                uniform mat4 matrix;
                in vec2 position;

                void main() {
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
                }
            "#, r#"
                #version 410
                uniform int q;
                out vec3 color;

                void main() {
                    if (q == 1) {
                        color = vec3(0.533333, 0.4, 1.0);
                    } else {
                        color = vec3(1.0, 0.82745, 0.14118);
                    }
                }
            "#, None)),
            pos: (0.0, 0.0),
            angle: 0.0,
        };

        Ok(Nemo { unit: unit, state: State::Stopped })
    }
}

impl Object for Nemo {
    fn update(&mut self, elapsed: f32) {
        let mut next = None;

        match self.state {
            State::Stopped => {}
            State::Moving { dest } => {
                let unit = &mut self.unit;

                let dx = dest.0 - unit.pos.0;
                let dy = dest.1 - unit.pos.1;

                let left_dist = (dx*dx + dy*dy).sqrt();

                let speed = 50.0;
                let diff = speed*elapsed;

                if left_dist <= diff {
                    // 도착
                    unit.pos = dest;
                    next = Some(State::Stopped);
                } else {
                    unit.pos.0 += diff*unit.angle.cos();
                    unit.pos.1 += diff*unit.angle.sin();
                }
            }
            State::QSkill { ref mut t } => {
                *t += elapsed;

                if 1.0 <= *t {
                    next = Some(State::Stopped);
                }
            }
        };

        next.map(|next| {
            self.state = next;
        });
    }

    fn draw(&self, target: &mut Frame, camera: &Matrix) -> Result<(), DrawError> {
        let uniforms = uniform! {
            q: match self.state { State::QSkill { .. } => 1, _ => 0 }
        };
        self.unit.draw(target, camera, uniforms)
    }
}

impl Move for Nemo {
    fn go(&mut self, dest: (f32, f32)) {
        match self.state {
            State::QSkill { .. } => return,
            _ => ()
        }

        let unit = &mut self.unit;
        if unit.pos == dest { return; }

        let dx = dest.0 - unit.pos.0;
        let dy = dest.1 - unit.pos.1;
        unit.angle = dy.atan2(dx);
        self.state = State::Moving { dest: dest };
    }
}

impl Nemo {
    pub fn q(&mut self) {
        self.state = State::QSkill { t: 0.0 };
    }
}