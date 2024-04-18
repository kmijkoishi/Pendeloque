fn main()
{
    let vector1: [f32;4] = [1.0, 0.0, 2.0, 0.0];
    let vector2: [f32;4] = [-1.0, 3.0, -2.0, 0.0];
    let vector3: [f32;4] = [-1.0, 3.0, -4.0, 0.0];
    let vertex1: [f32;4] = [5.0, 1.0, 3.0, 1.0];
    let vertex2: [f32;4] = [3.0, -2.0, 4.0, 1.0];
    let mut point1: [[f32; 4]; 4] = get_identity();
    point1 = multiply_matrix(get_translation_homogeneous(2.0, 2.0, 2.0), point1);
    let mut point2: [[f32; 4]; 4] = get_identity();
    point2 = multiply_matrix(get_translation_homogeneous(1.0, 1.0, 1.0), point2);
    println!("Vector 1: {:?}", vector1);
    println!("Vector 2: {:?}", vector2);
    println!("Vector 3: {:?}", vector3);
    println!("Vertex 1: {:?}", vertex1);
    println!("Vertex 2: {:?}", vertex2);
    for i in &point1 {println!("Point 1: {:?}", i);}
    for i in &point2 {println!("Point 2: {:?}", i);}
    println!("multiply Point 1 and Point 2:");
    let point3: [[f32; 4]; 4] = multiply_matrix(point1, point2);
    for i in &point3 {println!("Point 3: {:?}", i);}
    let degree = 90.0f32;
    point1 = multiply_matrix(get_rotation_homogeneous('X', degree.to_radians()), point1);
    println!("point 1 after rotate 90degree by x axis: ");
    for i in &point1 {println!("\t{:?}", i);}
    println!("Size of Vector 1: {:?}", f32::sqrt(dot_product(vector1, vector1)));
    println!("Size of Vector 2: {:?}", f32::sqrt(dot_product(vector2, vector2)));
    println!("Size of Vector 3: {:?}", f32::sqrt(dot_product(vector3, vector3)));
    println!("Dot Product of Vector1 and Vector2: {:?}", dot_product(vector1, vector2));
    println!("Add Vector1 and Vector2: {:?}", addition(vector1, vector2));
    println!("Subtract Vector1 and Vector2: {:?}", substraction(vector1, vector2));
    println!("Cross Product of Vector1 and Vector2: {:?}", cross_product(vector1, vector2));
    println!("Volume of Parallelepiped of 3 Vectors: {:?}", f32::abs(dot_product(vector1, cross_product(vector2, vector3))));
    println!("Vector From vertex1 to vertex2: {:?}", get_vector_from_vertex(vertex1, vertex2));
    println!("Position of point1: {:?}", get_position(point1));
    println!("Determinant of Point 3: {}", determinant(&point3));
    //println!("test radian: {}, {}", 90.0f32.to_radians().cos(), 90.0f32.to_radians().sin());
}

fn addition(src: Vec<f32>, dst: Vec<f32>) -> Vec<f32> {
    let mut temp: Vec<f32> = Vec::new();
    let N = src.len();
    for i in 0..N
    {
        temp[i] = src[i] + dst[i];
    }
    return temp;
}
fn substraction(src: [f32; 4], dst: [f32; 4]) -> [f32; 4] {
    let mut temp: [f32; 4] = [0.0; 4];
    for i in 0..4
    {
        temp[i] = src[i] - dst[i];
    }
    return temp;
}

fn dot_product(src: [f32; 4], dst: [f32; 4]) -> f32 {
    let mut temp: f32 = 0.0;
    for i in 0..4
    {
        temp += src[i] * dst[i];
    }
    return temp;
}

fn gauss_jordan(m: &mut Matrix, inv: &mut Matrix) {
    let n = m.len();
    for i in 0..n {
        let mut max = m[i][i];
        let mut max_row = i;
        for k in (i+1)..n {
            if m[k][i] > max {
                max = m[k][i];
                max_row = k;
            }
        }
        m.swap(i, max_row);
        inv.swap(i, max_row);

        let div = m[i][i];
        for k in 0..n {
            m[i][k] /= div;
            inv[i][k] /= div;
        }

        for k in 0..n {
            if i != k {
                let coeff = m[k][i];
                for j in 0..n {
                    m[k][j] -= m[i][j] * coeff;
                    inv[k][j] -= inv[i][j] * coeff;
                }
            }
        }
    }
}


fn determinant(matrix: &Vec<Vec<f32>>, N: i32) -> f32 {
    if N == 1
    {
        return matrix[0][0];
    }
    if N == 2
    {
        let mut temp: f32 = matrix[0][0] * matrix[1][1];
        temp -= matrix[1][0] * matrix[0][1];
        return temp;
    }
    else
    {
        let mut res: f32 = 0.0;
        for i in 0..N
        {
            //let mut temp: [[f32; N-1]; N-1] = [[0.0; N-1]; N-1];
            let mut temp: Vec<Vec<f32>>;
            for n in 0..N-1
            {
                for m in 0..N-1
                {
                    temp[n][m] = matrix[n+1][if m >= i {m+1} else {m}];
                }
            }
            res += determinant(&temp);
        }
        return res;
    }
}

fn cross_product(vec1: [f32; 4], vec2: [f32; 4]) -> [f32; 4] {
    let mut temp: [[f32; 2]; 2] = [[vec1[1], vec1[2]], [vec2[1], vec2[2]]];
    let mut res: [f32; 4] = [0.0; 4];
    res[0] = determinant(&temp.to_vec());
    temp = [[vec1[0], vec1[2]], [vec2[0], vec2[2]]];
    res[1] = -1.0 * determinant(&temp.to_vec());
    temp = [[vec1[0], vec1[1]], [vec2[0], vec2[1]]];
    res[2] = determinant(&temp.to_vec());
    return res;
}

fn get_identity() -> [[f32; 4]; 4] {
    let temp: [[f32; 4]; 4] =[
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]];
    return temp;
}

fn get_uniform_scale(scale: f32) -> [[f32; 4]; 4] {
    let mut temp = get_identity();
    temp[0][0] *= scale;
    temp[1][1] *= scale;
    temp[2][2] *= scale;
    return temp;
}

fn get_linear_transform(scale_x: f32, scale_y: f32, scale_z: f32) -> [[f32; 4]; 4] {
    let mut temp = get_identity();
    temp[0][0] *= scale_x;
    temp[1][1] *= scale_y;
    temp[2][2] *= scale_z;
    return temp;
}

fn get_vector_from_vertex(src: [f32; 4], dst: [f32; 4]) -> [f32; 4] {
    let temp = substraction(dst, src);
    return temp;
}

fn flip(axis: char) -> [[f32; 4]; 4] {
    let mut temp = get_identity();
    match axis{
        'X'=>temp[0][0] = -1.0,
        'Y'=>temp[1][1] = -1.0,
        'Z'=>temp[2][2] = -1.0,
        _=> {},
    }
    return temp;
}

fn get_position(point: [[f32; 4]; 4]) -> [f32; 4]
{
    let mut temp: [f32; 4] = [0.0; 4];
    for i in 0..4
    {
        temp[i] = point[i][3];
    }
    return temp;
}

fn get_translation_homogeneous(pos_x: f32, pos_y: f32, pos_z: f32) -> [[f32; 4]; 4]
{
    let mut temp = get_identity();
    temp[0][3] = pos_x;
    temp[1][3] = pos_y;
    temp[2][3] = pos_z;
    return temp;
}

fn get_rotation_homogeneous(axis: char, radian: f32) -> [[f32; 4]; 4]
{
    let mut temp = get_identity();
    match axis{
        'X'=>{
            temp[1][1] = radian.cos();  temp[1][2] = -radian.sin();
            temp[2][1] = radian.sin();  temp[2][2] = radian.cos();
        },
        'Y'=>{
            temp[0][0] = radian.cos();  temp[0][2] = radian.sin();
            temp[2][0] = -radian.sin(); temp[2][2] = radian.cos();
        },
        'Z'=>{
            temp[0][0] = radian.cos();  temp[0][1] = -radian.sin();
            temp[1][0] = radian.sin();  temp[1][1] = radian.cos();
        },
        _=> {},
    }
    return temp;
}

fn multiply_matrix(vec1: [[f32; 4]; 4], vec2: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut temp = get_identity();
    let mut temp2 = 0.0;
    for i in 0..4
    {
        for j in 0..4
        {
            for k in 0..4
            {
                //println!("multiply {} and {}, which is at [{}, {}] and [{}, {}], as result: {}.",vec1[i][k], vec2[k][j], i, k, k, j, vec1[i][k] * vec2[k][j]);
                temp2 += vec1[i][k] * vec2[k][j];
            }
            temp[i][j] = temp2;
            temp2 = 0.0;
        }
    }
    return temp;
}