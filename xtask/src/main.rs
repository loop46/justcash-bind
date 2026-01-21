use std::{
    env,
};
use xshell::{Shell, cmd};

type DynError = Box<dyn std::error::Error>;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("build") => build()?,
        Some("publish") => publish()?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:

build	builds application
"
    )
}

fn build() -> Result<(), DynError> {
    let sh = Shell::new()?;

    cmd!(sh, "cargo ndk -t armeabi-v7a -t arm64-v8a -t x86_64 -o ./jclib/src/main/jniLibs/ build --release").run()?;
    cmd!(sh, "cargo run --bin uniffi-bindgen generate --library ./jclib/src/main/jniLibs/x86_64/libjustcash.so --language kotlin --out-dir jclib/src/main/java/").run()?;
    cmd!(sh, "./gradlew build").run()?;
    Ok(())
}

fn publish() -> Result<(), DynError> {
    let sh = Shell::new()?;
    build()?;
    cmd!(sh, "./gradlew publish").run()?;
    Ok(())
}
