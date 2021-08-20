use glium::glutin;
use mylib::video_downloader::VideoDownloader;
use tokio::runtime::Builder;

fn create_display(event_loop: &glutin::event_loop::EventLoop<()>) -> glium::Display {
    let window_builder = glutin::window::WindowBuilder::new()
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize {
            width: 620.0,
            height: 640.0,
        })
        .with_title("Arquivos RTP Downloader");

    let context_builder = glutin::ContextBuilder::new()
        .with_depth_buffer(0)
        .with_srgb(true)
        .with_stencil_buffer(0)
        .with_vsync(true);

    glium::Display::new(window_builder, context_builder, event_loop).unwrap()
}


/// Show a button to switch to/from dark/light mode (globally).
fn dark_light_mode_switch(ui: &mut egui::Ui) {
    let style: egui::Style = (*ui.ctx().style()).clone();
    let new_visuals = style.visuals.light_dark_small_toggle_button(ui);
    if let Some(visuals) = new_visuals {
        ui.ctx().set_visuals(visuals);
    }
}


fn main() {
    let runtime = Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();
    
    let video_downloader = VideoDownloader::new();
        
    let event_loop = glutin::event_loop::EventLoop::with_user_event();
    let display = create_display(&event_loop);

    let mut egui = egui_glium::EguiGlium::new(&display);

    let mut links: Vec<String> = Vec::new();
    let mut link: String = String::new();

    event_loop.run(move |event, _, control_flow| {
        let mut redraw = || {
            egui.begin_frame(&display);

            egui::CentralPanel::default().show(egui.ctx(), |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    dark_light_mode_switch(ui);
                    
                    ui.heading("Arquivos RTP Downloader");
                    ui.label("");
                    ui.label("");
                    ui.label("Link do video:");
                    ui.text_edit_singleline(&mut link);

                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        if ui.button("Adicionar link").clicked() {
                            links.push(link.clone());
                            link = String::new();
                        }
                        
                        if ui.button("Limpar Lista").clicked() {
                            links = Vec::new();
                        }

                        if ui.button("Download").clicked() {
                            for l in links.clone() {
                                video_downloader.add_video(l.clone());
                            }

                            runtime.block_on(video_downloader.download());
                        }
                    });

                    ui.label("");
                    ui.separator();
                    ui.label("");

                    egui::ScrollArea::from_max_height(240.0).show(ui, |ui| {
                        for l in links.clone() {
                            let label = format!("- {}", l);
                            ui.label(label);
                        }
                    });
                    
                    ui.label("");
                    ui.separator();
                    ui.label("");

                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                        ui.add(
                            egui::Hyperlink::new("https://github.com/vascoferreira25/arquivos_rtp_downloader").text("Made by Vasco Ferreira"),
                        );
                    });
                    
                });

            });
                                               
            let (needs_repaint, shapes) = egui.end_frame(&display);

            *control_flow = if needs_repaint {
                display.gl_window().window().request_redraw();
                glutin::event_loop::ControlFlow::Poll
            } else {
                glutin::event_loop::ControlFlow::Wait
            };

            {
                use glium::Surface as _;
                let mut target = display.draw();

                let clear_color = egui::Rgba::from_rgb(0.8, 0.8, 0.8);
                target.clear_color(
                    clear_color[0],
                    clear_color[1],
                    clear_color[2],
                    clear_color[3],
                );

                // draw things behind egui here

                egui.paint(&display, &mut target, shapes);

                // draw things on top of egui here

                target.finish().unwrap();
            }
        };

        match event {
            // Platform-dependent event handlers to workaround a winit bug
            // See: https://github.com/rust-windowing/winit/issues/987
            // See: https://github.com/rust-windowing/winit/issues/1619
            glutin::event::Event::RedrawEventsCleared if cfg!(windows) => redraw(),
            glutin::event::Event::RedrawRequested(_) if !cfg!(windows) => redraw(),

            glutin::event::Event::WindowEvent { event, .. } => {
                if egui.is_quit_event(&event) {
                    *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                }

                egui.on_event(&event);

                display.gl_window().window().request_redraw(); // TODO: ask egui if the events warrants a repaint instead
            }

            _ => (),
        }
        });
    }
