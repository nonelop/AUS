use super::Matrix3;
use super::Vector3;

fn test_matrix3() -> Matrix3 {
    Matrix3 { 
        c1: Vector3 { x: 1.0, y: 2.0, z: 3.0 }, 
        c2: Vector3 { x: 4.0, y: 5.0, z: 6.0 }, 
        c3: Vector3 { x: 7.0, y: 8.0, z: 9.0 }
    }
}

#[test]
fn test_matrix_neg() {
    let matrix = test_matrix3();
    let expected = Matrix3 { 
        c1: Vector3 { x: -1.0, y: -2.0, z: -3.0 }, 
        c2: Vector3 { x: -4.0, y: -5.0, z: -6.0 }, 
        c3: Vector3 { x: -7.0, y: -8.0, z: -9.0 }
    };

    assert_eq!(-matrix, expected)
}

#[test]
fn test_matrix_add() {
    let m1 = test_matrix3();
    let m2 = test_matrix3();

    let expected = Matrix3 { 
        c1: Vector3 { x: 2.0, y: 4.0, z: 6.0 }, 
        c2: Vector3 { x: 8.0, y: 10.0, z: 12.0 }, 
        c3: Vector3 { x: 14.0, y: 16.0, z: 18.0 }
    };

    assert_eq!(m1 + m2, expected)
}

#[test]
fn test_matrix_add_assign() {
    let mut m1 = test_matrix3();
    let m2 = test_matrix3();

    let expected = Matrix3 { 
        c1: Vector3 { x: 2.0, y: 4.0, z: 6.0 }, 
        c2: Vector3 { x: 8.0, y: 10.0, z: 12.0 }, 
        c3: Vector3 { x: 14.0, y: 16.0, z: 18.0 }
    };

    m1 += m2;
    assert_eq!(m1, expected)
}

#[test]
fn test_matrix_sub() {
    let m1 = test_matrix3();
    let m2 = test_matrix3();

    let expected = Matrix3::ZERO;

    assert_eq!(m1 - m2, expected)
}

#[test]
fn test_matrix_sub_assign() {
    let mut m1 = test_matrix3();
    let m2 = test_matrix3();

    let expected = Matrix3::ZERO;

    m1 -= m2;
    assert_eq!(m1, expected)
}

#[test]
fn test_matrix_mul_f64() {
    let matrix = test_matrix3();
    let scalar = 2.0;

    let expected = Matrix3 { 
        c1: Vector3 { x: 2.0, y: 4.0, z: 6.0 }, 
        c2: Vector3 { x: 8.0, y: 10.0, z: 12.0 }, 
        c3: Vector3 { x: 14.0, y: 16.0, z: 18.0 }
    };

    assert_eq!(matrix * scalar, expected)
}

#[test]
fn test_matrix_mulf64_2() {
    let matrix = test_matrix3();
    let scalar = 2.0;

    let expected = Matrix3 { 
        c1: Vector3 { x: 2.0, y: 4.0, z: 6.0 }, 
        c2: Vector3 { x: 8.0, y: 10.0, z: 12.0 }, 
        c3: Vector3 { x: 14.0, y: 16.0, z: 18.0 }
    };

    assert_eq!(scalar * matrix, expected)
}

#[test]
fn test_matrix_mul_assign_f64() {
    let mut matrix = test_matrix3();
    let scalar = 2.0;

    let expected = Matrix3 { 
        c1: Vector3 { x: 2.0, y: 4.0, z: 6.0 }, 
        c2: Vector3 { x: 8.0, y: 10.0, z: 12.0 }, 
        c3: Vector3 { x: 14.0, y: 16.0, z: 18.0 }
    };

    matrix *= scalar;
    assert_eq!(matrix, expected)
}