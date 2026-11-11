use super::Vector3;

fn test_vector3() -> Vector3 {
    Vector3 { x: 1.0, y: 2.0, z: 3.0 }
}

#[test]
fn test_vector3_neg() {
    let result = -Vector3::RIGHT;
    assert_eq!(result, Vector3::LEFT)
}

#[test]
fn test_vector3_add() {
    let result = test_vector3() + test_vector3();
    assert_eq!(result, Vector3 { x: 2.0, y: 4.0, z: 6.0 })
}

#[test]
fn test_vector3_add_assign() {
    let mut left = test_vector3();
    left += test_vector3();
    assert_eq!(left, Vector3 { x: 2.0, y: 4.0, z: 6.0 })
}

#[test]
fn test_vector3_sub() {
    let result = test_vector3() - Vector3::ONE;
    assert_eq!(result, Vector3 { x: 0.0, y: 1.0, z: 2.0 })
}

#[test]
fn test_vector3_sub_assign() {
    let mut left = test_vector3();
    left -= Vector3::ONE;
    assert_eq!(left, Vector3 { x: 0.0, y: 1.0, z: 2.0 })
}

#[test]
fn test_vector3_mul() {
    let result = test_vector3() * test_vector3();
    assert_eq!(result, Vector3 { x: 1.0, y: 4.0, z: 9.0 })
}

#[test]
fn test_vector3_mul_assign() {
    let mut left = test_vector3();
    left *= test_vector3();
    assert_eq!(left, Vector3 { x: 1.0, y: 4.0, z: 9.0 })
}

#[test]
fn test_vector3_mul_f64() {
    let left = test_vector3();
    let right: f64 = 2.0;

    assert_eq!(right * left, Vector3 { x: 2.0, y: 4.0, z: 6.0 })
}

#[test]
fn test_vector3_mul_f64_2() {
    let left = test_vector3();
    let right: f64 = 2.0;

    assert_eq!(left * right, Vector3 { x: 2.0, y: 4.0, z: 6.0 })
}

#[test]
fn test_vector3_div() {
    let result = test_vector3() / Vector3::ONE;
    assert_eq!(result, test_vector3())
}

#[test]
fn test_vector3_div_assign() {
    let mut left = test_vector3();
    left /= Vector3::ONE;
    assert_eq!(left, test_vector3())
}

#[test]
fn test_vector3_div_f64() {
    let left = test_vector3();
    let right: f64 = 2.0;

    assert_eq!(left / right, Vector3 { x: 0.5, y: 1.0, z: 1.5 })
}