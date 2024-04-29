fn main()
{
    let vector1: Vec<f32> = vec![1.0, 0.0, 2.0, 0.0];
    let vector2: Vec<f32> = vec![-1.0, 3.0, -2.0, 0.0];
    let vector3: Vec<f32> = vec![-1.0, 3.0, -4.0, 0.0];
    let vertex1: Vec<f32> = vec![5.0, 1.0, 3.0, 1.0];
    let vertex2: Vec<f32> = vec![3.0, -2.0, 4.0, 1.0];
    let mut point1: Vec<Vec<f32>> = get_identity(4);
    point1 = multiply_matrix(&homogeneous_get_translation(2.0, 2.0, 2.0), &point1);
    let mut point2: Vec<Vec<f32>> = get_identity(4);
    point2 = multiply_matrix(&homogeneous_get_translation(1.0, 1.0, 1.0), &point2);


    let modelPoint1: Vec<Vec<f32>> = vec![
    vec![1.0, 0.0, 0.0, 0.0],
    vec![0.0, 1.0, 0.0, 0.0],
    vec![0.0, 0.0, 1.0, 0.0],
    vec![0.0, 0.0, 0.0, 1.0]];
    let modelPoint2: Vec<Vec<f32>> = vec![
    vec![1.0, 0.0, 0.0, 1.0],
    vec![0.0, 1.0, 0.0, 0.0],
    vec![0.0, 0.0, 1.0, 0.0],
    vec![0.0, 0.0, 0.0, 1.0]];
    let modelPoint3: Vec<Vec<f32>> = vec![
    vec![1.0, 0.0, 0.0, 0.0],
    vec![0.0, 1.0, 0.0, 1.0],
    vec![0.0, 0.0, 1.0, 0.0],
    vec![0.0, 0.0, 0.0, 1.0]];
    let modelPoint4: Vec<Vec<f32>> = vec![
    vec![1.0, 0.0, 0.0, 0.0],
    vec![0.0, 1.0, 0.0, 0.0],
    vec![0.0, 0.0, 1.0, 1.0],
    vec![0.0, 0.0, 0.0, 1.0]];
    let modelPoint5: Vec<Vec<f32>> = vec![
    vec![1.0, 0.0, 0.0, 1.0],
    vec![0.0, 1.0, 0.0, 1.0],
    vec![0.0, 0.0, 1.0, 0.0],
    vec![0.0, 0.0, 0.0, 1.0]];
    let modelPoint6: Vec<Vec<f32>> = vec![
    vec![1.0, 0.0, 0.0, 1.0],
    vec![0.0, 1.0, 0.0, 0.0],
    vec![0.0, 0.0, 1.0, 1.0],
    vec![0.0, 0.0, 0.0, 1.0]];
    let modelPoint7: Vec<Vec<f32>> = vec![
    vec![1.0, 0.0, 0.0, 0.0],
    vec![0.0, 1.0, 0.0, 1.0],
    vec![0.0, 0.0, 1.0, 1.0],
    vec![0.0, 0.0, 0.0, 1.0]];
    let modelPoint8: Vec<Vec<f32>> = vec![
    vec![1.0, 0.0, 0.0, 1.0],
    vec![0.0, 1.0, 0.0, 1.0],
    vec![0.0, 0.0, 1.0, 1.0],
    vec![0.0, 0.0, 0.0, 1.0]];
    let modelTranslation1: Vec<Vec<f32>> = homogeneous_get_translation(1.0, 3.0, 2.0);
    let modelRotation1: Vec<Vec<f32>> = homogeneous_get_rotation('X', 45.0f32.to_radians());
    let mut model = Model {
        group: vec![modelPoint1, modelPoint2, modelPoint3, modelPoint4, modelPoint5, modelPoint6, modelPoint7, modelPoint8],
        stack: vec![modelRotation1, modelTranslation1],
    };
    calculate_final_transform(&mut model);

    println!("Vector 1: {:?}", vector1);
    println!("Vector 2: {:?}", vector2);
    println!("Vector 3: {:?}", vector3);
    println!("Vertex 1: {:?}", vertex1);
    println!("Vertex 2: {:?}", vertex2);
    println!("Point 1:");
    for i in &point1 {println!("\t{:?}", i);}
    println!("Point 2:");
    for i in &point2 {println!("\t{:?}", i);}
    let point3: Vec<Vec<f32>> = multiply_matrix(&point1, &point2);
    println!("multiply Point 1 and Point 2:");
    for i in &point3 {println!("\t{:?}", i);}
    let degree = 90.0f32;
    let point3 = multiply_matrix(&homogeneous_get_rotation('X', degree.to_radians()), &point1);
    println!("point 1 after rotate 90degree by x axis: ");
    for i in &point3 {println!("\t{:?}", i);}
    println!("Size of Vector 1: {:?}", get_magnitude(&vector1));
    println!("Size of Vector 2: {:?}", get_magnitude(&vector2));
    println!("Size of Vector 3: {:?}", get_magnitude(&vector3));
    println!("Dot Product of Vector1 and Vector2: {:?}", dot_product(&vector1, &vector2));
    println!("Add Vector1 and Vector2: {:?}", addition(&vector1, &vector2));
    println!("Subtract Vector1 and Vector2: {:?}", substraction(&vector1, &vector2));
    println!("Cross Product of Vector1 and Vector2: {:?}", cross_product(&vector1, &vector2));
    println!("Volume of Parallelepiped of 3 Vectors: {:?}", f32::abs(dot_product(&vector1, &cross_product(&vector2, &vector3))));
    println!("Vector From vertex1 to vertex2: {:?}", get_vector_from_vertex(vertex1, vertex2));
    println!("Position of point1: {:?}", homogeneous_get_position(&point1));
    println!("Determinant of Point 3: {}", determinant(&point3, 4));
    let mut reverse_matrix = get_identity(4);
    gauss_jordan(&mut point1, &mut reverse_matrix);
    println!("Reverse Matrix of point 1:");
    for i in &reverse_matrix {println!("\t{:?}", i);}
    //println!("test radian: {}, {}", 90.0f32.to_radians().cos(), 90.0f32.to_radians().sin());
}
fn print_matrix(matrix: &Vec<Vec<f32>>) {
    for i in matrix.iter() {println!("\t{:?}", i);}
}

fn is_square_matrix(matrix: &Vec<Vec<f32>>) -> bool {
    if matrix.len() == 0 {panic!("is_square_matrix: The size of matrix must be larger than zero.");}
    let n = matrix.len();
    let m = matrix[0].len();
    if n == m {return true;} else {return false;}
}

fn addition(vec1: &Vec<f32>, vec2: &Vec<f32>) -> Vec<f32> {
    if vec1.len() != vec2.len() {panic!("addition: The length of two arguments do not match.");}
    let n = vec1.len();
    let mut result: Vec<f32> = Vec::new();
    for i in 0..n
    {
        result.push(vec1[i] + vec2[i]);
    }
    return result;
}
fn substraction(vec1: &Vec<f32>, vec2: &Vec<f32>) -> Vec<f32> {
    if vec1.len() != vec2.len() {panic!("subtraction: The length of two arguments do not match.");}
    let n = vec1.len();
    let mut result: Vec<f32> = Vec::new();
    for i in 0..n
    {
        result.push(vec1[i] - vec2[i]);
    }
    return result;
}

fn dot_product(vec1: &Vec<f32>, vec2: &Vec<f32>) -> f32 {
    if vec1.len() != vec2.len() {panic!("dot_product: The length of two arguments do not match.");}
    let mut result: f32 = 0.0;
    let n = vec1.len();
    for i in 0..n
    {
        result += vec1[i] * vec2[i];
    }
    return result;
}

fn get_magnitude(vec: &Vec<f32>) -> f32 {
    return f32::sqrt(dot_product(&vec, &vec));
}

fn gauss_jordan(m: &mut Vec<Vec<f32>>, inv: &mut Vec<Vec<f32>>) {
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


fn determinant(matrix: &Vec<Vec<f32>>, n: i32) -> f32 {
    if !is_square_matrix(&matrix) {panic!("determinant: matrix must be square matrix.");}
    if matrix.len() != n as usize {panic!("determinant: matrix's size must be equal to n.");}
    let mut result: f32 = 0.0;
    if n == 1
    {
        return matrix[0][0];
    }
    if n == 2
    {
        result = matrix[0][0] * matrix[1][1];
        result -= matrix[1][0] * matrix[0][1];
        return result;
    }
    else
    {
        for i in 0..n
        {
            let mut minor: Vec<Vec<f32>> = Vec::new();
            for j in 0..n-1
            {
                minor.push(Vec::new());
                for k in 0..n-1
                {
                    minor[j as usize].push(matrix[(j+1) as usize][if k >= i {(k+1) as usize} else {k as usize}]);
                }
            }
            result += determinant(&minor, n-1);
        }
        return result;
    }
}

//TODO: rework this
fn cross_product(vec1: &Vec<f32>, vec2: &Vec<f32>) -> Vec<f32> {
    if vec1.len() != vec2.len() {panic!("cross_product: The length of two arguments do not match.");}
    let mut temp: Vec<Vec<f32>> = vec![vec![vec1[1], vec1[2]], vec![vec2[1], vec2[2]]];
    let mut res: Vec<f32> = Vec::new();
    res.push(determinant(&temp, 2));
    temp = vec![vec![vec1[0], vec1[2]], vec![vec2[0], vec2[2]]];
    res.push(-1.0 * determinant(&temp, 2));
    temp = vec![vec![vec1[0], vec1[1]], vec![vec2[0], vec2[1]]];
    res.push(determinant(&temp, 2));
    res.push(0.0);
    return res;
}

fn get_identity(size: i32) -> Vec<Vec<f32>> {
    if size <= 0 {panic!("get_identity: size must be larger than 0.");}
    let mut result: Vec<Vec<f32>> = Vec::new();
    for i in 0..size
    {
        result.push(Vec::new());
        for j in 0..size
        {
            result[i as usize].push(if i == j {1.0} else {0.0});
        }
    }
    return result;
}

// For homogeneous-transformation-matrix.
fn homogeneous_get_uniform_scale(scale: f32) -> Vec<Vec<f32>> {
    let mut result = get_identity(4);
    result[0][0] *= scale;
    result[1][1] *= scale;
    result[2][2] *= scale;
    return result;
}

// For homogeneous-transformation-matrix.
fn homogeneous_get_linear_transform(scale_x: f32, scale_y: f32, scale_z: f32) -> Vec<Vec<f32>> {
    let mut result = get_identity(4);
    result[0][0] *= scale_x;
    result[1][1] *= scale_y;
    result[2][2] *= scale_z;
    return result;
}

fn get_vector_from_vertex(src: Vec<f32>, dst: Vec<f32>) -> Vec<f32> {
    if src.len() != dst.len() {panic!("get_vector_from_vertex: The length of two arguments do not match.");}
    let temp = substraction(&dst, &src);
    return temp;
}

// For homogeneous-transformation-matrix.
fn homogeneous_flip(axis: char) -> Vec<Vec<f32>> {
    let mut temp = get_identity(4);
    match axis{
        'X'=>temp[0][0] = -1.0,
        'Y'=>temp[1][1] = -1.0,
        'Z'=>temp[2][2] = -1.0,
        _=> {},
    }
    return temp;
}

// For homogeneous-transformation-matrix.
fn homogeneous_get_position(matrix: &Vec<Vec<f32>>) -> Vec<f32>
{
    if !is_square_matrix(&matrix) {panic!("homogeneous_get_position: matrix must be square matrix.");}
    let mut result: Vec<f32> = Vec::new();
    let n = matrix.len();
    for i in 0..n
    {
        result.push(matrix[i][n-1]);
    }
    return result;
}

// For homogeneous-transformation-matrix.
fn homogeneous_get_translation(pos_x: f32, pos_y: f32, pos_z: f32) -> Vec<Vec<f32>>
{
    let mut temp = get_identity(4);
    temp[0][3] = pos_x;
    temp[1][3] = pos_y;
    temp[2][3] = pos_z;
    return temp;
}

// For homogeneous-transformation-matrix.
fn homogeneous_get_rotation(axis: char, radian: f32) -> Vec<Vec<f32>>
{
    let mut temp = get_identity(4);
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

//TODO: rework this
fn multiply_matrix(vec1: &Vec<Vec<f32>>, vec2: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    if vec1[0].len() != vec2.len() {panic!("multiply_matrix: The width of vec1 and height of vec2 must be same.");}
    let n = vec1.len();
    let mut result = get_identity(n as i32);
    let mut sum = 0.0;
    for i in 0..n
    {
        for j in 0..n
        {
            for k in 0..n
            {
                //println!("multiply {} and {}, which is at [{}, {}] and [{}, {}], as result: {}.",vec1[i][k], vec2[k][j], i, k, k, j, vec1[i][k] * vec2[k][j]);
                sum += vec1[i][k] * vec2[k][j];
            }
            result[i][j] = sum;
            sum = 0.0;
        }
    }
    return result;
}

fn calculate_final_transform(model: &mut Model)
{
    println!("Start Calculation.");
    let v_count = model.group.len();
    let s_count = model.stack.len();
    for mat in model.stack.iter_mut()
    {
        for point in model.group.iter_mut()
        {
            *point = multiply_matrix(&mat, &point);
            print_matrix(&point);
            println!();
        }
        println!("Calculate finished.");
    }
}

struct Model
{
    group: Vec<Vec<Vec<f32>>>,
    stack: Vec<Vec<Vec<f32>>>,
}