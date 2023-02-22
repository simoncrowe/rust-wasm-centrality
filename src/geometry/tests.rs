use super::*;

#[test]
fn test_layout_to_clipspace_square_from_origin() {
    let layout_location = Vector2::new(2.0, 3.5);
    let display_offset = Vector2::new(0.0, 0.0);
    let display_scale = 0.25;
    let aspect_ratio = 1.0;

    let resulting_location =
        layout_to_clipspace(layout_location, display_offset, display_scale, aspect_ratio);

    assert_eq!(resulting_location, Vector2::new(0.5, 0.875))
}

#[test]
fn test_layout_to_clipspace_square_with_offset() {
    let layout_location = Vector2::new(10.0, 8.0);
    let display_offset = Vector2::new(7.0, 5.5);
    let display_scale = 0.5;
    let aspect_ratio = 1.0;

    let resulting_location =
        layout_to_clipspace(layout_location, display_offset, display_scale, aspect_ratio);

    assert_eq!(resulting_location, Vector2::new(1.5, 1.25))
}

#[test]
fn test_layout_to_clipspace_square_with_offset_and_aspect_ratio() {
    let layout_location = Vector2::new(3.0, 4.0);
    let display_offset = Vector2::new(3.5, 4.5);
    let display_scale = 1.0;
    let aspect_ratio = 1.6;

    let resulting_location =
        layout_to_clipspace(layout_location, display_offset, display_scale, aspect_ratio);

    assert_eq!(resulting_location, Vector2::new(-0.3125, -0.5))
}

#[test]
fn test_rect_contains_point_inside() {
    let bottom_left = Vector2::new(0.0, 0.0);
    let top_right = Vector2::new(5.5, 4.5);
    let rect = Rect::new(bottom_left, top_right);

    let point_inside = Vector2::new(1.0, 1.0);

    assert!(rect.contains(point_inside));
}

#[test]
fn test_rect_contains_point_low_left() {
    let bottom_left = Vector2::new(0.0, 0.0);
    let top_right = Vector2::new(5.5, 4.5);
    let rect = Rect::new(bottom_left, top_right);

    let point_low_left = Vector2::new(-1.0, -1.0);

    assert!(!rect.contains(point_low_left));
}

#[test]
fn test_rect_contains_point_low() {
    let bottom_left = Vector2::new(0.0, 0.0);
    let top_right = Vector2::new(5.5, 4.5);
    let rect = Rect::new(bottom_left, top_right);

    let point_low = Vector2::new(3.0, -1.0);

    assert!(!rect.contains(point_low));
}

#[test]
fn test_rect_contains_point_low_right() {
    let bottom_left = Vector2::new(0.0, 0.0);
    let top_right = Vector2::new(5.5, 4.5);
    let rect = Rect::new(bottom_left, top_right);

    let point_low_right = Vector2::new(5.7, -1.0);

    assert!(!rect.contains(point_low_right));
}

#[test]
fn test_rect_contains_point_right() {
    let bottom_left = Vector2::new(0.0, 0.0);
    let top_right = Vector2::new(5.5, 4.5);
    let rect = Rect::new(bottom_left, top_right);

    let point_right = Vector2::new(5.7, 1.0);

    assert!(!rect.contains(point_right));
}

#[test]
fn test_rect_contains_point_high_right() {
    let bottom_left = Vector2::new(0.0, 0.0);
    let top_right = Vector2::new(5.5, 4.5);
    let rect = Rect::new(bottom_left, top_right);

    let point_high_right = Vector2::new(7.1, 5.0);

    assert!(!rect.contains(point_high_right));
}

#[test]
fn test_rect_contains_point_high() {
    let bottom_left = Vector2::new(0.0, 0.0);
    let top_right = Vector2::new(5.5, 4.5);
    let rect = Rect::new(bottom_left, top_right);

    let point_high = Vector2::new(2.7, 5.0);

    assert!(!rect.contains(point_high));
}

#[test]
fn test_rect_contains_point_high_left() {
    let bottom_left = Vector2::new(0.0, 0.0);
    let top_right = Vector2::new(5.5, 4.5);
    let rect = Rect::new(bottom_left, top_right);

    let point_high_right = Vector2::new(-2.3, 5.1);

    assert!(!rect.contains(point_high_right));
}

#[test]
fn test_rect_contains_point_left() {
    let bottom_left = Vector2::new(0.0, 0.0);
    let top_right = Vector2::new(5.5, 4.5);
    let rect = Rect::new(bottom_left, top_right);

    let point_left = Vector2::new(-2.3, 2.5);

    assert!(!rect.contains(point_left));
}
