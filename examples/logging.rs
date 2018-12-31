fn main() -> Result<(), failure::Error> {
    let output = convey::new()
        .add_target(convey::human::stdout()?)?
        .use_as_logger(log::Level::Debug)?;

    output.print("hello")?;
    log::info!("welcome");
    log::error!("oh noes");

    Ok(())
}
