use hadal::App;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let event_loop = winit::event_loop::EventLoop::new()?;
    let mut application = App::default();

    Ok(event_loop.run_app(&mut application)?)
}
