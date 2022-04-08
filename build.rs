use tonic_build;

static PROTOS_DIR: &str = "proto";
static WEATHER_PROTO: &str = "weather_service";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos(format!("{}/{}.proto", PROTOS_DIR, WEATHER_PROTO))?;
    Ok(())
}