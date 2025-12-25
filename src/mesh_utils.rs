use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use three_d::*;

/// Load STL file and return positions and normals
pub fn load_stl(path: &Path) -> anyhow::Result<(Vec<[f32; 3]>, Vec<[f32; 3]>)> {
    let mut file = File::open(path)?;
    let stl = stl_io::read_stl(&mut file)?;

    let mut positions = Vec::new();
    let mut normals = Vec::new();

    for face in &stl.faces {
        let normal = [face.normal[0], face.normal[1], face.normal[2]];
        for &idx in &face.vertices {
            let v = stl.vertices[idx];
            positions.push([v[0], v[1], v[2]]);
            normals.push(normal);
        }
    }

    Ok((positions, normals))
}

/// Calculate bounds of a mesh
pub fn calculate_bounds(positions: &[[f32; 3]]) -> ([f32; 3], [f32; 3]) {
    let mut min = [f32::MAX; 3];
    let mut max = [f32::MIN; 3];

    for pos in positions {
        for i in 0..3 {
            min[i] = min[i].min(pos[i]);
            max[i] = max[i].max(pos[i]);
        }
    }

    (min, max)
}

/// Calculate center of bounds
pub fn calculate_center(min: [f32; 3], max: [f32; 3]) -> [f32; 3] {
    [
        (min[0] + max[0]) / 2.0,
        (min[1] + max[1]) / 2.0,
        (min[2] + max[2]) / 2.0,
    ]
}

/// Calculate appropriate scale to fit in unit cube
pub fn calculate_scale(min: [f32; 3], max: [f32; 3]) -> f32 {
    let dx = (max[0] - min[0]).abs();
    let dy = (max[1] - min[1]).abs();
    let dz = (max[2] - min[2]).abs();
    let max_dim = dx.max(dy).max(dz);
    if max_dim > 0.0 {
        2.0 / max_dim
    } else {
        1.0
    }
}

/// Center and scale mesh positions
pub fn normalize_mesh(positions: &mut [[f32; 3]]) {
    let (min, max) = calculate_bounds(positions);
    let center = calculate_center(min, max);
    let scale = calculate_scale(min, max);

    for pos in positions.iter_mut() {
        pos[0] = (pos[0] - center[0]) * scale;
        pos[1] = (pos[1] - center[1]) * scale;
        pos[2] = (pos[2] - center[2]) * scale;
    }
}

/// Create a three-d mesh from positions and normals
pub fn create_mesh(
    _context: &Context,
    positions: &[[f32; 3]],
    normals: &[[f32; 3]],
) -> CpuMesh {
    let positions: Vec<Vec3> = positions.iter().map(|p| vec3(p[0], p[1], p[2])).collect();
    let normals: Vec<Vec3> = normals.iter().map(|n| vec3(n[0], n[1], n[2])).collect();

    CpuMesh {
        positions: Positions::F32(positions),
        normals: Some(normals),
        ..Default::default()
    }
}

/// Apply transform to positions
pub fn transform_positions(
    positions: &[[f32; 3]],
    translation: [f32; 3],
    rotation: [f32; 3],
    scale: f32,
) -> Vec<[f32; 3]> {
    let t = Mat4::from_translation(vec3(translation[0], translation[1], translation[2]));
    let rx = Mat4::from_angle_x(Deg(rotation[0]));
    let ry = Mat4::from_angle_y(Deg(rotation[1]));
    let rz = Mat4::from_angle_z(Deg(rotation[2]));
    let s = Mat4::from_scale(scale);
    let transform = t * rz * ry * rx * s;

    positions
        .iter()
        .map(|p| {
            let v = transform.transform_point(Point3::new(p[0], p[1], p[2]));
            [v.x, v.y, v.z]
        })
        .collect()
}

/// Save mesh to STL file
pub fn save_stl(
    path: &Path,
    positions: &[[f32; 3]],
    normals: &[[f32; 3]],
    binary: bool,
) -> anyhow::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    let triangles: Vec<stl_io::Triangle> = (0..positions.len() / 3)
        .map(|i| {
            let idx = i * 3;
            stl_io::Triangle {
                normal: stl_io::Normal::new(normals[idx]),
                vertices: [
                    stl_io::Vertex::new(positions[idx]),
                    stl_io::Vertex::new(positions[idx + 1]),
                    stl_io::Vertex::new(positions[idx + 2]),
                ],
            }
        })
        .collect();

    if binary {
        stl_io::write_stl(&mut writer, triangles.iter())?;
    } else {
        // Write ASCII STL
        writeln!(writer, "solid stlvi_export")?;
        for tri in &triangles {
            writeln!(
                writer,
                "  facet normal {} {} {}",
                tri.normal[0], tri.normal[1], tri.normal[2]
            )?;
            writeln!(writer, "    outer loop")?;
            for v in &tri.vertices {
                writeln!(writer, "      vertex {} {} {}", v[0], v[1], v[2])?;
            }
            writeln!(writer, "    endloop")?;
            writeln!(writer, "  endfacet")?;
        }
        writeln!(writer, "endsolid stlvi_export")?;
    }

    Ok(())
}

/// Recalculate normals for a mesh
pub fn recalculate_normals(positions: &[[f32; 3]]) -> Vec<[f32; 3]> {
    let mut normals = Vec::with_capacity(positions.len());

    for chunk in positions.chunks(3) {
        if chunk.len() == 3 {
            let v0 = vec3(chunk[0][0], chunk[0][1], chunk[0][2]);
            let v1 = vec3(chunk[1][0], chunk[1][1], chunk[1][2]);
            let v2 = vec3(chunk[2][0], chunk[2][1], chunk[2][2]);

            let edge1 = v1 - v0;
            let edge2 = v2 - v0;
            let normal = edge1.cross(edge2).normalize();

            let n = [normal.x, normal.y, normal.z];
            normals.push(n);
            normals.push(n);
            normals.push(n);
        }
    }

    normals
}
