use crate::cli;
use crossterm as ct;
use crossterm::Command as _Command;
use glint::{Config, Git};
use std::io::Write as _Write;
use std::{io, iter};

// fn with_raw<R>(f: impl FnOnce(crossterm::RawScreen) -> R) -> R {
//     match crossterm::RawScreen::into_raw_mode() {
//         Err(_) => {
//             eprintln!("Failed to convert stdio to raw mode. Can't continue.");
//             std::process::exit(1);
//         }
//         Ok(raw_screen) => f(raw_screen),
//     }
// }

pub fn log(params: cli::Log, _config: Config) {
    let git = match Git::from_cwd() {
        Ok(git) => git,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let height = std::cmp::max(ct::terminal().terminal_size().1, 15);
    let count_arg = format!("-{}", height);
    let args = iter::once(&count_arg).chain(params.git_args.iter());
    let logs = git.log_parsed(args).expect("parse logs");

    let stdout = &mut io::stdout();
    for log in logs {
        let conv = log.as_conventional();

        let scope = conv.clone().and_then(|c| c.scope.map(String::from));
        let ty = match conv {
            Some(ref conv) => conv.ty.to_string(),
            None => "unknown".to_string(),
        };

        let message = match conv {
            Some(ref conv) => conv.message.to_string(),
            None => log.message.clone(),
        };

        ct::queue!(
            stdout,
            ct::SetFg(ct::Color::Yellow),
            ct::Output(log.commit[..8].into()),
            ct::Output(" ".into()),
            ct::SetFg(ct::Color::Magenta),
            ct::Output(ty),
            ct::SetFg(ct::Color::Blue),
            ct::Output(match scope {
                Some(scope) => format!("({}):", scope),
                None => ":".into(),
            }),
            ct::SetFg(ct::Color::Reset),
            ct::Output(" ".into()),
            ct::Output(message),
            ct::Output("\n".into())
        )
        .unwrap();
    }
    stdout.flush();
}