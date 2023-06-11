use std::println;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let (a, b) = tokio::join!(do_sync(30), do_sync(300));
    println!("process");
    println!("{a}, {b}");

    Ok(())
}

async fn do_sync(arg: u32) -> u32 {
    sleep(1500, "do_sync").await;
    arg * arg
}

async fn sleep(duration: u32, name: impl Into<String>) {
    println!("- sleeping '{}'", name.into());
    tokio::time::sleep(std::time::Duration::from_millis(duration.into())).await;
}
