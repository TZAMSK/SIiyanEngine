use crate::selection::SelectionState;

#[derive(Debug)]
pub struct AppState {
    pub viewport: ViewportState,
    pub project: ProjectState,
    pub camera: CameraState,
    pub selection: SelectionState,
    pub console: Console,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            viewport: ViewportState::default(),
            project: ProjectState::new("Untitled".to_string()),
            camera: CameraState::default(),
            selection: SelectionState::new(),
            console: Console::new(),
        }
    }
}

impl AppState {
    pub fn update(&mut self) {
        // Update any time-dependent state
    }
}

#[derive(Debug, Default)]
pub struct ViewportState {
    pub width: u32,
    pub height: u32,
    pub zoom: f32,
    pub pan: [f32; 2],
    pub hovered: bool,
    pub rect: [f32; 4],
}

#[derive(Debug)]
pub struct ProjectState {
    pub name: String,
    pub modified: bool,
    pub path: Option<String>,
}

impl ProjectState {
    pub fn new(name: String) -> Self {
        Self {
            name,
            modified: false,
            path: None,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CameraState {
    pub position: [f32; 3],
    pub target: [f32; 3],
}

#[derive(Debug)]
pub struct Console {
    pub log: String,
}

impl Console {
    pub fn new() -> Self {
        Self {
            log: String::from("Ready.\n"),
        }
    }

    pub fn append(&mut self, content: &str) {
        self.log.push_str(content);
        self.log.push('\n');
    }

    pub fn clear(&mut self) {
        self.log.clear();
        self.log.push_str("Ready.");
    }
}
