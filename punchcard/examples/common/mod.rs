extern crate warmy;
extern crate glutin;
extern crate winit;
extern crate webrender;
extern crate euclid;
extern crate gleam;

use self::euclid::vec2;
use self::gleam::gl;
use self::glutin::GlContext;
use pretty_env_logger;
use punchcard::*;
use std::path::{Path, PathBuf};
use std::env;
use std::cmp::{min,max};
use self::webrender::api::*;

const INIT_WINDOW_SIZE: (u32, u32) = (1024, 720);

/*fn ui_render(
    ui: &mut Ui,
    api: &RenderApi,
    builder: &mut DisplayListBuilder,
    txn: &mut Transaction,
    framebuffer_size: DeviceUintSize,
    pipeline_id: PipelineId,
    document_id: DocumentId)
{
    let mut wr_renderer = WRRenderer {
        api,
        builder,
        txn,
        framebuffer_size,
        pipeline_id,
        document_id,
    };

    debug!("fb size= {:?}", framebuffer_size);
    ui.render((framebuffer_size.width as f32, framebuffer_size.height as f32), &mut wr_renderer);
}*/

/*fn ui_event(
    ui: &mut Ui,
    event: winit::WindowEvent,
    api: &RenderApi,
    document_id: DocumentId) -> bool
{
    ui.dispatch_event(&event);
    // don't redraw
    false
}*/



pub fn main_wrapper(title: &str, width: u32, height: u32, mut f: impl FnMut(&mut DomSink))
{
    env::set_current_dir(env!("CARGO_MANIFEST_DIR"));
    pretty_env_logger::init();

    let args: Vec<String> = env::args().collect();
    let res_path = if args.len() > 1 {
        Some(PathBuf::from(&args[1]))
    } else {
        None
    };

    //========================================================================
    //========================================================================
    // Window & GL context setup
    // ========================================================================
    // ========================================================================
    let mut events_loop = winit::EventsLoop::new();
    let context_builder = glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_gl(glutin::GlRequest::GlThenGles {
            opengl_version: (4, 6),
            opengles_version: (3, 0),
        });
    let window_builder = winit::WindowBuilder::new()
        .with_title(title)
        .with_multitouch()
        .with_dimensions(width, height);
    let window = glutin::GlWindow::new(window_builder, context_builder, &events_loop)
        .unwrap();

    unsafe {
        window.make_current().ok();
    }

    /*let gl = match window.get_api() {
        glutin::Api::OpenGl => unsafe {
            gl::GlFns::load_with(|symbol| window.get_proc_address(symbol) as *const _)
        },
        glutin::Api::OpenGlEs => unsafe {
            gl::GlesFns::load_with(|symbol| window.get_proc_address(symbol) as *const _)
        },
        glutin::Api::WebGl => unimplemented!(),
    };*/


    //========================================================================
    //========================================================================
    // UI
    //========================================================================
    //========================================================================
    let mut ui = Ui::new(&window, &events_loop);
    ui.load_stylesheet("data/css/default.css");

    //========================================================================
    //========================================================================
    // Event loop
    //========================================================================
    //========================================================================

    // initial render
    ui.update(|dom| f(dom));

    println!("Entering event loop");

    let mut should_close = false;

    while !should_close {
        let frame_time = measure_time(|| {
            events_loop.poll_events(|ev| {
                match ev {
                    winit::Event::WindowEvent { event, .. } => {
                        match event {
                            winit::WindowEvent::CloseRequested => should_close = true,
                            _ => {
                                ui.event(&event)
                            }
                        }
                    }
                    _ => {},
                };
            });

            ui.update(|dom| f(dom));
            ui.render(&window);
            window.swap_buffers().ok();
        });
        //debug!("frame time: {}us", frame_time);
        // target 60fps
        /*if frame_time < 1_000_000 / 60 {
            ::std::thread::sleep(::std::time::Duration::from_micros(1_000_000 / 60 - frame_time));
        }*/
    }

}

// Supporting multi-window and other stuff
// - Should be transparent to the user
// - let ownership of the event loop to the user
// - must provide an interface to create a window
// - *** or just use winit+webrender internally
// - pass ref to event_loop to Ui::new(&event_loop, context_parameters)
// - Ui::set_context_parameters() for the GL (or Vulkan?) context of the created platform windows.
// - Then, call Ui::platform_window(|ui| {
//      window.width = XXX;
//      window.height = XXX;
//      window.title = XXX;
//      window.show_decorations = XXX;
// })
// - depends on the simplification / refactor of UI specification
