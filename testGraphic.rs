fn main()
{
    // let mut matrix: [[f32;4]; 4] = [[0.0; 4]; 4];
    // for i in 0..4
    // {
    //     for j in 0..4
    //     {
    //         matrix[i][j] = (i+j) as f32;
    //     }
    // }
    // for i in 0..4
    // {
    //     for j in 0..4
    //     {
    //         print!("{}, ", matrix[i][j]);
    //     }
    //     println!();
    // }
    let vector1: [f32;4] = [1.0, 0.0, 2.0, 0.0];
    let vector2: [f32;4] = [-1.0, 3.0, -2.0, 0.0];
    let vector3: [f32;4] = [-1.0, 3.0, -4.0, 0.0];
    let vertex1: [f32;4] = [5.0, 1.0, 3.0, 1.0];
    let vertex2: [f32;4] = [3.0, -2.0, 4.0, 1.0];
    println!("Vector 1: {:?}", vector1);
    println!("Vector 2: {:?}", vector2);
    println!("Vector 3: {:?}", vector3);
    println!("Vertex 1: {:?}", vertex1);
    println!("Vertex 2: {:?}", vertex2);
    println!("Size of Vector 1: {:?}", f32::sqrt(dot_product(vector1, vector1)));
    println!("Size of Vector 2: {:?}", f32::sqrt(dot_product(vector2, vector2)));
    println!("Size of Vector 3: {:?}", f32::sqrt(dot_product(vector3, vector3)));
    println!("Dot Product of Vector1 and Vector2: {:?}", dot_product(vector1, vector2));
    println!("Add Vector1 and Vector2: {:?}", addition(vector1, vector2));
    println!("Subtract Vector1 and Vector2: {:?}", substraction(vector1, vector2));
    println!("Cross Product of Vector1 and Vector2: {:?}", cross_product(vector1, vector2));
    println!("Volume of Parallelepiped of 3 Vectors: {:?}", f32::abs(dot_product(vector1, cross_product(vector2, vector3))));
    println!("Vector From vertex1 to vertex2: {:?}", get_vector_from_vertex(vertex1, vertex2));
}

fn addition(src: [f32; 4], dst: [f32; 4]) -> [f32; 4] {
    let mut temp: [f32; 4] = [0.0; 4];
    for i in 0..4
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

fn determinant(matrix: [[f32; 2]; 2]) -> f32 {
    let mut temp: f32 = matrix[0][0] * matrix[1][1];
    temp -= matrix[1][0] * matrix[0][1];
    return temp;
}

fn cross_product(vec1: [f32; 4], vec2: [f32; 4]) -> [f32; 4] {
    let mut temp: [[f32; 2]; 2] = [[vec1[1], vec1[2]], [vec2[1], vec2[2]]];
    let mut res: [f32; 4] = [0.0; 4];
    res[0] = determinant(temp);
    temp = [[vec1[0], vec1[2]], [vec2[0], vec2[2]]];
    res[1] = -1.0 * determinant(temp);
    temp = [[vec1[0], vec1[1]], [vec2[0], vec2[1]]];
    res[2] = determinant(temp);
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