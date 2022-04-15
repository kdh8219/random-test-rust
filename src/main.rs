use rand::Rng;
use std::process::Command;
fn main() {
    let mut line_number: i128 = 0;
    loop {
        let number: i64 = rand::thread_rng().gen_range(0, 25565);
        line_number += 1;
        println!("{}", &number);
        if number == 0 {
            println!("finish in {} lines", line_number);
            line_number = 0;
            let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
        };
    }
}
