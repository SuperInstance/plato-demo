use plato_demo::FishingBoatDemo;

fn main() {
    let demo = FishingBoatDemo::new()
        .with_ticks(80);

    demo.run();
}
