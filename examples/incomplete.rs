use sdp_nom::sdp_line;

fn main() {
    let mut err_count = 1;
    if let Some(arg) = std::env::args().nth(1) {
        if let Ok(content) = std::fs::read_to_string(arg) {
            err_count = 0;
            for line in content.lines() {
                if let Err(e) = sdp_line(line) {
                    println!("{:#}", line);
                    println!("{}\n", e);
                    err_count += 1;
                }
            }
        }
    }
    println!("{} errors", err_count);
    std::process::exit(err_count);
}
