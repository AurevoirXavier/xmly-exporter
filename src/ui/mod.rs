mod event_loop;

// --- std ---
use std::{
    path::Path,
    sync::{Arc, Mutex},
};
// --- external ---
use conrod::backend::glium::glium::{self, Surface, glutin};

fn copy_info(button_color: &mut conrod::color::Color, info: &str) {
    // --- external ---
    use clipboard::ClipboardProvider;
    use clipboard::ClipboardContext;

    *button_color = conrod::color::rgb(rand::random(), rand::random(), rand::random());
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(info.to_owned()).unwrap();
}

fn load_track_cover(display: &glium::Display, path: &Path) -> glium::texture::Texture2d {
    let rgba_image = image::open(&path).unwrap().to_rgba();
    let image_dimensions = rgba_image.dimensions();
    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&rgba_image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();

    texture
}

pub fn display() {
    // --- custom ---
    use crate::fetcher::{
        FETCHER,
        album::Album,
    };
    use self::event_loop::EventLoop;

    const WIDTH: u32 = 1200;
    const HEIGHT: u32 = 800;

    let mut events_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_title("xmly-exporter")
        .with_dimensions((WIDTH, HEIGHT).into());
    let context_builder = glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window_builder, context_builder, &events_loop).unwrap();

    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    {
        // --- std ---
        use std::path::Path;

        if cfg!(target_os = "windows") {
            ui.fonts.insert_from_file(Path::new("C:/Windows/Fonts/SIMFANG.ttf")).unwrap();
        } else {
            ui.fonts.insert_from_file(Path::new("/Library/Fonts/Arial Unicode.ttf")).unwrap();
        };
    }

    widget_ids! { struct Ids {
        canvas,
        track_id_button,
        tracks_album_id_button,
        track_title_src_button,
        tracks_album_title_button,
        get_album_detail_button,
        export_album_button,
        album_id_text,
        track_category_text,
        track_nickname_text,
        track_duration_text,
        track_plays_text,
        track_comments_text,
        track_shares_text,
        track_likes_text,
        tracks_list_select,
        track_cover_image,
    }}
    let ids = Ids::new(ui.widget_id_generator());

    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    let mut image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    let album = Arc::new(Mutex::new(Album::new()));
    let mut album_track_url = String::from("Album or Track url");
    let mut track_selected = std::collections::HashSet::new();

    let temp_dir = tempfile::tempdir().unwrap();
    let mut track_cover_src = String::new();
    let mut track_cover_img = load_track_cover(
        &display,
        &FETCHER.fetch_to_temp_file(
            "https://aurevoirxavier.github.io/favicon.ico",
            temp_dir.path(),
        ),
    );
    let track_cover_id = image_map.insert(track_cover_img);

    let mut track_buttons_color = vec![conrod::color::WHITE; 4];
    let mut track_id = String::new();
    let mut track_title_src = String::new();
    let mut track_album_id = String::new();
    let mut track_album_title = String::new();
    let mut track_category = String::new();
    let mut track_nickname = String::new();
    let mut track_duration = String::new();
    let mut track_plays = String::new();
    let mut track_comments = String::new();
    let mut track_shares = String::new();
    let mut track_likes = String::new();

    let mut event_loop = EventLoop::new();
    'main: loop {
        for event in event_loop.next(&mut events_loop) {
            if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }

            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested | glutin::WindowEvent::KeyboardInput {
                        input: glutin::KeyboardInput {
                            virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => break 'main,
                    _ => ()
                }
                _ => ()
            }
        }

        {
            // --- external ---
            use conrod::{Borderable, Colorable, Labelable, Positionable, Sizeable, Widget, color, widget};

            let ui = &mut ui.set_widgets();

            let widget_height = 30.;
            let margin = 40.;
            let font_size = (widget_height / 2.) as conrod::FontSize;
            let canvas_color = color::DARK_CHARCOAL;

            widget::Canvas::new()
                .color(canvas_color)
                .set(ids.canvas, ui);

            widget::Text::new(&album_track_url)
                .w_h(ui.win_w - 80., widget_height)
                .mid_top_with_margin(margin - widget_height)
                .font_size(font_size)
                .color(color::WHITE)
                .center_justify()
                .set(ids.album_id_text, ui);

            {
                {
                    let widget_width = 200.;

                    widget::Image::new(track_cover_id)
                        .w_h(widget_width, widget_width)
                        .top_right_with_margins_on(ui.window, margin, margin)
                        .set(ids.track_cover_image, ui);

                    let margin = 25.;

                    if widget::Button::new()
                        .w_h(widget_width, widget_height)
                        .mid_top_with_margin_on(ids.track_cover_image, widget_width + margin)
                        .label("Click to copy download link")
                        .label_font_size(font_size)
                        .label_color(track_buttons_color[0])
                        .color(canvas_color)
                        .hover_color(canvas_color)
                        .press_color(canvas_color)
                        .border(0.)
                        .set(ids.track_title_src_button, ui)
                        .was_clicked() {
                        copy_info(&mut track_buttons_color[0], &track_title_src);
                    }

                    if widget::Button::new()
                        .w_h(widget_width, widget_height)
                        .mid_top_with_margin_on(ids.track_title_src_button, margin)
                        .label(&format!("Track id: {}", track_id))
                        .label_font_size(font_size)
                        .label_color(track_buttons_color[1])
                        .color(canvas_color)
                        .hover_color(canvas_color)
                        .press_color(canvas_color)
                        .border(0.)
                        .set(ids.track_id_button, ui)
                        .was_clicked() {
                        copy_info(&mut track_buttons_color[1], &track_id);
                    }

                    if widget::Button::new()
                        .w_h(widget_width, widget_height)
                        .mid_top_with_margin_on(ids.track_id_button, margin)
                        .label(&format!("Album: {}", track_album_title))
                        .label_font_size(font_size)
                        .label_color(track_buttons_color[2])
                        .color(canvas_color)
                        .hover_color(canvas_color)
                        .press_color(canvas_color)
                        .border(0.)
                        .set(ids.tracks_album_title_button, ui)
                        .was_clicked() {
                        copy_info(&mut track_buttons_color[2], &track_album_title);
                    }

                    if widget::Button::new()
                        .w_h(widget_width, widget_height)
                        .mid_top_with_margin_on(ids.tracks_album_title_button, margin)
                        .label(&format!("Album id: {}", track_album_id))
                        .label_font_size(font_size)
                        .label_color(track_buttons_color[3])
                        .color(canvas_color)
                        .hover_color(canvas_color)
                        .press_color(canvas_color)
                        .border(0.)
                        .set(ids.tracks_album_id_button, ui)
                        .was_clicked() {
                        copy_info(&mut track_buttons_color[3], &track_album_id);
                    }

                    {
                        widget::Text::new(&track_category)
                            .w_h(widget_width, widget_height)
                            .mid_top_with_margin_on(ids.tracks_album_id_button, widget_height)
                            .font_size(font_size)
                            .color(color::WHITE)
                            .center_justify()
                            .set(ids.track_category_text, ui);

                        widget::Text::new(&track_nickname)
                            .w_h(widget_width, widget_height)
                            .mid_top_with_margin_on(ids.track_category_text, margin)
                            .font_size(font_size)
                            .color(color::WHITE)
                            .center_justify()
                            .set(ids.track_nickname_text, ui);

                        widget::Text::new(&track_duration)
                            .w_h(widget_width, widget_height)
                            .mid_top_with_margin_on(ids.track_nickname_text, margin)
                            .font_size(font_size)
                            .color(color::WHITE)
                            .center_justify()
                            .set(ids.track_duration_text, ui);

                        widget::Text::new(&track_plays)
                            .w_h(widget_width, widget_height)
                            .mid_top_with_margin_on(ids.track_duration_text, margin)
                            .font_size(font_size)
                            .color(color::WHITE)
                            .center_justify()
                            .set(ids.track_plays_text, ui);

                        widget::Text::new(&track_comments)
                            .w_h(widget_width, widget_height)
                            .mid_top_with_margin_on(ids.track_plays_text, margin)
                            .font_size(font_size)
                            .color(color::WHITE)
                            .center_justify()
                            .set(ids.track_comments_text, ui);

                        widget::Text::new(&track_shares)
                            .w_h(widget_width, widget_height)
                            .mid_top_with_margin_on(ids.track_comments_text, margin)
                            .font_size(font_size)
                            .color(color::WHITE)
                            .center_justify()
                            .set(ids.track_shares_text, ui);

                        widget::Text::new(&track_likes)
                            .w_h(widget_width, widget_height)
                            .mid_top_with_margin_on(ids.track_shares_text, margin)
                            .font_size(font_size)
                            .color(color::WHITE)
                            .center_justify()
                            .set(ids.track_likes_text, ui);
                    }
                }

                {
                    let widget_width = 100.;
                    let button_color = color::LIGHT_BLUE;
                    let button_press_color = color::LIGHT_GREY;
                    let label_color = color::BLACK;

                    if widget::Button::new()
                        .w_h(widget_width, widget_height)
                        .mid_bottom_with_margin_on(ids.export_album_button, margin)
                        .label("Fetch")
                        .label_font_size(font_size)
                        .color(button_color)
                        .label_color(label_color)
                        .press_color(button_press_color)
                        .border(0.)
                        .set(ids.get_album_detail_button, ui)
                        .was_clicked() {
                        // --- std ---
                        use std::{
                            time::Duration,
                            thread::{sleep, spawn},
                        };
                        // --- external ---
                        use clipboard::ClipboardProvider;
                        use clipboard::ClipboardContext;
                        // --- custom ---
                        use crate::fetcher::track::Track;

                        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                        if let Ok(paste) = ctx.get_contents() {
                            if album_track_url != paste {
                                if paste.starts_with("http") { album_track_url = paste; }
                                let url_ids: Vec<&str> = album_track_url.split('/')
                                    .filter(|s| !s.is_empty() && s.chars().all(|c| c.is_digit(10)))
                                    .collect();

                                if !url_ids.is_empty() {
                                    album.lock().unwrap().set_id(url_ids[0]).tracks.clear();

                                    match url_ids.len() {
                                        0 => (),
                                        1 => {
                                            let album = album.clone();
                                            spawn(move ||
                                                for page_num in 1u32.. {
                                                    if !album
                                                        .lock()
                                                        .unwrap()
                                                        .next_page(page_num) {
                                                        break;
                                                    }

                                                    sleep(Duration::from_millis(16));
                                                }
                                            );
                                        }
                                        2 => album.lock().unwrap().tracks = vec![Track::fetch(url_ids[1])],
                                        _ => unreachable!(),
                                    }
                                }
                            }
                        }
                    }

                    if widget::Button::new()
                        .w_h(widget_width, widget_height)
                        .bottom_right_with_margins_on(ui.window, margin, margin)
                        .label("Export All")
                        .label_font_size(font_size)
                        .color(button_color)
                        .label_color(label_color)
                        .press_color(button_press_color)
                        .border(0.)
                        .set(ids.export_album_button, ui)
                        .was_clicked() {
                        if !album_track_url.starts_with('A') {
                            if let Ok(album) = album.try_lock() { album.save_aria2_input_file(); }
                        }
                    }
                }
            }

            {
                let ref mut tracks = album.lock().unwrap().tracks;
                let (mut events, scrollbar) = widget::ListSelect::multiple(tracks.len())
                    .flow_down()
                    .item_size(widget_height)
                    .scrollbar_color(color::WHITE)
                    .scrollbar_next_to()
                    .w_h(ui.win_w - 320., ui.win_h - 80.)
                    .top_left_with_margins_on(ids.canvas, margin, margin)
                    .set(ids.tracks_list_select, ui);

                while let Some(event) = events.next(ui, |i| track_selected.contains(&i)) {
                    // --- external ---
                    use conrod::widget::list_select::Event;

                    match event {
                        Event::Item(item) => {
                            let label = &tracks[item.i].title;
                            let (color, label_color) = match track_selected.contains(&item.i) {
                                true => (color::LIGHT_BLUE, color::YELLOW),
                                false => (color::LIGHT_GREY, color::BLACK),
                            };
                            let button = widget::Button::new()
                                .label(label)
                                .label_font_size(font_size)
                                .label_color(label_color)
                                .color(color)
                                .border(0.);
                            item.set(button, ui);
                        }
                        Event::Selection(selection) => {
                            selection.update_index_set(&mut track_selected);

                            if let Some(selected) = track_selected.iter().next() {
                                let ref mut track = tracks[*selected];
                                track.update();

                                if track_cover_src != track.cover {
                                    track_cover_src = track.cover.clone();
                                    track_cover_img = load_track_cover(
                                        &display,
                                        &FETCHER.fetch_to_temp_file(
                                            &track_cover_src,
                                            temp_dir.path(),
                                        ),
                                    );
                                    image_map.replace(track_cover_id, track_cover_img).unwrap();
                                }

                                track_id = track.id.to_string();
                                track_title_src = format!("{} {}", track.title, track.src);
                                track_album_title = track.album_title.clone();
                                track_album_id = track.album_id.to_string();
                                track_category = format!("Category: {}", track.category);
                                track_nickname = format!("Nickname: {}", track.nickname);
                                {
                                    let duration = track.duration;
                                    track_duration = format!("Duration: {}:{:0<2}", duration / 60, duration % 60);
                                }
                                track_plays = format!("Plays: {}", track.plays);
                                track_comments = format!("Comments: {}", track.comments);
                                track_shares = format!("Shares: {}", track.shares);
                                track_likes = format!("Likes: {}", track.likes);
                            }
                        }
                        _ => ()
                    }
                }

                if let Some(s) = scrollbar { s.set(ui); }
            }
        }

        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0., 0., 0., 1.);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}
