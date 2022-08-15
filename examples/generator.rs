#[tokio::main]
async fn main() {
    env_logger::init();
    let things = lci_gateway::get_things()
        .await
        .expect("Couldn't get things");

    let generator = things
        .into_iter()
        .filter(|thing| thing.label() == "Generator")
        .collect::<Vec<_>>()
        .remove(0);
    let mut generator =
        lci_gateway::Generator::new(generator).expect("Failed to convert to generator");
    println!("Generator loaded");
    println!(
        "Generator state = {}",
        generator
            .state()
            .await
            .expect("Failed to get generator state")
    );
    println!("Turning generator on");
    generator.on().await;
    println!(
        "Generator state = {}",
        generator
            .state()
            .await
            .expect("Failed to get generator state")
    );
    let wait_min = 5;
    println!("Waiting {} seconds ({}m)", wait_min * 60, wait_min);
    tokio::time::sleep(std::time::Duration::from_secs(wait_min * 60)).await;
    println!(
        "Generator state = {}",
        generator
            .state()
            .await
            .expect("Failed to get generator state")
    );
    println!("Turning the generator off");
    generator.off().await;
    println!(
        "Generator state = {}",
        generator
            .state()
            .await
            .expect("Failed to get generator state")
    );
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    println!(
        "Generator state = {}",
        generator
            .state()
            .await
            .expect("Failed to get generator state")
    );
}
