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
    let test1: [f32;3] = [1.0, 5.0, 2.0];
    let test2: [f32;3] = [2.0, 3.0, 1.0];
    let test3: [f32;3] = [2.0, 0.0, 0.0];
    println!("Vector 1: {:?}", test1);
    println!("Vector 2: {:?}", test2);
    println!("Size of Vector 1: {:?}", f32::sqrt(dotProduct(test1, test1)));
    println!("Size of Vector 2: {:?}", f32::sqrt(dotProduct(test2, test2)));
    println!("Size of Vector 3: {:?}", f32::sqrt(dotProduct(test3, test3)));
    println!("Dot Product of Vector1 and Vector2: {:?}", dotProduct(test1, test2));
    println!("Add Vector1 and Vector2: {:?}", addition(test1, test2));
    println!("Subtract Vector1 and Vector2: {:?}", substraction(test1, test2));
    println!("Cross Product of Vector1 and Vector2: {:?}", crossProduct(test1, test2));
}

fn addition(src: [f32; 3], dst: [f32; 3]) -> [f32; 3]
{
    let mut temp: [f32; 3] = [0.0; 3];
    for i in 0..3
    {
        temp[i] = src[i] + dst[i];
    }
    return temp;
}
fn substraction(src: [f32; 3], dst: [f32; 3]) -> [f32; 3]
{
    let mut temp: [f32; 3] = [0.0; 3];
    for i in 0..3
    {
        temp[i] = src[i] - dst[i];
    }
    return temp;
}

fn dotProduct(src: [f32; 3], dst: [f32; 3]) -> f32
{
    let mut temp: f32 = 0.0;
    for i in 0..3
    {
        temp += src[i] * dst[i];
    }
    return temp;
}

fn determinant(matrix: [[f32; 2]; 2]) -> f32
{
    let mut temp: f32 = matrix[0][0] * matrix[1][1];
    temp -= matrix[1][0] * matrix[0][1];
    return temp;
}

fn crossProduct(vec1: [f32; 3], vec2: [f32; 3]) -> [f32; 3]
{
    let mut temp: [[f32; 2]; 2] = [[vec1[1], vec1[2]], [vec2[1], vec2[2]]];
    let mut res: [f32; 3] = [0.0; 3];
    res[0] = determinant(temp);
    temp = [[vec1[0], vec1[2]], [vec2[0], vec2[2]]];
    res[1] = -1.0 * determinant(temp);
    temp = [[vec1[0], vec1[1]], [vec2[0], vec2[1]]];
    res[2] = determinant(temp);
    return res;
}