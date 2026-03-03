use compier::lexer::Lexer;
use std::io::{self, BufRead, Write};

pub struct Lexi<R, W> {
    input: R,
    output: W,
    lexer: Lexer,
}

impl<R, W> Lexi<R, W>
where
    R: BufRead,
    W: Write,
{
    pub fn new(input: R, output: W) -> Self {
        Self {
            input,
            output,
            lexer: Lexer::default(),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut line = String::new();

        loop {
            self.print_prompt()?;

            line.clear();
            let bytes = self.input.read_line(&mut line)?;
            if bytes == 0 {
                // eof
                return Ok(());
            }

            self.handle_line(&line)?;
        }
    }

    fn print_prompt(&mut self) -> io::Result<()> {
        write!(self.output, "{} ", ">")?;
        self.output.flush()
    }

    fn handle_line(&mut self, line: &str) -> io::Result<()> {
        self.lexer.push_line(line);
        self.dump_tokens()
    }

    fn dump_tokens(&mut self) -> io::Result<()> {
        for item in self.lexer.by_ref() {
            match item {
                Ok(tok) => writeln!(self.output, "{:?} {:?}", tok.kind, tok.span)?,
                Err(e) => {
                    writeln!(self.output, "{:?}", e)?;
                }
            }
        }
        Ok(())
    }
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut lexi = Lexi::new(stdin.lock(), stdout.lock());
    if let Err(e) = lexi.run() {
        eprintln!("lexi error: {e}");
    }
}
