use fraction::*;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use std::fmt::{Display, Formatter};

/*
Three distinct points are plotted at random on a Cartesian plane, for which -1000 ≤ x, y ≤ 1000, such that a triangle is formed.

Consider the following two triangles:

A(-340,495), B(-153,-910), C(835,-947)

X(-175,41), Y(-421,-714), Z(574,-645)

It can be verified that triangle ABC contains the origin, whereas triangle XYZ does not.

Using triangles.txt (right click and 'Save Link/Target As...'), a 27K text file containing the co-ordinates of one thousand "random" triangles, find the number of triangles for which the interior contains the origin.

NOTE: The first two examples in the file represent the triangles in the example given above.
 */

// If any vertex contains 0,0 then ORIGIN
// If all vertex are <>0,y, then NO ORIGIN
// If all vertex are x,<>y then NO ORIGIN

#[derive(Clone, Copy, PartialOrd, PartialEq)]
struct Vertex {
    x: i32,
    y: i32,
}

struct Triangle {
    vertex_a: Vertex,
    vertex_b: Vertex,
    vertex_c: Vertex,
}

impl Vertex {
    fn new(x: i32, y: i32) -> Self {
        Vertex {
            x,
            y,
        }
    }

}

impl Triangle {
    fn new(vertex_a: Vertex, vertex_b: Vertex, vertex_c: Vertex) -> Self {
        Triangle {
            vertex_a,
            vertex_b,
            vertex_c,
        }
    }

    fn new_from_ints(a_x: i32, a_y: i32, b_x: i32, b_y: i32, c_x: i32, c_y: i32) -> Self {
        Triangle::new(
            Vertex::new(a_x, a_y),
            Vertex::new(b_x, b_y),
            Vertex::new(c_x, c_y),
        )
    }
}

impl Display for Triangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}), ({}, {}), ({}, {})",
               self.vertex_a.x,
               self.vertex_a.y,
               self.vertex_b.x,
               self.vertex_b.y,
               self.vertex_c.x,
               self.vertex_c.y
        )
    }
}

const MIN_AXIS: i32 = -1000;
const MAX_AXIS: i32 = 1000;
const ORIGIN: Vertex = Vertex { x: 0, y: 0 };

/*
3 points
 Does a line cross Y in the coord

 If AxBx < 0, line crosses Y
 If two lines cross Y

 Y intercept for the two lines:
 c = Ay-Ax((Ay-By)/(Ax-Bx))

 y = mx + c
 x = (y-c)/m

 b = Ay - ((By - Ay)/(Bx - Ax)Ax)

 multiply the two cs: < 0 = origin in triangle
*/

fn main() -> std::io::Result<()> {
    let file = File::open("p102_triangles.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut triangles = Vec::new();
    let fract_zero: Fraction = Fraction::from(0.0);

    let mut ct: u32 = 0;
    let mut tc: u32 = 0;

    for line in buf_reader.lines() {
        let line_as_string = line.unwrap();
        let split = line_as_string.split(",");
        let mut vals: Vec<i32> = Vec::new();

        for num in split {
            vals.push(num.parse::<i32>().unwrap())
        }

        triangles.push(Triangle::new_from_ints(vals[0], vals[1], vals[2], vals[3], vals[4], vals[5]))
    }

    println!("{} triangles...", triangles.len());


    for tri in triangles {

        let ab_thru_Y = tri.vertex_a.x * tri.vertex_b.x < 0;
        let ac_thru_Y = tri.vertex_a.x * tri.vertex_c.x < 0;
        let bc_thru_Y = tri.vertex_b.x * tri.vertex_c.x < 0;

        // Skip if 0 lines through Y
        if !(ab_thru_Y || ac_thru_Y || bc_thru_Y) {
            continue;
        }

        let common_char;
        if ab_thru_Y {
            if ac_thru_Y {
                common_char = 'a';
            } else {
                common_char = 'b';
            }
        } else {
            common_char = 'c';
        }

        let (common, jay, kay) = match common_char {
            'a' => (&tri.vertex_a, &tri.vertex_b, &tri.vertex_c),
            'b' => (&tri.vertex_b, &tri.vertex_a, &tri.vertex_c),
            'c' => (&tri.vertex_c, &tri.vertex_b, &tri.vertex_a),
            _ => {panic!();}
        };

        // b = Ay - ((By - Ay)/(Bx - Ax)Ax)
        let c_jay = Fraction::from(common.y) - (Fraction::from(jay.y - common.y) / Fraction::from(jay.x - common.x) * Fraction::from(common.x));
        let c_kay = Fraction::from(common.y) - (Fraction::from(kay.y - common.y) / Fraction::from(kay.x - common.x) * Fraction::from(common.x));

        let og = (c_jay * c_kay);
        if og < fract_zero {
            ct += 1;
            continue;
        }
    }

    println!("{} triangles contain the origin", ct);

    Ok(())
}


