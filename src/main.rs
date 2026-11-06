use hadal::Application;

fn main() -> anyhow::Result<()> {
    env_logger::builder().format_timestamp(None).init();

    let event_loop = winit::event_loop::EventLoop::new()?;
    let mut application = Application::default();

    Ok(event_loop.run_app(&mut application)?)
}
