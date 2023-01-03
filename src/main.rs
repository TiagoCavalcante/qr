fn main() {
  let args: Vec<_> = std::env::args().collect();

  let data = match args.len() {
    1 => {
      let mut buffer = String::with_capacity(256);
      match std::io::stdin().read_line(&mut buffer) {
        Ok(_) => buffer.trim().to_string(),
        Err(_) => {
          eprintln!("Failed to read from stdin");
          std::process::exit(1);
        }
      }
    }
    2 => args[1].clone(),
    _ => {
      eprintln!("Too many arguments");
      std::process::exit(1);
    }
  };

  match qrcode::QrCode::new(data) {
    Ok(code) => println!(
      "{}",
      code
        .render::<qrcode::render::unicode::Dense1x2>()
        .build(),
    ),
    Err(_) => {
      eprintln!(
        "Couldn't create a QR code, the input is too big",
      );
      std::process::exit(1);
    }
  };
}
