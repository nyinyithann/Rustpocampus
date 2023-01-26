use std::io::{self, Write};
use std::fmt::{self};


fn write_error_log(arg: fmt::Arguments) -> std::io::Result<()>{
    writeln!(&mut io::stderr(), "{}", arg )?;
    Ok(())
}

fn main() -> io::Result<()> {

    write_error_log(format_args!("Error number is {}.", 1))?;

    Ok(())
}

