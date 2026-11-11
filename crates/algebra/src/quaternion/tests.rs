use super::Quaternion;

fn test_quaternion() -> Quaternion {
    Quaternion { w: 1.0, x: 2.0, y: 3.0, z: 4.0 }
}

#[test]
fn test_quaternion_neg() {
    let result = -Quaternion::IDENTITY;
    assert_eq!(result, Quaternion { w: -1.0, x: 0.0, y: 0.0, z: 0.0 })
}

#[test]
fn test_quaternion_add() {
    let result = test_quaternion() + test_quaternion();
    assert_eq!(result, Quaternion { w: 2.0, x: 4.0, y: 6.0, z: 8.0 })
}

#[test]
fn test_quaternion_add_assign() {
    let mut left = test_quaternion();
    left += test_quaternion();
    assert_eq!(left, Quaternion { w: 2.0, x: 4.0, y: 6.0, z: 8.0 })
}

#[test]
fn test_quaternion_sub() {
    let result = test_quaternion() - Quaternion::IDENTITY;
    assert_eq!(result, Quaternion { w: 0.0, x: 2.0, y: 3.0, z: 4.0 })
}

#[test]
fn test_quaternion_sub_assign() {
    let mut left = test_quaternion();
    left -= Quaternion::IDENTITY;
    assert_eq!(left, Quaternion { w: 0.0, x: 2.0, y: 3.0, z: 4.0 })
}

#[test]
fn test_quaternion_mul() {
    let result = test_quaternion() * test_quaternion();
    assert_eq!(result, Quaternion { w: -28.0, x: 4.0, y: 6.0, z: 8.0 })
}

#[test]
fn test_quaternion_mul_assign() {
    let mut left = test_quaternion();
    left *= test_quaternion();
    assert_eq!(left, Quaternion { w: -28.0, x: 4.0, y: 6.0, z: 8.0 })
}