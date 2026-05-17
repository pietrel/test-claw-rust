use console::{Key, Term, style};
use std::io;

#[derive(Debug)]
pub enum Consent {
    Yes,
    No,
    Reason(String),
}

pub fn ask(prompt: &str) -> io::Result<Option<Consent>> {
    let options = ["Yes", "No", "No, do this instead"];
    let mut cursor: usize = 0;

    let term = Term::stderr();
    term.hide_cursor()?;

    term.write_line("")?;
    render(&term, prompt, &options, cursor)?;

    let selection = loop {
        match term.read_key()? {
            Key::Char('1') => break 0,
            Key::Char('2') => break 1,
            Key::Char('3') => break 2,

            Key::ArrowUp | Key::Char('k') => {
                cursor = if cursor == 0 {
                    options.len() - 1
                } else {
                    cursor - 1
                };
            }
            Key::ArrowDown | Key::Char('j') => {
                cursor = (cursor + 1) % options.len();
            }

            Key::Enter | Key::Char(' ') => break cursor,

            Key::Escape | Key::Char('q') => {
                clear(&term, &options)?;
                term.show_cursor()?;
                return Ok(None);
            }

            _ => continue,
        }

        clear(&term, &options)?;
        render(&term, prompt, &options, cursor)?;
    };

    clear(&term, &options)?;
    term.clear_last_lines(1)?;
    term.show_cursor()?;

    let result = match selection {
        0 => Consent::Yes,
        1 => Consent::No,
        2 => {
            term.write_line(&format!("\n{}", style(format!("? {}", prompt)).bold()))?;
            term.write_str(&format!("{} ", style("No, do this instead:").yellow()))?;
            let reason = term.read_line()?;
            Consent::Reason(reason)
        }
        _ => unreachable!(),
    };

    Ok(Some(result))
}

fn render(term: &Term, prompt: &str, options: &[&str], cursor: usize) -> io::Result<()> {
    term.write_line(&format!("{}", style(format!("? {}", prompt)).bold()))?;

    for (i, option) in options.iter().enumerate() {
        let shortcut = format!("[{}]", i + 1);
        if i == cursor {
            term.write_line(&format!(
                "  {} {} {}",
                style("›").green().bold(),
                style(&shortcut).dim(),
                style(option).green().bold(),
            ))?;
        } else {
            term.write_line(&format!("    {} {}", style(&shortcut).dim(), option,))?;
        }
    }

    term.write_line(&format!(
        "\n  {}",
        style("↑↓/jk navigate · 1-3 jump · enter confirm · q quit").dim()
    ))?;

    Ok(())
}

fn clear(term: &Term, options: &[&str]) -> io::Result<()> {
    term.clear_last_lines(options.len() + 3)
}
