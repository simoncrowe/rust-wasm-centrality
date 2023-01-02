use super::*;

#[test]
fn test_layout_to_display_square_from_origin() {
    let layout_location = vec![2.0, 3.5];
    let display_offset = vec![0.0, 0.0];
    let display_scale = 4.0;
    let aspect_ratio = 1.0;

    let resulting_location = layout_to_display(
        &layout_location,
        &display_offset,
        &display_scale,
        &aspect_ratio,
    );

    assert_eq!(resulting_location, vec![8.0, 14.0])
}

#[test]
fn test_rect_contains_point_inside() {
    let bottom_left = vec![0.0, 0.0];
    let top_right = vec![5.5, 4.5];
    let rect = Rect::new(bottom_left, top_right);

    let point_inside = vec![1.0, 1.0];

    assert!(rect.contains(&point_inside));
}

#[test]
fn test_rect_contains_point_low_left() {
    let bottom_left = vec![0.0, 0.0];
    let top_right = vec![5.5, 4.5];
    let rect = Rect::new(bottom_left, top_right);

    let point_low_left = vec![-1.0, -1.0];

    assert!(!rect.contains(&point_low_left));
}

#[test]
fn test_rect_contains_point_low() {
    let bottom_left = vec![0.0, 0.0];
    let top_right = vec![5.5, 4.5];
    let rect = Rect::new(bottom_left, top_right);

    let point_low = vec![3.0, -1.0];

    assert!(!rect.contains(&point_low));
}

#[test]
fn test_rect_contains_point_low_right() {
    let bottom_left = vec![0.0, 0.0];
    let top_right = vec![5.5, 4.5];
    let rect = Rect::new(bottom_left, top_right);

    let point_low_right = vec![5.7, -1.0];

    assert!(!rect.contains(&point_low_right));
}

#[test]
fn test_rect_contains_point_right() {
    let bottom_left = vec![0.0, 0.0];
    let top_right = vec![5.5, 4.5];
    let rect = Rect::new(bottom_left, top_right);

    let point_right = vec![5.7, 1.0];

    assert!(!rect.contains(&point_right));
}

#[test]
fn test_rect_contains_point_high_right() {
    let bottom_left = vec![0.0, 0.0];
    let top_right = vec![5.5, 4.5];
    let rect = Rect::new(bottom_left, top_right);

    let point_high_right = vec![7.1, 5.0];

    assert!(!rect.contains(&point_high_right));
}

#[test]
fn test_rect_contains_point_high() {
    let bottom_left = vec![0.0, 0.0];
    let top_right = vec![5.5, 4.5];
    let rect = Rect::new(bottom_left, top_right);

    let point_high = vec![2.7, 5.0];

    assert!(!rect.contains(&point_high));
}

#[test]
fn test_rect_contains_point_high_left() {
    let bottom_left = vec![0.0, 0.0];
    let top_right = vec![5.5, 4.5];
    let rect = Rect::new(bottom_left, top_right);

    let point_high_right = vec![-2.3, 5.1];

    assert!(!rect.contains(&point_high_right));
}

#[test]
fn test_rect_contains_point_left() {
    let bottom_left = vec![0.0, 0.0];
    let top_right = vec![5.5, 4.5];
    let rect = Rect::new(bottom_left, top_right);

    let point_left = vec![-2.3, 2.5];

    assert!(!rect.contains(&point_left));
}
