extern crate getopts;
extern crate encoding_rs;

use getopts::Options;
use encoding_rs::*;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} -ie INPUT_ENCODING -oe OUTPUT_ENCODING -o OUTFILE INFILE  ...",
                        program);
    print!("{}", opts.usage(&brief));
}

fn get_encoding(opt: Option<String>) -> &'static Encoding {
    match opt {
        None => UTF_8,
        Some(label) => {
            match Encoding::for_label((&label).as_bytes()) {
                None => {
                    print!("{} is not a known encoding label; exiting.", label);
                    std::process::exit(-2);
                }
                Some(encoding) => encoding,
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("o",
                "output-file",
                "set output file name (- for stdout; the default)",
                "PATH");
    opts.optopt("ie",
                "input-encoding",
                "set input encoding (defaults to UTF-8)",
                "LABEL");
    opts.optopt("oe",
                "output-encoding",
                "set output encoding (defaults to UTF-8)",
                "LABEL");
    opts.optflag("16",
                 "utf16-intermediate",
                 "use UTF-16 instead of UTF-8 as the intermediate encoding");
    opts.optflag("h", "help", "print usage help");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_) => {
            print_usage(&program, opts);
            std::process::exit(-1);
        }
    };

    let input_encoding = get_encoding(matches.opt_str("ie"));
}
