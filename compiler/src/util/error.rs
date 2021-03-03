#[derive(Debug, Clone, PartialEq)]
pub struct RaptorexError<'a> {
    pub diagnostic: Diagnostic<'a>,
    pub line: u32,
    pub file: &'a str,
}

impl std::fmt::Display for RaptorexError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.diagnostic.help_msgs.len() >= 1 && self.diagnostic.notes.len() >= 1 {
            // help messages and notes are included
        } else if self.diagnostic.help_msgs.len() >= 1 {
            // only help messages are included
        } else if self.diagnostic.notes.len() >= 1 {
            // only notes are included
            write!(
                f,
                "Error: {} {}:{}",
                self.diagnostic.message, self.file, self.line
            )?;
        } else {
            // only error message
            write!(
                f,
                "Error: {} {}:{}",
                self.diagnostic.message, self.file, self.line
            )?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Diagnostic<'a> {
    pub notes: Vec<&'a str>,
    pub help_msgs: Vec<&'a str>,
    pub message: &'a str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Diagnostics<'a> {
    errors: Vec<RaptorexError<'a>>,
}

impl<'a> Diagnostics<'a> {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn add_error(&mut self, error: RaptorexError<'a>) {
        self.errors.push(error);
    }
}

// #[macro_export]
// macro_rules! throw_error {
//     () => {
//         RaptorexError {
//             message: String::new(),
//             line: line!(),
//             file: file!().to_owned(),
//         }
//     };

//     ($($msg:tt),*) => {{
//         use crate::util::error::RaptorexError;
//             let mut final_msg = String::new();

//             $(
//                 final_msg.push_str(&format!("{} ", $msg));
//             )*

//             // remove trailing whitespace
//             final_msg.pop();

//             RaptorexError {
//                 diagnostic: Diagnostic::new(),
//                 line: line!(),
//                 file: file!(),
//             }
//         }};
// }
