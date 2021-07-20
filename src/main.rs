use fraction::*;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;

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
    x: f32,
    y: f32,
}

#[derive(PartialEq, Debug)]
enum Relationship {
    Ahead,
    Behind,
    Cross,
    Above,
    Below
}

#[derive(Debug)]
struct Slope {
    m: Fraction,
    b: Fraction,
    no_slope: (bool, f32),
}

struct Triangle {
    vertex_a: Vertex,
    vertex_b: Vertex,
    vertex_c: Vertex,
}

impl Vertex {
    fn new(x: f32, y: f32) -> Self {
        Vertex {
            x,
            y,
        }
    }

    pub fn sum(&self) -> Fraction {
        Fraction::from((self.x + self.y))
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
            Vertex::new(a_x as f32, a_y as f32),
            Vertex::new(b_x as f32, b_y as f32),
            Vertex::new(c_x as f32, c_y as f32),
        )
    }
}

impl Slope {
    fn new(a: Vertex, b: Vertex) -> Self {
        let m = Slope::make_m(a, b);

        return match m {
            Ok(v) => {
                let b = Fraction::from(a.y) - (v * Fraction::from(a.x));

                Slope {
                    m: v,
                    b,
                    no_slope: (false, 0.0),
                }
            }
            Err(_e) => {
                Slope {
                    m: Fraction::from(0),
                    b: Fraction::from(0),
                    no_slope: (true, b.x as f32),
                }
            }
        };

    }

    pub fn make_m(a: Vertex, b: Vertex) -> Result<Fraction, f32> {
        let f = Fraction::from(b.y - a.y).checked_div(&Fraction::from(b.x - a.x));

        return match f {
            None => {
                Err(a.x)
            }
            Some(_) => {
                Ok(f.unwrap())
            }
        };
    }

    pub fn try_y(&self, y: f32, x: f32) -> f32 {
        let y_f = Fraction::from(y);


        if self.no_slope.0 || self.m.is_infinite() || self.b.is_infinite() {
            println!("No slope = {}, M = {}, B = {}", self.no_slope.0, self.m, self.b);
            return x;
        }

        (y_f - self.b).checked_div(&self.m).unwrap().to_f32().unwrap()
    }

    pub fn try_x(&self, x: i32) -> f32 {
        let x_f = Fraction::from(x);

        (self.m * x_f + self.b).to_f32().unwrap()
    }
}

/*fn bound_x(x: f32, a: Coordinate, b: Coordinate) -> Coordinate {

    let mut points = vec![a, b];
    points.sort_by(|a,b| a.x.partial_cmp(&b.x).unwrap());

    if x < points[0].sum() {
        return points[0];
    }

    if x > points[1].sum() {
        return points[1];
    }

    Coordinate::new(x, 0.0)
}*/

const MIN_AXIS: i32 = -1000;
const MAX_AXIS: i32 = 1000;
const ORIGIN: Vertex = Vertex { x: 0.0, y: 0.0 };

// First, determine the triangle isn't entirely away from the origin, eliminating any triangles that have NO slopes that cross 0 in both dimensions
// Figure out which vertex has a unique relationship to the origin (higher than, lower than, westerly, easterly), specifically, we need the vertex that will give two slopes that CROSS 0 in both dimensions
// Compare that vertex to the other two in terms of the linear function
// By looking at y = 0 and x = 0 of each slope (if any are 0,0 then the triangle includes the origin ofc), we can tell if the slope is ahead or behind the origin
// If the two slopes have different relationships to ORIGIN we know that the origin resides within the triangle

//A:(-340,495) = 155,B:(-153,-910) = -1063,C:(835,-947) = -112
// Outlier Y = A, > 0
// Outlier X = C, > 0
// Outliers are different so we just pick one (go with highest Y), knowing that each Vertex must be in its own quadrant
// Highest Y = A
// Ax < 0, Bx < 0, Cx > 0
// Because Ax < 0 AND Bx < 0, we know that AB must fall behind the origin because we already know that Ay and By cannot have the same relationship to 0
// Because Cx > 0 we need to check if AC falls ahead or behind the origin
// To check this take the slope AC and determine x when y is zero, if it is
// On the AC slope, when x = 0, y > 0 AND when y = 0, x > 0
// Therefore Origin behind AC slope
// AB slope behind, AC slope ahead

//A: (-175,41) = -134,B: (-421,-714) = -1135,C: (574,-645) = -71
// Outlier Y = A, > 0
// Outlier X = C, < 0
// Outliers are different!
// Highest Y = A
// By and Cy < 0
// Bx and Cx not BOTH > or < than 0
// Ax < 0, Bx < 0, Cx > 0 so we look at AC knowing AB slope MUST fall behind 0
// On the AC slope, when x = 0, y < 0 AND when y = 0, x < 0
// Therefore Origin ahead AC slope
// Both AC and AB slope are behind origin

//A: (-547,712) = -445,B: (-352,579) = 567,C: (951,-786) = -228
// Outlier y is C, only < 0
// Outlier x is C, only > 0
// So we get C slopes
// On the AC slope, when x = 0, y > 0 AND when y = 0, x > 0 (AC ahead of Origin)
// On the BC slope, when x = 0, y > 0 AND when y = 0, x > 0 (BC ahead of Origin)
// Both slopes ahead of origin
// Origin is outside of triangle

fn main() -> std::io::Result<()> {
    let file = File::open("p102_triangles.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut triangles = Vec::new();
    let fract_zero: Fraction = Fraction::from(0.0);

    let mut ct: u32 = 0;

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

        println!("({},{}),({},{}),({},{})", tri.vertex_a.x, tri.vertex_a.y, tri.vertex_b.x, tri.vertex_b.y, tri.vertex_c.x, tri.vertex_c.y, );
        let ab = Slope::new(tri.vertex_a, tri.vertex_b);
        let ac = Slope::new(tri.vertex_a, tri.vertex_c);
        let bc = Slope::new(tri.vertex_b, tri.vertex_c);
        if ab.no_slope.0 {
            println!("  AB: No traverse y = {}", ab.no_slope.1);
        } else {
            println!("  AB: y = {}x + {}", ab.m, ab.b);
        }

        if ac.no_slope.0 {
            println!("  AC: No traverse y = {}", ac.no_slope.1);
        } else {
            println!("  AC: y = {}x + {}", ac.m, ac.b);
        }

        if bc.no_slope.0 {
            println!("  BC:No traverse y = {}", bc.no_slope.1);
        } else {
            println!("  BC: y = {}x + {}", bc.m, bc.b);
        }

        // First, determine the triangle isn't entirely away from the origin, eliminating any triangles that have NO slopes that cross 0 in both dimensions
        // Or one of the vertexes is exactly on the origin
        if tri.vertex_a == ORIGIN || tri.vertex_b == ORIGIN || tri.vertex_c == ORIGIN {
            println!("      Origin in triangle because a vertex is on the origin");
            ct += 1;
            continue;
        }

        if tri.vertex_a.x < 0.0 && tri.vertex_b.x < 0.0 && tri.vertex_c.x < 0.0 {
            println!("      Entire triangle left of origin");
            continue;
        }

        if tri.vertex_a.x > 0.0 && tri.vertex_b.x > 0.0 && tri.vertex_c.x > 0.0 {
            println!("      Entire triangle right of origin");
            continue;
        }

        if tri.vertex_a.y < 0.0 && tri.vertex_b.y < 0.0 && tri.vertex_c.y < 0.0 {
            println!("      Entire triangle under origin");
            continue;
        }

        if tri.vertex_a.y > 0.0 && tri.vertex_b.y > 0.0 && tri.vertex_c.y > 0.0 {
            println!("      Entire triangle above origin");
            continue;
        }

        // Figure out which vertex has a unique relationship to the origin (higher than, lower than, westerly, easterly), specifically, we need the vertex that will give two slopes that CROSS 0 in both dimensions
        let (target_vertex, e_vertex, f_vertex)= find_unique(tri);

        // Compare that vertex to the other two in terms of the linear function so we get m and b (or a number if the line straight)
        let te = Slope::new(target_vertex, e_vertex);
        let tf = Slope::new(target_vertex, f_vertex);

        // By looking at y = 0 and x = 0 of each slope (if any are 0,0 then the triangle includes the origin ofc), we can tell if the slope is ahead or behind the origin
        let mut te_rel = Relationship::Cross;
        let mut tf_rel = Relationship::Cross;


        // y = mx + b // x = (y-b)/m
        let te_y_at_zero = if te.no_slope.0 {te.no_slope.1} else { ((fract_zero - te.b) / te.m).to_f32().unwrap() };
        let tf_y_at_zero = if tf.no_slope.0 {tf.no_slope.1} else { ((fract_zero - tf.b) / tf.m).to_f32().unwrap() };


        if te_y_at_zero == 0.0 {
            println!("      Origin in triangle because the slope TE falls on origin");
            ct += 1;
            continue;
        }

        if tf_y_at_zero == 0.0 {
            println!("      Origin in triangle because the slope TF falls on origin");
            ct += 1;
            continue;
        }

        let t_left = target_vertex.x < 0.0;
        let e_left = e_vertex.x < 0.0;
        let f_left = f_vertex.x < 0.0;

        if t_left && e_left {
            te_rel = match t_left {
                true => {Relationship::Behind }
                false => {Relationship::Ahead }
            }
        }

        if t_left && f_left {
            tf_rel = match t_left {
                true => {Relationship::Behind }
                false => {Relationship::Ahead }
            }
        }

        if te_rel == Relationship::Cross {
            te_rel = match te_y_at_zero > 0.0 {
                true => { Relationship::Ahead }
                false => { Relationship::Behind}
            }
        }

        if tf_rel == Relationship::Cross {
            tf_rel = match tf_y_at_zero > 0.0 {
                true => { Relationship::Ahead }
                false => { Relationship::Behind}
            }
        }

        println!("      TE = {:?} (y = 0, x = {})", te_rel, te_y_at_zero);
        println!("      TF = {:?} (y = 0, x = {})", tf_rel, tf_y_at_zero);

        if te_rel != tf_rel {
            println!("      Origin in triangle because the target slopes have different relationships to the origin");
            ct += 1;
            continue;
        }


        println!("      Not Origin in triangle");
    }

    println!(" {} triangles contain Origin", ct);

    Ok(())
}

// Looks at the given triangle and returns  vertex with a unique relationship to the origin, if there is multiple it returns the highest Y (favouring vertex A)
// It also gives the other two for reference
fn find_unique(tri: Triangle) -> (Vertex, Vertex, Vertex) {
    let (mut a_is_unique, mut b_is_unique, mut c_is_unique) = (true, true, true);

    let a_high = tri.vertex_a.y > 0.0;
    let b_high = tri.vertex_b.y > 0.0;
    let c_high = tri.vertex_c.y > 0.0;

    let a_right = tri.vertex_a.x > 0.0;
    let b_right = tri.vertex_b.x > 0.0;
    let c_right = tri.vertex_c.x > 0.0;

    // For each vertex, identify if it shares at least one cardinal properties with at least one other vertex
    a_is_unique = !((a_high && (b_high || c_high)) || (!a_high && (!b_high || !c_high)) || (a_right && (b_right || c_right)) || (!a_right && (!b_right || !c_right)));
    b_is_unique = !((b_high && (a_high || c_high)) || (!b_high && (!a_high || !c_high)) || (b_right && (a_right || c_right)) || (!b_right && (!a_right || !c_right)));
    c_is_unique = !((c_high && (a_high || b_high)) || (!c_high && (!a_high || !b_high)) || (c_right && (a_right || b_right)) || (!c_right && (!a_right || !b_right)));

    println!("      A: {}, B: {}, C: {}", a_is_unique, b_is_unique, c_is_unique);

    if a_is_unique {
        println!("    A is unique");
        return (tri.vertex_a, tri.vertex_b, tri.vertex_c);
    }

    if b_is_unique {
        println!("    B is unique");
        return (tri.vertex_b, tri.vertex_a, tri.vertex_c);
    }

    if c_is_unique {
        println!("    C is unique");
        return (tri.vertex_c, tri.vertex_a, tri.vertex_b);
    }

    // If none are unique, just give the highest vertex favouring vertex a
    if (tri.vertex_a.y >= tri.vertex_b.y) && (tri.vertex_a.y >= tri.vertex_c.y) {
        println!("    No unique; take A as highest");
        (tri.vertex_a, tri.vertex_b, tri.vertex_c)
    } else if (tri.vertex_b.y >= tri.vertex_a.y) && (tri.vertex_b.y >= tri.vertex_c.y) {
        println!("    No unique; take B as highest");
        (tri.vertex_b, tri.vertex_a, tri.vertex_c)
    } else {
        println!("    No unique; take C as highest");
        (tri.vertex_c, tri.vertex_a, tri.vertex_b)
    }
}