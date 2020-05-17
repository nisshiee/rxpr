extern crate clipboard;
extern crate nom;
extern crate termion;

use std::io;
use std::io::{stdin, stdout, Write};

use clipboard::{ClipboardContext, ClipboardProvider};

use termion::cursor::DetectCursorPos;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

use crate::state::State;

mod core;
mod state;

fn update<W: Write>(t: &mut RawTerminal<W>, state: &State) -> io::Result<()> {
    let expr = state.expr();

    write!(t, "{}", termion::cursor::Goto(1, 1))?;
    write!(t, "expr> {}", &expr[..state.cursor()])?;
    write!(t, "{}", termion::cursor::Save)?;
    write!(t, "{}", &expr[state.cursor()..])?;
    write!(t, "{}", termion::clear::UntilNewline)?;

    let (_, y) = t.cursor_pos()?;
    write!(t, "{}", termion::cursor::Goto(1, y + 1))?;
    write!(t, "{}", state.last_result().unwrap_or(0))?;
    write!(t, "{}", termion::clear::UntilNewline)?;

    write!(t, "{}", termion::cursor::Restore)?;
    t.flush()
}

fn copy_to_clipboard(state: &State) -> Option<()> {
    let res = state.last_result()?;
    let res = format!("{}", res);
    let mut ctx: ClipboardContext = ClipboardProvider::new().ok()?;
    ctx.set_contents(res).ok()
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", termion::clear::All).unwrap();

    let mut state = State::new();
    update(&mut stdout, &state).unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('c') => break,
            Key::Char(d @ '0'..='9')
            | Key::Char(d @ ' ')
            | Key::Char(d @ '+')
            | Key::Char(d @ '-')
            | Key::Char(d @ '*')
            | Key::Char(d @ '/')
            | Key::Char(d @ '(')
            | Key::Char(d @ ')') => state.input(d),
            Key::Left | Key::Ctrl('b') => state.cursor_left(),
            Key::Right | Key::Ctrl('f') => state.cursor_right(),
            Key::Up | Key::Ctrl('a') => state.cursor_first(),
            Key::Down | Key::Ctrl('e') => state.cursor_last(),
            Key::Backspace | Key::Ctrl('h') => state.backspace(),
            Key::Delete | Key::Ctrl('d') => state.delete(),
            Key::Ctrl('u') => state.clear(),
            Key::Char('\n') => {
                copy_to_clipboard(&state).unwrap_or(()); // ignore clipboard error
                break;
            }
            _ => {}
        }
        update(&mut stdout, &state).unwrap();
    }

    let (_, y) = stdout.cursor_pos().unwrap();
    write!(stdout, "{}", termion::cursor::Goto(1, y + 2)).unwrap();
    stdout.flush().unwrap();
}
