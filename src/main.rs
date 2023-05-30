use ariadne::{sources, Label, Report, ReportKind};
use boja::Command;
use chumsky::prelude::Simple;

fn main() {
    let cfg = Command::init();
    let input = cfg.input();

    let res = boja::parse_color(&input);

    match res {
        Ok(col) => {
            dbg!(col);
        }
        Err(errs) => errs.into_iter().for_each(|err| pretty_print(&input, err)),
    };
}

fn pretty_print(input: &str, error: Simple<char>) {
    let msg = error.to_string();

    Report::<(&str, _)>::build(ReportKind::Error, "stdin", error.span().start)
        .with_message(error.label().unwrap_or("Unexpected input"))
        .with_label(Label::new(("stdin", error.span())).with_message(msg))
        .with_help("run 'boja --help' for usage instructions")
        .finish()
        .eprint(sources([("stdin", input)]))
        .unwrap();
}
