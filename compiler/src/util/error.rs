#[macro_export]
macro_rules! throw_error {
    () => {
        RaptorexError {
            message: String::new(),
            line: line!(),
            file: file!().to_owned(),
        }
    };

    ($($msg:tt),*) => {
        {
            let mut final_msg = String::new();

            $(
                final_msg.push_str(&format!("{} ", $msg));
            )*

            // remove trailing whitespace
            final_msg.pop();


            RaptorexError {
                message: final_msg,
                line: line!(),
                file: file!(),
            }
        }
    }
}
