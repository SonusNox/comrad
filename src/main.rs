use eframe::{App, CreationContext, NativeOptions, run_native};

use egui::{
    Align,
    Button,
    CentralPanel,
    Color32,
    Context,
    Frame,
    IconData,
    Id,
    Label,
    Layout,
    Margin,
    PointerButton,
    Response,
    Rgba,
    RichText,
    ScrollArea,
    Sense,
    SidePanel,
    Slider,
    TextEdit,
    TopBottomPanel,
    Ui,
    Vec2,
    ViewportBuilder,
    ViewportClass,
    ViewportCommand,
    ViewportId,
    Visuals
};

use egui_extras::install_image_loaders;
use egui_file_dialog::FileDialog;
use image::open;

use std::{
    collections::HashMap,
    io::Result,
    sync::{Arc, Mutex},
    time::{Duration, Instant}
};

use comrad::{
    media::{
        playback::Playback,
        source::Playlist
    },
    utils::{data, filesys, images, styles}
};

enum PlayState { Paused, Playing, Stopped }
enum RepeatState { All, None, One }

impl Main {
    //** Misc. **//
    fn new(cc: &CreationContext<'_>) -> Self {
        images::load(cc);

        let dir = filesys::get_dir();

        Self {
            // Booleans
            is_shuffled: false,
            big_player_open: false,
            mini_player_open: false,
            playlist_add_open: false,
            playlist_edit_open: false,
            start_playback: false,
            stop_playback: false,

            // Dialogs
            dir_dialog: FileDialog::new().initial_directory(dir.clone().into()),
            playlist_dialog: FileDialog::new().initial_directory(dir.clone().into()),

            // Directories
            dir: dir.clone(),
            path: dir.clone(),
            folders: Vec::new(),

            // Files
            playback: Arc::new(Mutex::new(Playback::new())),

            // Misc.
            volume: filesys::get_volume(),
            temp_playlist_name: String::new(),

            // Playlists
            active_playlist: None,
            edited_playlist: None,
            now_playinglist: None,
            pseudo_playlist: None,
            sorted_playlist: None,
            playlists: filesys::get_playlists(),

            // Sources
            now_playing: String::new(),
            selected: String::new(),

            // States
            play_state: PlayState::Stopped,
            repeat_state: RepeatState::None,
    
            // Time
            elapsed_time: Duration::ZERO,
            start_time: Duration::ZERO,
            total_time: Duration::ZERO,
            stopwatch_instant: None
        }
    }

    //** Playback **//
    fn pause(&mut self) {
        self.play_state = PlayState::Paused;
        self.start_playback = false;
        self.stop_playback = true;

        if let Ok(mut player) = self.playback.try_lock() { player.pause(); };
    }
    
    fn play(&mut self) {
        self.shuffle();

        if let Ok(mut player) = self.playback.try_lock() {
            match self.play_state {
                PlayState::Paused | PlayState::Stopped => {
                    self.play_state = PlayState::Playing;
                    self.start_playback = true;
                    self.stop_playback = false;

                    let _ = player.play(self.now_playing.clone());
                },

                PlayState::Playing => {
                    self.play_state = PlayState::Paused;
                    self.start_playback = false;
                    self.stop_playback = true;

                    player.pause();
                    player.try_seek(self.get_elapsed_time());
                }
            };
        };
    }

    fn repeat(&mut self) {
        match self.repeat_state {
            RepeatState::All => self.repeat_state = RepeatState::One,
            RepeatState::None => self.repeat_state = RepeatState::All,
            RepeatState::One => self.repeat_state = RepeatState::None
        };
    }
    
    fn shuffle(&mut self) {
        match self.is_shuffled {
            false => {
                if let Some(sorted) = self.sorted_playlist.clone() {
                    if !sorted.is_empty() { self.now_playinglist = Some(sorted); };
                };
            },
            
            true => {
                let mut index = 0;
                let mut playlist = Playlist::new(String::new());
                let mut shuffled: HashMap<i32, String> = HashMap::new();

                if let Some(list) = &self.now_playinglist.clone() {
                    if let Some(sources) = list.clone().get_sources() {
                        for source in sources {
                            shuffled.insert(index, source.clone());

                            index += 1;
                        };
                    };
                };
                
                for (_index, source) in shuffled { playlist.add_source(Some(source)); };

                self.now_playinglist = Some(playlist);
            }
        };
    }
    
    fn skip_backward(&mut self) {
        self.stop();

        if let Some(playlist) = &self.now_playinglist.clone() {
            if let Some(sources) = playlist.clone().get_sources() {
                let mut index = 0;

                for source in sources {
                    if self.now_playing == source {
                        if let Some(first) = playlist.clone().get_source(0) {
                            if source != first && self.elapsed_time.as_secs() <= 3 {
                                if let Some(item) = playlist.clone().get_source(index - 1) {
                                    self.now_playing = item;
                                };
                            };
                        };
                    } else { index += 1; };
                };
            };
        };
    }
    
    fn skip_forward(&mut self) {
        self.stop();

        if let Some(playlist) = &self.now_playinglist {
            if let Some(last) = &playlist.get_source(playlist.len() - 1) {
                if last != &self.now_playing {
                    if let Some(sources) = &playlist.get_sources() {
                        let mut index = 0;

                        for source in sources {
                            if source == &self.now_playing {
                                if let Some(next) = &playlist.get_source(index + 1) {
                                    self.now_playing = next.to_string();
                                };
                            } else { index += 1; };
                        };

                        self.play();
                    };
                } else {
                    match self.repeat_state {
                        RepeatState::All => {
                            if let Some(sources) = &playlist.get_sources() {
                                if let Some(source) = sources.get(0) {
                                    self.now_playing = source.to_string();

                                    self.play();
                                };
                            };
                        },

                        RepeatState::None | RepeatState::One => {}
                    };
                };
            };
        };
    }
    
    fn stop(&mut self) {
        self.play_state = PlayState::Stopped;
        self.start_playback = false;
        self.stop_playback = true;

        self.start_time = Duration::ZERO;
        self.elapsed_time = Duration::ZERO;
        self.total_time = Duration::ZERO;

        if let Ok(mut player) = self.playback.try_lock() { player.pause(); };
    }
    
    //** Playlist **//
    fn load_sources(&mut self, dir: String) -> Result<()> {
        let mut sources = Playlist::new(String::new());

        let mut entries = filesys::read_dir(dir.clone())?.map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>>>()?;

        entries.sort();

        for entry in entries {
            let file_path = entry.as_path().display().to_string();

            if entry.as_path().is_file() { sources.add_source(Some(file_path.clone())); }
            else {
                let mut entries1 = filesys::read_dir(file_path.clone())?
                    .map(|res| res.map(|e| e.path())).collect::<Result<Vec<_>>>()?;

                entries1.sort();

                for entry1 in entries1 {
                    let file_path1 = entry1.as_path().display().to_string();

                    if entry1.as_path().is_file() { sources.add_source(Some(file_path1.clone())); }
                    else {
                        let mut entries2 = filesys::read_dir(file_path1.clone())?
                            .map(|res| res.map(|e| e.path())).collect::<Result<Vec<_>>>()?;

                        entries2.sort();

                        for entry2 in entries2 {
                            let file_path2 = entry2.as_path().display().to_string();

                            if entry2.as_path().is_file() {
                                sources.add_source(Some(file_path2.clone()));
                            };
                        };
                    };
                };
            };
        };

        self.pseudo_playlist = Some(sources);
        self.sorted_playlist = self.pseudo_playlist.clone();

        Ok(())
    }

    fn save_playlists(&mut self, playlist: Option<Playlist>) {
        let mut playlists: Vec<Playlist> = Vec::new();

        for entry in &mut self.playlists.clone() {
            if let Some(list) = playlist.clone() {
                if list.get_id() == entry.get_id() { playlists.push(list.clone()); }
                else { playlists.push(entry.clone()); };
            };
        };

        self.playlists = playlists.clone();

        let _ = filesys::edit_playlists(self.playlists.clone());
    }

    //** Time **//
    fn get_elapsed_time(&self) -> Duration {
        match self.stopwatch_instant {
            None => self.elapsed_time,
            Some(instance) => instance.elapsed() + self.start_time
        }
    }

    fn setup_stopwatch(&mut self) {
        self.elapsed_time = self.get_elapsed_time();

        if self.start_playback {
            self.start_time = self.elapsed_time;
            self.stopwatch_instant = Some(Instant::now());
        };

        if self.stop_playback { self.stopwatch_instant = None; };
    }

    fn update_playback(&mut self) {
        if self.now_playing != String::new() {
            self.total_time = data::get_total_time(&self.now_playing);

            if self.get_elapsed_time() >= self.total_time && self.total_time != Duration::ZERO {
                self.elapsed_time = Duration::ZERO;

                self.stop();

                self.play_state = PlayState::Stopped;

                match self.repeat_state {
                    RepeatState::All => self.skip_forward(),
                    RepeatState::None => self.skip_forward(),
                    RepeatState::One => self.play()
                };
            };
        } else { self.total_time = Duration::ZERO; };
    }

    //** UI - Buttons **//
    fn big_player_button(&mut self, ui: &mut Ui) {
        let icon = images::get_big_player();
        let button = Button::new(icon);
        let component = ui.add_sized([30.0, 30.0], button);

        if component.clicked() {
            match self.big_player_open {
                false => self.big_player_open = true,
                true => self.big_player_open = false
            };
        };
    }

    fn catalog_back_button(&mut self, ui: &mut Ui) {
        let button = Button::new("Back");
        let component = ui.add_sized([ui.available_width(), 30.0], button);
        let dir = self.dir.clone();

        if component.clicked() {
            self.selected = String::new();

            if self.folders.len() == 0 { self.path = dir.clone(); }
            else {
                let mut folder = self.folders.pop();

                if Some(self.path.clone()) == folder && self.folders.len() > 0 {
                    folder = self.folders.last().cloned();
                } else { folder = Some(dir.clone()); }
                
                self.path = folder.clone().unwrap();
            };
        };
    }

    fn change_dir_button(&mut self, ui: &mut Ui) {
        let button = Button::new("Change Directory");
        let component = ui.add_sized([30.0, 30.0], button);

        if component.clicked() {
            self.selected = String::new();
            
            self.dir_dialog.pick_directory();
        };
    }

    fn dir_button(&mut self, ui: &mut Ui) {
        let button = Button::new("Directory");
        let component = ui.add_sized([ui.available_width(), 30.0], button);
        let dir = self.dir.clone();

        if component.clicked() && dir != "/" {
            self.selected = String::new();
            self.path = dir.clone();

            self.folders.clear();
        };
    }

    fn mini_player_button(&mut self, ui: &mut Ui) {
        let icon = images::get_mini_player();
        let button = Button::new(icon);
        let component = ui.add_sized([30.0, 30.0], button);

        if component.clicked() {
            match self.mini_player_open {
                false => self.mini_player_open = true,
                true => self.mini_player_open = false
            };
        };
    }

    fn play_button(&mut self, ui: &mut Ui, big: bool) {
        let icon = match self.play_state {
            PlayState::Paused | PlayState::Stopped => images::get_play(),
            PlayState::Playing => images::get_pause()
        };

        let size = if big { 60.0 } else { 30.0 };

        let button = Button::new(icon).corner_radius(90);
        let component = ui.add_sized([size, size], button);

        if component.clicked() { self.play(); };
    }

    fn playlist_add_button(&mut self, ui: &mut Ui) {
        let icon = images::get_playlist_add();
        let button = Button::new(icon);
        let component = ui.add_sized([ui.available_width(), 30.0], button);

        if component.clicked() {
            self.selected = String::new();
            self.playlist_add_open = true;
            self.playlist_edit_open = false;
        };
    }

    fn playlist_add_save_button(&mut self, ui: &mut Ui) {
        let button = Button::new("Save");
        let component = ui.add_sized([55.0, 30.0], button);

        if component.clicked() {
            self.playlists.push(Playlist::new(self.temp_playlist_name.clone()));

            let _ = filesys::edit_playlists(self.playlists.clone());

            self.temp_playlist_name = String::new();
            self.selected = String::new();
            self.playlist_add_open = false;
        };
    }

    fn playlist_back_button(&mut self, ui: &mut Ui) {
        let button = Button::new("Back");
        let component = ui.add_sized([ui.available_width(), 30.0], button);

        if component.clicked() {
            self.active_playlist = None;
            self.selected = String::new();
        };
    }

    fn playlist_edit_button(&mut self, ui: &mut Ui, playlist: Playlist) {
        let icon = images::get_playlist();
        let button = Button::new(icon);
        let component = ui.add_sized([30.0, 30.0], button);

        if component.clicked() {
            self.edited_playlist = Some(playlist.clone());
            self.temp_playlist_name = playlist.get_name().unwrap();
            self.selected = String::new();
            self.playlist_add_open = false;
            self.playlist_edit_open = true;
        };
    }

    fn playlist_edit_save_button(&mut self, ui: &mut Ui) {
        let button = Button::new("Save");
        let component = ui.add_sized([55.0, 30.0], button);

        if component.clicked() {
            let mut playlist = Playlist::new(self.temp_playlist_name.clone());

            if let Some(list) = self.edited_playlist.clone() {
                playlist.set_id(list.get_id().unwrap());

                if let Some(sources) = list.get_sources() {
                    for source in sources { playlist.add_source(Some(source)); };
                };
            };

            self.save_playlists(Some(playlist));

            self.temp_playlist_name = String::new();
            self.selected = String::new();
            self.playlist_edit_open = false;
        };
    }
    
    fn playlist_play_button(&mut self, ui: &mut Ui, playlist: Playlist) {
        let icon = images::get_play();
        let button = Button::new(icon).corner_radius(90);
        let component = ui.add_sized([30.0, 30.0], button);

        if component.clicked() {
            self.stop();
            
            self.active_playlist = Some(playlist);
            self.now_playinglist = self.active_playlist.clone();
            self.sorted_playlist = self.active_playlist.clone();

            if let Some(playlist) = &self.now_playinglist {
                if let Some(sources) = playlist.get_sources() {
                    for source in &sources {
                        if *source == self.selected { self.now_playing = source.to_string(); }
                        else {
                            if let Some(first) = playlist.get_source(0) {
                                self.now_playing = first;
                            };
                        };
                    };
                };
            };

            self.play();
        };
    }

    fn playlist_remove_button(&mut self, ui: &mut Ui) {
        let button = Button::new("Delete");
        let component = ui.add_sized([55.0, 30.0], button);

        if component.clicked() {
            let mut playlists: Vec<Playlist> = Vec::new();

            for entry in &self.playlists.clone() {
                if let Some(playlist) = self.edited_playlist.clone() {
                    if entry.get_id() != playlist.get_id() { playlists.push(entry.clone()); };
                };
            };

            self.playlists = playlists.clone();

            let _ = filesys::edit_playlists(self.playlists.clone());

            self.temp_playlist_name = String::new();
            self.selected = String::new();
            self.playlist_edit_open = false;
        };
    }

    fn repeat_button(&mut self, ui: &mut Ui, big: bool) {
        let color = match self.repeat_state {
            RepeatState::All | RepeatState::One => styles::get_button_fill(),
            RepeatState::None => Color32::TRANSPARENT
        };

        let icon = match self.repeat_state {
            RepeatState::All => images::get_repeat(),
            RepeatState::None => images::get_repeat_off(),
            RepeatState::One => images::get_repeat_one()
        };

        let size = if big { 60.0 } else { 30.0 };

        let button = Button::new(icon).corner_radius(90).fill(color);
        let component = ui.add_sized([size, size], button);

        if component.clicked() { self.repeat(); };
    }
    
    fn shuffle_button(&mut self, ui: &mut Ui, big: bool) {
        let color = match self.is_shuffled {
            false => Color32::TRANSPARENT,
            true => styles::get_button_fill()
        };

        let icon = match self.is_shuffled {
            false => images::get_shuffle_off(),
            true => images::get_shuffle()
        };

        let size = if big { 60.0 } else { 30.0 };

        let button = Button::new(icon).corner_radius(90).fill(color);
        let component = ui.add_sized([size, size], button);

        if component.clicked() {
            self.is_shuffled = !self.is_shuffled;
            
            self.shuffle();
        };
    }
    
    fn skip_backward_button(&mut self, ui: &mut Ui, big: bool) {
        let icon = images::get_skip_backward();

        let size = if big { 60.0 } else { 30.0 };

        let button = Button::new(icon).corner_radius(90);
        let component = ui.add_sized([size, size], button);

        if component.clicked() { self.skip_backward(); };
    }
    
    fn skip_forward_button(&mut self, ui: &mut Ui, big: bool) {
        let icon = images::get_skip_forward();

        let size = if big { 60.0 } else { 30.0 };

        let button = Button::new(icon).corner_radius(90);
        let component = ui.add_sized([size, size], button);

        if component.clicked() { self.skip_forward(); };
    }

    fn source_add_button(&mut self, ui: &mut Ui) {
        let icon = images::get_add();
        let button = Button::new(icon);
        let component = ui.add_sized([ui.available_width(), 30.0], button);

        if component.clicked() {
            self.selected = String::new();
            
            self.playlist_dialog.pick_file();
        };
    }
    
    fn source_play_button(&mut self, ui: &mut Ui, source: String) {
        let icon = images::get_play();
        let button = Button::new(icon).corner_radius(90);
        let component = ui.add_sized([30.0, 30.0], button);

        if component.clicked() {
            self.stop();
            
            self.selected = source;
            self.now_playing = self.selected.clone();
            self.now_playinglist = self.active_playlist.clone();
            self.sorted_playlist = self.active_playlist.clone();
            
            self.play();
        };
    }

    fn source_remove_button(&mut self, ui: &mut Ui, source: String) {
        let button = Button::new("-");
        let component = ui.add_sized([30.0, 30.0], button);

        if component.clicked() {
            self.selected = String::new();
            
            if let Some(playlist) = self.active_playlist.clone() {
                let mut list = Playlist::new(playlist.get_name().unwrap());
                
                list.set_id(playlist.get_id().unwrap());

                if let Some(sources) = playlist.get_sources() {
                    for entry in sources {
                        if entry != source { list.add_source(Some(entry)); };
                    };
                };

                self.active_playlist = Some(list.clone());

                self.save_playlists(self.active_playlist.clone());
            };
        };
    }
    
    fn stop_button(&mut self, ui: &mut Ui, big: bool) {
        let icon = images::get_stop();

        let size = if big { 60.0 } else { 30.0 };

        let button = Button::new(icon).corner_radius(90);
        let component = ui.add_sized([size, size], button);

        if component.clicked() {
            self.now_playing = String::new();
            self.selected = String::new();
            self.now_playinglist = None;
        
            self.stop();
        };
    }

    //** UI - Panels **//
    fn add_playlist(&mut self, ui: &mut Ui) {
        ui.add_space(8.5);

        ui.horizontal(|ui| {
            ui.add(Label::new("Name:"));
            ui.add_space(5.5);

            ui.add_sized([ui.available_width(), 30.0],
                TextEdit::singleline(&mut self.temp_playlist_name));
        });

        ui.add_space(5.5);

        ui.columns(9, |columns| {
            self.playlist_add_save_button(&mut columns[3]);
            self.cancel_button(&mut columns[5]);
        });
    }

    fn big_player(&mut self, ctx: &Context) {
        let id = ViewportId::from_hash_of("big_player");

        let viewport = ViewportBuilder::default().with_decorations(false)
            .with_inner_size(Vec2::new(700.0, 241.0));

        if self.big_player_open {
            ctx.show_viewport_immediate(id, viewport, |ctx, class| {
                assert!(class == ViewportClass::Immediate, "Fatal error.");

                CentralPanel::default().frame(Frame::NONE).show(ctx, |ui| {
                    let rect = {
                        let mut rect = ui.max_rect();

                        rect.max.y = rect.min.y + ui.available_height();

                        rect
                    };

                    let panel = ui.interact(rect, Id::new("big_player"), Sense::click_and_drag());

                    if panel.drag_started_by(PointerButton::Primary) {
                        ctx.send_viewport_cmd(ViewportCommand::StartDrag);
                    };

                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.add_space(12.0);

                        let album = data::get_album(Some(self.now_playing.clone()));
                        let artist = data::get_artist(Some(self.now_playing.clone()));
                        let mut title = data::get_title(Some(self.now_playing.clone()));

                        if title == "" {
                            title = filesys::create_from_path(self.now_playing.clone());
                        };

                        self.playing_view(ui, album, artist, title, false);
                    });

                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.add_space(155.0);

                        self.controls(ui, true);
                    });
                });
            });
        };
    }

    fn controls(&mut self, ui: &mut Ui, big: bool) {
        ui.horizontal(|ui| {
            self.repeat_button(ui, big);

            let enabled = if self.now_playing != String::new() { true }
            else { false };

            ui.add_enabled_ui(enabled, |ui| {
                self.skip_backward_button(ui, big);
                self.stop_button(ui, big);
                self.play_button(ui, big);
                self.skip_forward_button(ui, big);
            });

            self.shuffle_button(ui, big);
        });
    }

    fn mini_player(&mut self, ctx: &Context) {
        let id = ViewportId::from_hash_of("mini_player");

        let viewport = ViewportBuilder::default().with_always_on_top().with_decorations(false)
            .with_inner_size(Vec2::new(223.0, 115.0));

        if self.mini_player_open {
            ctx.show_viewport_immediate(id, viewport, |ctx, class| {
                assert!(class == ViewportClass::Immediate, "Fatal error.");

                CentralPanel::default().frame(Frame::NONE).show(ctx, |ui| {
                    let rect = {
                        let mut rect = ui.max_rect();

                        rect.max.y = rect.min.y + ui.available_height();

                        rect
                    };

                    let panel = ui.interact(rect, Id::new("mini_player"), Sense::click_and_drag());

                    if panel.drag_started_by(PointerButton::Primary) {
                        ctx.send_viewport_cmd(ViewportCommand::StartDrag);
                    };

                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.add_space(12.0);

                        let album = data::get_album(Some(self.now_playing.clone()));
                        let artist = data::get_artist(Some(self.now_playing.clone()));
                        let mut title = data::get_title(Some(self.now_playing.clone()));

                        if title == "" {
                            title = filesys::create_from_path(self.now_playing.clone());
                        };

                        self.playing_view(ui, album, artist, title, true);
                    });

                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.add_space(12.0);

                        self.controls(ui, false);
                    });
                });
            });
        };
    }

    fn buffer(&mut self, ui: &mut Ui) {
        ui.add_space(8.5);
        ui.add_space(3.0);
    }
    
    fn cancel_button(&mut self, ui: &mut Ui) {
        let button = Button::new("Cancel");
        let component = ui.add_sized([70.0, 30.0], button);

        if component.clicked() {
            self.temp_playlist_name = String::new();
            self.playlist_add_open = false;
            self.playlist_edit_open = false;
        };
    }

    fn catalog(&mut self, ui: &mut Ui) {
        ui.add_space(8.5);

        ui.columns(3, |columns| {
            let mut can_go_back = false;
            let mut can_go_dir = false;

            if self.folders.len() > 0 { can_go_back = true; };
            if self.path.clone() != self.dir.clone() { can_go_dir = true; };

            columns[0].add_enabled_ui(can_go_dir, |ui| { self.dir_button(ui); });
            columns[1].add_sized([columns[1].available_width(), 30.0], Label::new("Catalog"));
            columns[2].add_enabled_ui(can_go_back, |ui| { self.catalog_back_button(ui); });
        });

        ui.add_space(3.0);
        ui.separator();
        ui.vertical(|ui| { let _ = self.catalog_list(ui); });
    }

    fn edit_playlist(&mut self, ui: &mut Ui) {
        ui.add_space(8.5);

        ui.horizontal(|ui| {
            ui.add(Label::new("Name:"));
            ui.add_space(5.5);

            if let Some(playlist) = &self.edited_playlist {
                ui.add_sized([ui.available_width(), 30.0],
                    TextEdit::singleline(&mut playlist.get_name().unwrap()));
            };
        });

        ui.add_space(5.5);

        ui.columns(9, |columns| {
            self.playlist_edit_save_button(&mut columns[2]);
            self.playlist_remove_button(&mut columns[4]);
            self.cancel_button(&mut columns[6]);
        });

        ui.add_space(5.5);

        let salt = self.edited_playlist.clone().unwrap().get_id();

        ScrollArea::vertical().id_salt(salt).show(ui, |ui| {
            if let Some(playlist) = &self.edited_playlist {
                if let Some(sources) = playlist.get_sources() {
                    for source in sources {
                        ui.set_width(ui.available_width());

                        ui.horizontal(|ui| {
                            self.source_remove_button(ui, source.clone());

                            ui.add_space(5.5);

                            let mut title = data::get_title(Some(source.clone()));

                            if title == String::new() {
                                title = filesys::create_from_path(source.clone());
                            };

                            let button = ui.add(Button::new(title).frame(false)
                                .fill(Color32::TRANSPARENT));

                            if button.clicked() { self.selected = source; };
                        });
                    };
                };
            };
        });
    }

    fn playing_view(&mut self, ui: &mut Ui, album: String, artist: String, title: String, mini: bool) {
        let size = if mini { 12.0 } else { 40.0 };
        let width = if mini { 193.0 } else { 680.0 };

        ui.vertical(|ui| {
            ui.add_sized([width, size], Label::new(
                RichText::new(title.clone()).size(size)).truncate());

            ui.add_sized([width, size], Label::new(
                RichText::new(artist.clone()).size(size)).truncate());

            ui.add_sized([width, size], Label::new(
                RichText::new(album.clone()).size(size)).truncate());
        });
    }
    
    fn player(&mut self, ctx: &Context, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(Vec2::default(), Sense::click());

        if ui.is_rect_visible(rect) {
            ui.add_space(8.5);

            if let Ok(mut player) = self.playback.try_lock() {
                player.set_volume(self.volume as f32 / 100.0);
            };

            ui.columns(2, |columns| {
                columns[0].horizontal(|ui| {
                    self.change_dir_button(ui);
                    self.dir_dialog.update(ctx);

                    if let Some(path) = self.dir_dialog.take_picked() {
                        let path = path.to_path_buf().to_string_lossy().to_string();

                        self.dir = path.clone();
                        self.path = path.clone();

                        let _ = filesys::edit_config(path, self.volume);
                    };

                    ui.add_space(5.5);

                    ui.add_sized([ui.available_width(), 30.0],
                        TextEdit::singleline(&mut self.dir.clone()).interactive(false));
                });

                columns[1].horizontal(|ui| {
                    ui.columns(3, |columns| {
                        columns[0].horizontal(|ui| {
                            ui.add_space(10.0);

                            self.controls(ui, false);
                        });

                        columns[1].add_space(5.0);

                        columns[1].horizontal(|ui| {
                            self.tracking(ui);
                                    
                            ui.add_space(10.0);
                            
                            self.volume(ui);
                        });

                        columns[2].allocate_ui_with_layout(Vec2::ZERO,
                            Layout::right_to_left(Align::RIGHT),
                            |ui| {
                            self.mini_player_button(ui);
                            self.big_player_button(ui);
                        });
                    });
                });
            });

            ui.add_space(6.0);

            ui.horizontal(|ui| {
                let album = data::get_album(Some(self.now_playing.clone()));
                let artist = data::get_artist(Some(self.now_playing.clone()));
                let mut title = data::get_title(Some(self.now_playing.clone()));

                if title == "" {
                    title = filesys::create_from_path(self.now_playing.clone());
                };

                ui.vertical(|ui| {
                    let playing = "Playing:  ".to_owned();
                    let mut alb = "  —  ".to_owned() + &album.clone();
                    let mut art = "  —  ".to_owned() + &artist.clone();

                    if album == String::new() { alb = "".to_string(); };
                    if artist == String::new() { art = "".to_string(); };

                    let line = playing.clone() + &title.clone() + &art.clone() + &alb.clone();

                    if self.now_playing != String::new() { ui.label(line); }
                    else { ui.label(playing); };
                });
                
                self.update_playback();
            });

            self.setup_stopwatch();

            ui.add_space(1.5);

            ui.ctx().request_repaint_after(Duration::from_millis(10));
        };

        response
    }
    
    fn playlists(&mut self, ctx: &Context, ui: &mut Ui) {
        ui.add_space(8.5);

        let back: bool;

        if self.active_playlist.is_some() { back = true; }
        else { back = false; };

        ui.columns(3, |columns| {
            if !back { self.playlist_add_button(&mut columns[0]); }
            else { self.source_add_button(&mut columns[0]); };

            let mut label = "Playlists".to_string();

            if back { label = "Sources".to_string(); };
            
            columns[1].add_sized([columns[1].available_width(), 30.0], Label::new(label));
            columns[2].add_enabled_ui(back, |ui| { self.playlist_back_button(ui); });
        });

        ui.separator();
        ui.vertical(|ui| { let _ = self.playlists_list(ctx, ui); });
    }
    
    fn viewer(&mut self, ui: &mut Ui) { //TODO video playback
        ui.add_space(8.5);

        ui.add_space(3.0);
    }

    //** UI - Scroll Areas **//
    fn catalog_list(&mut self, ui: &mut Ui) -> Result<()> {
        let mut entries = filesys::read_dir(self.path.clone())?.map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>>>()?;

        entries.sort();

        if !self.pseudo_playlist.is_some() { let _ = self.load_sources(self.dir.clone()); };

        ScrollArea::vertical().auto_shrink(false).id_salt("catalog").show(ui, |ui| {
            if self.dir != "/" {
                for entry in &entries {
                    let file = entry.as_path().file_name().unwrap().display().to_string();
                    let button = ui.add(Button::new(&file).frame(false).fill(Color32::TRANSPARENT));
                    
                    if button.clicked() {
                        if entry.as_path().is_file() {
                            self.selected = entry.as_path().display().to_string();

                            match self.play_state {
                                PlayState::Paused | PlayState::Stopped => {
                                    self.now_playing = self.selected.clone();
                                    self.now_playinglist = self.pseudo_playlist.clone();
                                    self.sorted_playlist = self.pseudo_playlist.clone();
                                },

                                PlayState::Playing => {}
                            };
                        } else {
                            let new_path = entry.to_path_buf().display().to_string();

                            self.path = new_path.clone();

                            if new_path != self.dir.clone() { self.folders.push(new_path.clone()); };

                            self.pseudo_playlist = None;

                            let _ = self.load_sources(new_path);
                        };
                    };
                };
            };
        });

        Ok(())
    }

    fn playlists_list(&mut self, ctx: &Context, ui: &mut Ui) -> Result<()> {
        let entries = self.playlists.clone();

        if let Some(path) = self.playlist_dialog.take_picked() {
            if path.is_file() {
                let file_path = path.as_path().display().to_string();

                if let Some(mut playlist) = self.active_playlist.clone() {
                    playlist.add_source(Some(file_path));

                    self.active_playlist = Some(playlist.clone());

                    self.save_playlists(Some(playlist));
                };
            };
        };

        ScrollArea::vertical().auto_shrink(false).id_salt("playlists").show(ui, |ui| {
            if self.active_playlist.is_some() {
                self.playlist_dialog.update(ctx);
                
                if let Some(playlist) = &self.active_playlist {
                    if let Some(sources) = playlist.get_sources() {
                        for source in sources {
                            ui.horizontal(|ui| {
                                ui.add_space(5.5);

                                self.source_play_button(ui, source.clone());

                                ui.add_space(5.5);

                                let mut title = data::get_title(Some(source.clone()));

                                if title == String::new() {
                                    title = filesys::create_from_path(source.clone());
                                };

                                let button = ui.add(Button::new(title).frame(false)
                                    .fill(Color32::TRANSPARENT));

                                if button.clicked() { self.selected = source; };
                            });
                        };
                    };
                };
            } else {
                for entry in entries {
                    ui.horizontal(|ui| {
                        self.playlist_edit_button(ui, entry.clone());
                        
                        ui.add_space(5.5);

                        self.playlist_play_button(ui, entry.clone());

                        ui.add_space(5.5);

                        let button = ui.add(Button::new(entry.get_name().unwrap()).frame(false)
                            .fill(Color32::TRANSPARENT));

                        if button.clicked() {
                            self.active_playlist = Some(entry.clone());
                            self.sorted_playlist = self.active_playlist.clone();
                        };
                    });
                };
            };
        });

        Ok(())
    }

    //** UI - Sliders **//
    fn tracking(&mut self, ui: &mut Ui) {
        ui.label(RichText::new(data::format_duration(self.get_elapsed_time())).size(16.0));

        let mut slider_value = self.elapsed_time.as_secs_f32();

        let slider = Slider::new(&mut slider_value, 0.0..=self.total_time.as_secs_f32())
            .show_value(false);

        let slider_response = ui.add(slider);

        if slider_response.drag_started() {
            self.play_state = PlayState::Paused;

            self.pause();
        };

        if slider_response.dragged() {
            self.elapsed_time = Duration::from_secs_f32(slider_value);

            if self.now_playing != String::new() { self.play(); };
        };

        ui.label(RichText::new(data::format_duration(self.total_time)).size(16.0));
    }

    fn volume(&mut self, ui: &mut Ui) {
        let mut volume = self.volume;

        let volume_icon = if volume > 70 { images::get_volume_up() }
            else if volume > 40 { images::get_volume_down() }
            else if volume > 0 { images::get_volume_none() }
            else { images::get_volume_mute() };

        ui.menu_button(volume_icon, |ui| {
            let vol = ui.add(Slider::new(&mut volume, 0..=100).vertical());

            if vol.dragged() { let _ = filesys::edit_config(self.dir.clone(), volume); };
        });

        self.volume = volume;
    }
}

impl App for Main {
    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let margin = Margin::symmetric(8, 0);
        let frame = Frame::default().inner_margin(margin).fill(Color32::from_rgb(25, 10, 35));
        let height = 80.0;
        let width = 310.0;

        styles::set_styles(ctx);

        self.big_player(ctx);
        self.mini_player(ctx);

        TopBottomPanel::top("player").frame(frame).resizable(false).show(ctx, |ui| {
            self.player(ctx, ui);
        });

        TopBottomPanel::bottom("buffer").frame(frame).resizable(false).min_height(height / 3.0)
            .max_height(height).show(ctx, |ui| { self.buffer(ui); });

        SidePanel::left("catalog").frame(frame).resizable(false).min_width(width)
            .max_width(width).show(ctx, |ui| { self.catalog(ui); });

        SidePanel::right("playlists").frame(frame).resizable(false).min_width(width)
            .max_width(width).show(ctx, |ui| { self.playlists(ctx, ui); });

        if self.playlist_add_open {
            TopBottomPanel::bottom("add_edit").frame(frame).resizable(false).min_height(height)
                .max_height(height).show(ctx, |ui| { self.add_playlist(ui); });
        };

        if self.playlist_edit_open {
            CentralPanel::default().frame(frame).show(ctx, |ui| { self.edit_playlist(ui); });
        } else { CentralPanel::default().frame(frame).show(ctx, |ui| { self.viewer(ui); }); };
    }
}

struct Main {
    // Booleans
    is_shuffled: bool,
    big_player_open: bool,
    mini_player_open: bool,
    playlist_add_open: bool,
    playlist_edit_open: bool,
    start_playback: bool,
    stop_playback: bool,

    // Dialogs
    dir_dialog: FileDialog,
    playlist_dialog: FileDialog,

    // Directories
    dir: String,
    path: String,
    folders: Vec<String>,

    // Files
    playback: Arc<Mutex<Playback>>,

    // Misc.
    volume: i32,
    temp_playlist_name: String,

    // Playlists
    active_playlist: Option<Playlist>,
    edited_playlist: Option<Playlist>,
    now_playinglist: Option<Playlist>,
    pseudo_playlist: Option<Playlist>,
    sorted_playlist: Option<Playlist>,
    playlists: Vec<Playlist>,

    // Sources
    now_playing: String,
    selected: String,

    // States
    play_state: PlayState,
    repeat_state: RepeatState,
    
    // Time
    elapsed_time: Duration,
    start_time: Duration,
    total_time: Duration,
    stopwatch_instant: Option<Instant>
}

pub fn main() {
    let icon = "src/assets/icon.ico";
    let name = "ComRad: Compact Radio";
    let mut viewport = ViewportBuilder::default().with_maximized(true);

    if filesys::exists(icon.to_string()) {
        let icon_data: IconData = IconData {
            rgba: open(icon).unwrap().to_rgba8().into_raw(),
            height: 16,
            width: 16
        };

        viewport = viewport.with_icon(Arc::new(icon_data));
    };

    let options = NativeOptions {
        viewport,

        ..Default::default()
    };

    let _ = filesys::create_config();
    let _ = filesys::create_playlists();

    let _ = run_native(&name, options, Box::new(|cc| {
        install_image_loaders(&cc.egui_ctx);

        Ok(Box::new(Main::new(cc)))
    }));
}