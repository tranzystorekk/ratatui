/// Assert that two buffers are equal by comparing their areas and content.
///
/// On panic, displays the areas or the content and a diff of the contents.
#[deprecated = "use actual.assert_eq(&expected)"]
#[macro_export]
macro_rules! assert_buffer_eq {
    ($actual_expr:expr, $expected_expr:expr) => {
        match (&$actual_expr, &$expected_expr) {
            (actual, expected) => actual.assert_eq(expected),
        }
    };
}

#[allow(deprecated)]
#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn assert_buffer_eq_does_not_panic_on_equal_buffers() {
        let buffer = Buffer::empty(Rect::new(0, 0, 5, 1));
        let other_buffer = Buffer::empty(Rect::new(0, 0, 5, 1));
        assert_buffer_eq!(buffer, other_buffer);
    }

    #[should_panic = "buffer areas not equal"]
    #[test]
    fn assert_buffer_eq_panics_on_unequal_area() {
        let buffer = Buffer::empty(Rect::new(0, 0, 5, 1));
        let other_buffer = Buffer::empty(Rect::new(0, 0, 6, 1));
        assert_buffer_eq!(buffer, other_buffer);
    }

    #[should_panic = "buffer contents not equal"]
    #[test]
    fn assert_buffer_eq_panics_on_unequal_style() {
        let buffer = Buffer::empty(Rect::new(0, 0, 5, 1));
        let mut other_buffer = Buffer::empty(Rect::new(0, 0, 5, 1));
        other_buffer.set_string(0, 0, " ", Style::default().fg(Color::Red));
        assert_buffer_eq!(buffer, other_buffer);
    }
}
