/// Main application state
#[derive(Debug)]
pub struct AppState {
    pub viewport: ViewportState,
    pub project: ProjectState,
    pub camera: CameraState,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            viewport: ViewportState::default(),
            project: ProjectState::new("Untitled".to_string()),
            camera: CameraState::default(),
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
