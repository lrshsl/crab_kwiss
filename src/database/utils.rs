
pub struct WordPair {
    pub word: String,
    pub definition: String,
}

pub(crate) fn throw_error(msg: &str, code: i32) {
    eprintln!("{}", msg);
    std::process::exit(code);
}

pub(crate) const HELP_MENU: &str = "
USAGE:
    kwiss COMMAND [ARGS] [OPTIONS]

Commands:
    create <set>                            create a new set called <set>
    add <word> <definition> to <set>        add a new entry to <set>
    start <MODE> <set>                      start a new game in <MODE> mode     [Not yet implemented]

    dump <set>                              print all entries in <set>
    help                                    show this help message

Modes:   [Not yet implemented]
    learning                                normal mode
    test                                    test mode

Options:
    -h, --help                              show this help message
    -c, --config <path>                     specify a custom config file        [Not yet implemented]
";

