/// Example taken & slightly modified from mikktspace crate
pub type Face = [u32; 3];

#[derive(Debug)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    tex_coord: [f32; 2],
}

struct Mesh {
    faces: Vec<Face>,
    vertices: Vec<Vertex>,
}

fn vertex(mesh: &Mesh, face: usize, vert: usize) -> &Vertex {
    let vs: &[u32; 3] = &mesh.faces[face];
    &mesh.vertices[vs[vert] as usize]
}

impl mikktspace_sys::MikkTSpaceInterface for Mesh {
    fn get_num_faces(&self) -> usize {
        self.faces.len()
    }

    fn get_num_vertices_of_face(&self, _face: usize) -> usize {
        3
    }

    fn get_position(&self, face: usize, vert: usize) -> [f32; 3] {
        vertex(self, face, vert).position
    }

    fn get_normal(&self, face: usize, vert: usize) -> [f32; 3] {
        vertex(self, face, vert).normal
    }

    fn get_tex_coord(&self, face: usize, vert: usize) -> [f32; 2] {
        vertex(self, face, vert).tex_coord
    }

    fn set_tspace_basic(&mut self, tangent: [f32; 3], sign: f32, face: usize, vert: usize) {
        println!(
            "{face}-{vert}: v: {v:?}, vn: {vn:?}, vt: {vt:?}, vx: {vx:?}, sign: {sign:?}",
            face = face,
            vert = vert,
            v = vertex(self, face, vert).position,
            vn = vertex(self, face, vert).normal,
            vt = vertex(self, face, vert).tex_coord,
            vx = tangent,
        );
    }
}

fn make_cube() -> Mesh {
    struct ControlPoint {
        uv: [f32; 2],
        dir: [f32; 3],
    }
    let mut faces = Vec::new();
    let mut ctl_pts = Vec::new();
    let mut vertices = Vec::new();

    // +x plane
    {
        let base = ctl_pts.len() as u32;
        faces.push([base, base + 1, base + 4]);
        faces.push([base + 1, base + 2, base + 4]);
        faces.push([base + 2, base + 3, base + 4]);
        faces.push([base + 3, base, base + 4]);
        ctl_pts.push(ControlPoint {
            uv: [0.0, 0.0],
            dir: [1.0, -1.0, 1.0],
        });
        ctl_pts.push(ControlPoint {
            uv: [0.0, 1.0],
            dir: [1.0, -1.0, -1.0],
        });
        ctl_pts.push(ControlPoint {
            uv: [1.0, 1.0],
            dir: [1.0, 1.0, -1.0],
        });
        ctl_pts.push(ControlPoint {
            uv: [1.0, 0.0],
            dir: [1.0, 1.0, 1.0],
        });
        ctl_pts.push(ControlPoint {
            uv: [0.5, 0.5],
            dir: [1.0, 0.0, 0.0],
        });
    }

    // -x plane
    {
        let base = ctl_pts.len() as u32;
        faces.push([base, base + 1, base + 4]);
        faces.push([base + 1, base + 2, base + 4]);
        faces.push([base + 2, base + 3, base + 4]);
        faces.push([base + 3, base, base + 4]);
        ctl_pts.push(ControlPoint {
            uv: [1.0, 0.0],
            dir: [-1.0, 1.0, 1.0],
        });
        ctl_pts.push(ControlPoint {
            uv: [1.0, 1.0],
            dir: [-1.0, 1.0, -1.0],
        });
        ctl_pts.push(ControlPoint {
            uv: [0.0, 1.0],
            dir: [-1.0, -1.0, -1.0],
        });
        ctl_pts.push(ControlPoint {
            uv: [0.0, 0.0],
            dir: [-1.0, -1.0, 1.0],
        });
        ctl_pts.push(ControlPoint {
            uv: [0.5, 0.5],
            dir: [-1.0, 0.0, 0.0],
        });
    }

    // +y plane
    {
        let base = ctl_pts.len() as u32;
        faces.push([base, base + 1, base + 4]);
        faces.push([base + 1, base + 2, base + 4]);
        faces.push([base + 2, base + 3, base + 4]);
        faces.push([base + 3, base, base + 4]);
        ctl_pts.push(ControlPoint {
            uv: [0.0, 0.0],
            dir: [1.0, 1.0, 1.0],
        });
        ctl_pts.push(ControlPoint {
            uv: [0.0, 1.0],
            dir: [1.0, 1.0, -1.0],
        });
        ctl_pts.push(ControlPoint {
            uv: [0.0, 1.0],
            dir: [-1.0, 1.0, -1.0],
        });
        ctl_pts.push(ControlPoint {
            uv: [0.0, 0.0],
            dir: [-1.0, 1.0, 1.0],
        });
        ctl_pts.push(ControlPoint {
            uv: [0.0, 0.5],
            dir: [0.0, 1.0, 0.0],
        });
    }

    // -y plane
    {
        let base = ctl_pts.len() as u32;
        faces.push([base, base + 1, base + 4]);
        faces.push([base + 1, base + 2, base + 4]);
        faces.push([base + 2, base + 3, base + 4]);
        faces.push([base + 3, base, base + 4]);
        ctl_pts.push(ControlPoint {
            uv: [0.0, 0.0],
            dir: [-1.0, -1.0, 1.0],
        });
        ctl_pts.push(ControlPoint {
            uv: [0.0, 1.0],
            dir: [-1.0, -1.0, -1.0],
        });
        ctl_pts.push(ControlPoint {
            uv: [0.0, 1.0],
            dir: [1.0, -1.0, -1.0],
        });
        ctl_pts.push(ControlPoint {
            uv: [0.0, 0.0],
            dir: [1.0, -1.0, 1.0],
        });
        ctl_pts.push(ControlPoint {
            uv: [0.0, 0.5],
            dir: [0.0, -1.0, 0.0],
        });
    }

    // +z plane
    {
        let base = ctl_pts.len() as u32;
        faces.push([base, base + 1, base + 4]);
        faces.push([base + 1, base + 2, base + 4]);
        faces.push([base + 2, base + 3, base + 4]);
        faces.push([base + 3, base, base + 4]);
        ctl_pts.push(ControlPoint {
            uv: [0.0, 0.0],
            dir: [-1.0, 1.0, 1.0],
        });
        ctl_pts.push(ControlPoint {
            uv: [0.0, 1.0],
            dir: [-1.0, -1.0, 1.0],
        });
        ctl_pts.push(ControlPoint {
            uv: [1.0, 1.0],
            dir: [1.0, -1.0, 1.0],
        });
        ctl_pts.push(ControlPoint {
            uv: [1.0, 0.0],
            dir: [1.0, 1.0, 1.0],
        });
        ctl_pts.push(ControlPoint {
            uv: [0.5, 0.5],
            dir: [0.0, 0.0, 1.0],
        });
    }

    // -z plane
    {
        let base = ctl_pts.len() as u32;
        faces.push([base, base + 1, base + 4]);
        faces.push([base + 1, base + 2, base + 4]);
        faces.push([base + 2, base + 3, base + 4]);
        faces.push([base + 3, base, base + 4]);
        ctl_pts.push(ControlPoint {
            uv: [1.0, 0.0],
            dir: [1.0, 1.0, -1.0],
        });
        ctl_pts.push(ControlPoint {
            uv: [1.0, 1.0],
            dir: [1.0, -1.0, -1.0],
        });
        ctl_pts.push(ControlPoint {
            uv: [0.0, 1.0],
            dir: [-1.0, -1.0, -1.0],
        });
        ctl_pts.push(ControlPoint {
            uv: [0.0, 0.0],
            dir: [-1.0, 1.0, -1.0],
        });
        ctl_pts.push(ControlPoint {
            uv: [0.5, 0.5],
            dir: [0.0, 0.0, -1.0],
        });
    }

    for pt in ctl_pts {
        let normal_len =
            f32::sqrt((pt.dir[0] * pt.dir[0]) + (pt.dir[1] * pt.dir[1]) + (pt.dir[2] * pt.dir[2]));
        let normal = [
            pt.dir[0] / normal_len,
            pt.dir[1] / normal_len,
            pt.dir[2] / normal_len,
        ];
        vertices.push(Vertex {
            position: [pt.dir[0] / 2.0, pt.dir[1] / 2.0, pt.dir[2] / 2.0],
            normal,
            tex_coord: pt.uv,
        });
    }

    Mesh { faces, vertices }
}

fn main() {
    let mut cube = make_cube();
    let ret = mikktspace_sys::gen_tang_space_default(&mut cube);
    assert!(ret);
}
