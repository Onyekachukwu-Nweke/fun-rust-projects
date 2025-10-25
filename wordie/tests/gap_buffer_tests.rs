use wordie::GapBuffer;

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
    // place cursor before 'X' (already there at end; move left 1 to be before 'X')
    gb.move_left(); // cursor between 'b' and 'X'
    gb.delete();    // delete 'X'
    assert_eq!(gb.text(), "abc");
    gb.backspace(); // backspace 'b'
    assert_eq!(gb.text(), "ac");
}
