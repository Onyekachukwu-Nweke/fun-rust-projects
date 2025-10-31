use wordie::{GapBuffer, Editor, CursorPos};

#[test]
fn insert_and_text() {
    let mut gb = GapBuffer::with_capacity(8);
    gb.insert_str("Hello");
    gb.insert_char(' ');
    gb.insert_str("World");
    assert_eq!(gb.text(), "Hello World");
}

#[test]
fn move_and_insert() {
    let mut gb = GapBuffer::from_str("HelloWorld");
    // move cursor left 5 steps to between "Hello|World"
    for _ in 0..5 { gb.move_left(); }
    gb.insert_char(' ');
    assert_eq!(gb.text(), "Hello World");
}

#[test]
fn delete_and_backspace() {
    let mut gb = GapBuffer::from_str("abXc");
    gb.move_left();
    gb.move_left(); // cursor between 'b' and 'X'
    gb.delete();    // delete 'X'
    assert_eq!(gb.text(), "abc");
    gb.backspace(); // backspace 'b'
    assert_eq!(gb.text(), "ac");
}


#[test]
fn lines_and_cursor() {
    let mut ed = Editor::from_str("Hello\nWorld");
    // place cursor at end
    // move left 5 to be after '\n'
    for _ in 0..5 { ed.move_left(); }
    let pos = ed.cursor_pos();
    assert_eq!(pos, CursorPos { line: 1, col: 0 });
    assert_eq!(ed.line(0).as_deref(), Some("Hello"));
    assert_eq!(ed.line(1).as_deref(), Some("World"));
}
