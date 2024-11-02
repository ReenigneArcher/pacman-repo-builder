use super::super::{args::CopyrightArgs, status::Status, utils::LICENSE};

pub fn copyright(args: CopyrightArgs) -> Status {
    let CopyrightArgs {} = args;
    print!("{}", LICENSE);
    Ok(())
}
