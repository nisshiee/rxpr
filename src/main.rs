extern crate clipboard;
extern crate nom;
extern crate structopt;
extern crate termion;

use std::io;
use std::io::{stdin, stdout, Write};

use clipboard::{ClipboardContext, ClipboardProvider};

use structopt::StructOpt;

use termion::cursor::DetectCursorPos;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

use crate::core::{parse, Calculatable, Num as NumCore};
use crate::state::State;

mod core;
mod state;

trait Num: NumCore {
    fn available_chars() -> &'static str;
}

impl Num for i64 {
    fn available_chars() -> &'static str {
        "0123456789+-*/() "
    }
}
impl Num for f64 {
    fn available_chars() -> &'static str {
        "0123456789+-*/(). "
    }
}

fn update<W: Write, N: Num>(t: &mut RawTerminal<W>, state: &State<N>) -> io::Result<()> {
    let expr = state.expr();

    write!(t, "{}", termion::cursor::Goto(1, 1))?;
    write!(t, "expr> {}", &expr[..state.cursor()])?;
    write!(t, "{}", termion::cursor::Save)?;
    write!(t, "{}", &expr[state.cursor()..])?;
    write!(t, "{}", termion::clear::UntilNewline)?;

    let (_, y) = t.cursor_pos()?;
    write!(t, "{}", termion::cursor::Goto(1, y + 1))?;

    if !state.valid() {
        write!(t, "{}", termion::color::Fg(termion::color::LightRed))?;
    }
    if let Some(last_result) = state.last_result() {
        write!(t, "{}", last_result)?;
    } else {
        write!(t, "-")?;
    }
    write!(t, "{}", termion::color::Fg(termion::color::Reset))?;
    write!(t, "{}", termion::clear::UntilNewline)?;

    write!(t, "{}", termion::cursor::Restore)?;
    t.flush()
}

fn copy_to_clipboard<N: Num>(state: &State<N>) -> Option<()> {
    let res = state.last_result()?;
    let res = format!("{}", res);
    let mut ctx: ClipboardContext = ClipboardProvider::new().ok()?;
    ctx.set_contents(res).ok()
}

#[derive(StructOpt, Debug)]
#[structopt(name = "rxpr")]
struct Opt {
    /// 64bit integer mode (default: 64bit float mode)
    #[structopt(short, long)]
    i64: bool,
}

fn main() {
    let opt = Opt::from_args();

    if opt.i64 {
        run::<i64>()
    } else {
        run::<f64>()
    }
}

fn run<N: Num>() {
    if termion::is_tty(&stdin()) && termion::is_tty(&stdout()) {
        run_cli::<N>();
    } else {
        run_io::<N>();
    }
}

fn run_cli<N: Num>() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", termion::clear::All).unwrap();

    let mut state = State::<N>::new();
    update(&mut stdout, &state).unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('c') => break,
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
            Key::Char(c) => {
                if N::available_chars().contains(c) {
                    state.input(c)
                }
            }
            _ => {}
        }
        update(&mut stdout, &state).unwrap();
    }

    let (_, y) = stdout.cursor_pos().unwrap();
    write!(stdout, "{}", termion::cursor::Goto(1, y + 2)).unwrap();
    stdout.flush().unwrap();
}

fn run_io<N: Num>() {
    let mut buf = String::new();
    while let Ok(n) = stdin().read_line(&mut buf) {
        if n == 0 {
            break;
        }
        let expr = buf.trim();
        if let Ok((_, expr)) = parse::<N>(expr) {
            println!("{}", expr.calc());
        } else {
            println!();
        }
        buf.clear();
    }
}
