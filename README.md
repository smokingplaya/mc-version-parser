# Minecraft Version Parser
A crate that allows you to parse a version of the minecraft file, to collect arguments to run the game.

## Installing
> [!NOTE]
> I'm so lazy to publish this crate on crates.io, so use the git repository.\

Add crate to your cargo.toml
```toml
[dependencies]
mc_version_parser = {git="https://github.com/smokingplaya/mc-version-parser"}
```

## Usage
There is example:
```rs
use mc_version_parser::parse;

fn main() -> anyhow::Result<()> {
  let file = File::open("/home/user/.minecraft/versions/1.16.5/1.16.5.json")?;
  let version = parse(file);

  // Printing mainClass field
  println!("{}", version.mainClass);

  // Collecting libraries
  let libs = version.libraries
    .collect();

  let arguments = version.arguments
    .collect();

  // there should be some logic here for formatting the arguments;
  // for example, for the classpath substitution, the libs variable

  let process = Command::new("javaw")
    .args(arguments)
    .spawn();

  Ok(())
}
```