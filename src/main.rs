// Copyright 2016 Mozilla Foundation. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate getopts;
extern crate encoding_rs;

use getopts::Options;
use encoding_rs::*;
use std::io::Write;
use std::io::Read;
use std::fs::File;
use std::path::Path;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [-f INPUT_ENCODING] [-t OUTPUT_ENCODING] [-o OUTFILE] [INFILE] \
                         [...]",
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

fn convert_via_utf8(decoder: &mut Decoder,
                    encoder: &mut Encoder,
                    read: &mut Read,
                    write: &mut Write,
                    last: bool) {
    let mut input_buffer = [0u8; 2048];
    let mut intermediate_buffer_bytes = [0u8; 4096];
    // Is there a safe way to create a stack-allocated &mut str?
    let mut intermediate_buffer: &mut str = unsafe {
        std::mem::transmute(&mut intermediate_buffer_bytes[..])
    };
    let mut output_buffer = [0u8; 4096];
    let mut current_input_ended = false;
    while !current_input_ended {
        match read.read(&mut input_buffer) {
            Err(_) => {
                print!("Error reading input.");
                std::process::exit(-5);
            }
            Ok(decoder_input_end) => {
                current_input_ended = decoder_input_end == 0;
                let input_ended = last && current_input_ended;
                let mut decoder_input_start = 0usize;
                loop {
                    let (decoder_result, decoder_read, decoder_written, _) = decoder.decode_to_str(&input_buffer[decoder_input_start..decoder_input_end],
                                                            &mut intermediate_buffer,
                                                            input_ended);
                    decoder_input_start += decoder_read;

                    let last_output = if input_ended {
                        match decoder_result {
                            CoderResult::InputEmpty => true,
                            CoderResult::OutputFull => false,
                        }
                    } else {
                        false
                    };

                    // Regardless of whether the intermediate buffer got full
                    // or the input buffer was exhausted, let's process what's
                    // in the intermediate buffer.

                    if encoder.encoding() == UTF_8 {
                        // If the target is UTF-8, optimize out the encoder.
                        match write.write_all(&intermediate_buffer.as_bytes()[..decoder_written]) {
                            Err(_) => {
                                print!("Error writing output.");
                                std::process::exit(-7);
                            }
                            Ok(_) => {}
                        }
                    } else {
                        let mut encoder_input_start = 0usize;
                        loop {
                            let (encoder_result, encoder_read, encoder_written, _) = encoder.encode_from_utf8(&intermediate_buffer[encoder_input_start..decoder_written], &mut output_buffer, last_output);
                            encoder_input_start += encoder_read;
                            match write.write_all(&output_buffer[..encoder_written]) {
                                Err(_) => {
                                    print!("Error writing output.");
                                    std::process::exit(-6);
                                }
                                Ok(_) => {}
                            }
                            match encoder_result {
                                CoderResult::InputEmpty => {
                                    break;
                                }
                                CoderResult::OutputFull => {
                                    continue;
                                }
                            }
                        }
                    }

                    // Now let's see if we should read again or process the
                    // rest of the current input buffer.
                    match decoder_result {
                        CoderResult::InputEmpty => {
                            break;
                        }
                        CoderResult::OutputFull => {
                            continue;
                        }
                    }
                }
            }
        }
    }
}

fn convert_via_utf16(decoder: &mut Decoder,
                     encoder: &mut Encoder,
                     read: &mut Read,
                     write: &mut Write,
                     last: bool) {
    let mut input_buffer = [0u8; 2048];
    let mut intermediate_buffer = [0u16; 2048];
    let mut output_buffer = [0u8; 4096];
    let mut current_input_ended = false;
    while !current_input_ended {
        match read.read(&mut input_buffer) {
            Err(_) => {
                print!("Error reading input.");
                std::process::exit(-5);
            }
            Ok(decoder_input_end) => {
                current_input_ended = decoder_input_end == 0;
                let input_ended = last && current_input_ended;
                let mut decoder_input_start = 0usize;
                loop {
                    let (decoder_result, decoder_read, decoder_written, _) = decoder.decode_to_utf16(&input_buffer[decoder_input_start..decoder_input_end],
                                                            &mut intermediate_buffer,
                                                            input_ended);
                    decoder_input_start += decoder_read;

                    let last_output = if input_ended {
                        match decoder_result {
                            CoderResult::InputEmpty => true,
                            CoderResult::OutputFull => false,
                        }
                    } else {
                        false
                    };

                    // Regardless of whether the intermediate buffer got full
                    // or the input buffer was exhausted, let's process what's
                    // in the intermediate buffer.

                    let mut encoder_input_start = 0usize;
                    loop {
                        let (encoder_result, encoder_read, encoder_written, _) = encoder.encode_from_utf16(&intermediate_buffer[encoder_input_start..decoder_written], &mut output_buffer, last_output);
                        encoder_input_start += encoder_read;
                        match write.write_all(&output_buffer[..encoder_written]) {
                            Err(_) => {
                                print!("Error writing output.");
                                std::process::exit(-6);
                            }
                            Ok(_) => {}
                        }
                        match encoder_result {
                            CoderResult::InputEmpty => {
                                break;
                            }
                            CoderResult::OutputFull => {
                                continue;
                            }
                        }
                    }

                    // Now let's see if we should read again or process the
                    // rest of the current input buffer.
                    match decoder_result {
                        CoderResult::InputEmpty => {
                            break;
                        }
                        CoderResult::OutputFull => {
                            continue;
                        }
                    }
                }
            }
        }
    }
}

fn convert(decoder: &mut Decoder,
           encoder: &mut Encoder,
           read: &mut Read,
           write: &mut Write,
           last: bool,
           use_utf16: bool) {
    if use_utf16 {
        convert_via_utf16(decoder, encoder, read, write, last);
    } else {
        convert_via_utf8(decoder, encoder, read, write, last);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("o",
                "output",
                "set output file name (- for stdout; the default)",
                "PATH");
    opts.optopt("f",
                "from-code",
                "set input encoding (defaults to UTF-8)",
                "LABEL");
    opts.optopt("t",
                "to-code",
                "set output encoding (defaults to UTF-8)",
                "LABEL");
    opts.optflag("u",
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

    if matches.opt_present("h") {
        print_usage(&program, opts);
        std::process::exit(0);
    }

    let input_encoding = get_encoding(matches.opt_str("f"));
    let output_encoding = get_encoding(matches.opt_str("t"));
    let use_utf16 = matches.opt_present("u");
    let mut output = match matches.opt_str("o").as_ref().map(|s| &s[..]) {
        None | Some("-") => Box::new(std::io::stdout()) as Box<Write>,
        Some(path_string) => {
            match File::create(&Path::new(path_string)) {
                Ok(file) => Box::new(file) as Box<Write>,
                Err(_) => {
                    print!("Cannot open {} for writing; exiting.", path_string);
                    std::process::exit(-3);
                }
            }
        }
    };

    let mut decoder = input_encoding.new_decoder();
    let mut encoder = output_encoding.new_encoder();

    if matches.free.is_empty() {
        convert(&mut decoder,
                &mut encoder,
                &mut std::io::stdin(),
                &mut output,
                true,
                use_utf16);
    } else {
        let mut iter = matches.free.iter().peekable();
        loop {
            match iter.next() {
                None => {
                    break;
                }
                Some(path_string) => {
                    match &path_string[..] {
                        "-" => {
                            convert(&mut decoder,
                                    &mut encoder,
                                    &mut std::io::stdin(),
                                    &mut output,
                                    iter.peek().is_none(),
                                    use_utf16);
                        }
                        _ => {
                            match File::open(&Path::new(&path_string)) {
                                Ok(mut file) => {
                                    convert(&mut decoder,
                                            &mut encoder,
                                            &mut file,
                                            &mut output,
                                            iter.peek().is_none(),
                                            use_utf16);
                                }
                                Err(_) => {
                                    print!("Cannot open {} for reading; exiting.", &path_string);
                                    std::process::exit(-4);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    match output.flush() {
        Ok(_) => {}
        Err(_) => {
            print!("Cannot flush output; exiting.");
            std::process::exit(-3);
        }
    }
}
