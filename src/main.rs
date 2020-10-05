use encoding::*;

fn main() {
    let codepage: CodePage = load_codepage("cp437.txt");
    let input = read_file("386intel.txt");
    let output = convert_bytes(input, codepage);
    write_output("output.txt", output);
}
