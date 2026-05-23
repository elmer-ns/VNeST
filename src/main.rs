use std::{
    collections::HashMap,
    rc::{Rc, Weak},
    sync::Mutex,
};

use smithay_client_toolkit::{
    compositor::{CompositorHandler, CompositorState},
    output::{OutputHandler, OutputState},
    reexports::client::{
        Connection, Dispatch, EventQueue, QueueHandle, globals::registry_queue_init,
        protocol::wl_surface::WlSurface,
    },
    shell::{
        WaylandSurface,
        wlr_layer::{Layer, LayerShell, LayerShellHandler, LayerSurface},
    },
    shm::{Shm, slot::SlotPool},
};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = AppWindow::new()?;

    main_window.run()
}

type Renderer = slint::platform::software_renderer::SoftwareRenderer;

struct LayerShellWindow {
    renderer: Renderer,

    slint_window: slint::Window,
    wl_surface: WlSurface,

    size: slint::PhysicalSize,
}

impl LayerShellWindow {
    pub fn new(backend: &LayerShellBackend, renderer: Renderer) -> Rc<Self> {
        let surface = backend.compositor.create_surface(&backend.qh);
        let layer =
            backend
                .layer_shell
                .create_layer_surface(&backend.qh, surface, Layer::Top, None, None);

        const WIDTH: u32 = 256;
        const HEIGHT: u32 = 256;

        layer.set_size(WIDTH, HEIGHT);

        layer.commit();

        let pool = SlotPool::new(256 * 256 * 4, &backend.shm).expect("Failed to create pool");

        Rc::<LayerShellWindow>::new_cyclic(|weak| Self {
            renderer,

            slint_window: slint::Window::new(weak.clone()),
            wl_surface: surface,

            size: slint::PhysicalSize {
                width: WIDTH,
                height: HEIGHT,
            },
        })
    }
}

impl slint::platform::WindowAdapter for LayerShellWindow {
    fn window(&self) -> &slint::Window {
        &self.slint_window
    }

    fn size(&self) -> slint::PhysicalSize {
        self.size
    }

    fn renderer(&self) -> &dyn slint::platform::Renderer {
        &self.renderer
    }
}

struct LayerShellBackend {
    compositor: CompositorState,
    layer_shell: LayerShell,
    shm: Shm,
    event_queue: EventQueue<State>,
    qh: QueueHandle<State>,

    state: Mutex<State>,
}

impl LayerShellBackend {
    pub fn new() -> Self {
        let conn = Connection::connect_to_env().unwrap();

        let (globals, mut event_queue) = registry_queue_init(&conn).unwrap();
        let qh = event_queue.handle();

        let compositor =
            CompositorState::bind(&globals, &qh).expect("wl_compositor is not available");
        let layer_shell = LayerShell::bind(&globals, &qh).expect("layer shell is not available");
        let shm = Shm::bind(&globals, &qh).expect("wl_shm is not available");

        let state = Mutex::new(State::new());

        Self {
            compositor,
            layer_shell,
            shm,
            event_queue,
            qh,

            state,
        }
    }
}

impl slint::platform::Platform for LayerShellBackend {
    fn create_window_adapter(
        &self,
    ) -> Result<std::rc::Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError> {
        let renderer = Renderer::new();

        let window = LayerShellWindow::new(self, renderer);

        self.state
            .lock()
            .unwrap()
            .windows
            .insert(window.wl_surface.clone(), window.clone());

        Ok(window)
    }

    fn run_event_loop(&self) -> Result<(), slint::PlatformError> {
        loop {
            self.event_queue.blocking_dispatch(&mut self.state);

            if self.state.lock().unwrap().exit {
                break;
            }
        }

        return Ok(());
    }
}

struct State {
    output_state: OutputState,

    exit: bool,

    windows: HashMap<WlSurface, Rc<LayerShellWindow>>,
}

impl State {
    pub fn new() -> Self {
        todo!()
    }

    pub fn draw(qh: &QueueHandle<Self>) {}
}

impl CompositorHandler for State {
    fn scale_factor_changed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &smithay_client_toolkit::reexports::client::protocol::wl_surface::WlSurface,
        _new_factor: i32,
    ) {
    }

    fn transform_changed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &smithay_client_toolkit::reexports::client::protocol::wl_surface::WlSurface,
        _new_transform: smithay_client_toolkit::reexports::client::protocol::wl_output::Transform,
    ) {
    }

    fn frame(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        _surface: &smithay_client_toolkit::reexports::client::protocol::wl_surface::WlSurface,
        _time: u32,
    ) {
        self.draw(qh);
    }

    fn surface_enter(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &smithay_client_toolkit::reexports::client::protocol::wl_surface::WlSurface,
        _output: &smithay_client_toolkit::reexports::client::protocol::wl_output::WlOutput,
    ) {
    }

    fn surface_leave(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &smithay_client_toolkit::reexports::client::protocol::wl_surface::WlSurface,
        _output: &smithay_client_toolkit::reexports::client::protocol::wl_output::WlOutput,
    ) {
    }
}

impl OutputHandler for State {
    fn output_state(&mut self) -> &mut smithay_client_toolkit::output::OutputState {
        &mut self.output_state
    }

    fn new_output(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: smithay_client_toolkit::reexports::client::protocol::wl_output::WlOutput,
    ) {
    }

    fn update_output(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: smithay_client_toolkit::reexports::client::protocol::wl_output::WlOutput,
    ) {
    }

    fn output_destroyed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: smithay_client_toolkit::reexports::client::protocol::wl_output::WlOutput,
    ) {
    }
}

impl LayerShellHandler for State {
    fn closed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        layer: &smithay_client_toolkit::shell::wlr_layer::LayerSurface,
    ) {
        let surface = layer.wl_surface();
        let _window = self
            .windows
            .remove(surface)
            .expect("Window requested closed doesn't exist");

        // exit if there are no more windows left
        self.exit = self.windows.len() == 0;
    }

    fn configure(
        &mut self,
        conn: &Connection,
        qh: &QueueHandle<Self>,
        layer: &smithay_client_toolkit::shell::wlr_layer::LayerSurface,
        configure: smithay_client_toolkit::shell::wlr_layer::LayerSurfaceConfigure,
        serial: u32,
    ) {
        let surface = layer.wl_surface();
        let window = self
            .windows
            .get_mut(surface)
            .expect("configured window doesn't exist");

        window.size = slint::PhysicalSize {
            width: configure.new_size.0,
            height: configure.new_size.1,
        };
    }
}
