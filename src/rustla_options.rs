/*!
This submodule contains the type definition and implementation of ruSTLaOptions.

(c) Santtu Söderholm <santtu.soderholm@tuni.fi>
*/

/// A container for the flags and settings of the ruSTLa transpiler at a type level.
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct ruSTLaOptions {
    /// Choose between standard output and a file next to the original one.
    output_stream: OutputStream,

    /// Whether to add "\(begin&&end){document}" to the output file.
    print_full_document: bool,

    /// A flag that specifies whether the A+ class file should be written next to the source file.
    generate_class_file: bool
}

impl ruSTLaOptions {

    /// The ruSTLaOptions constructor. Receives the command line arguments in a vector of strings,
    /// and constructs the option object from them.
    pub fn new(args: &Vec<String>) -> Self {
        let mut arg_index = 0usize;
        let args_len = args.len();

        let mut options = Self {
            output_stream: OutputStream::StdOut,
            print_full_document: false,
            generate_class_file: false
        };

        while arg_index < args_len {
            let arg = if let Some(arg) = args.get(arg_index) {
                arg
            } else {
                break;
            };

            match arg.as_str() {
                "--to-stdout"   => options.output_stream = OutputStream::StdOut,
                "--to-file"     => options.output_stream = OutputStream::File,
                "--full-doc"    => options.print_full_document = true,
                "--aplus-cls"   => options.generate_class_file = true,
                _ => {}
            }

            arg_index += 1;
        }

        options
    }

    /// Returns a shared reference to the chosen output stream: `stdout` or `file`.
    pub fn shared_out_stream(&self) -> &OutputStream {
        &self.output_stream
    }

    /// Returns a copy of the boolean indicating whether the document is to be directly compilable with pdflatex or not.
    pub fn is_full_document(&self) -> bool {
        self.print_full_document
    }

    /// Returns a copy of the flag which determines whether a class file should be written next to the source file.
    pub fn create_class_file (&self) -> bool {
        self.generate_class_file
    }
}

#[derive(Debug)]
pub enum OutputStream {
    StdOut,
    File,
}
