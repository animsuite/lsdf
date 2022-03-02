use std::path::PathBuf;

use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

/// Defines a single set of frames making up a single clip
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimatedClip {
    /// The name of the clip
    pub name: String,
    /// The paths to each frame, in order
    pub frames: Vec<PathBuf>,
    // pub ground_lock: Option<FormatComponentPointer<GroundLockDeclaration>>,
}

/// The definition of an animation file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationDeclaration {

    /// Name of the animation.
    pub name: String,
    /// Animation framerate
    pub frames_per_second: u32,
    /// The clips that make up the animation.
    pub clips: Vec<AnimatedClip>,

    /// This is used to keep track of where the file was loaded from
    #[serde(skip)]
    loaded_path: PathBuf,
}

impl AnimationDeclaration {
    /// Load an animation from a file
    pub fn load(path: &PathBuf) -> Result<Self, ron::Error> {
        let file = std::fs::read_to_string(path)?;
        let mut decoded: Self = ron::from_str(&file)?;
        decoded.loaded_path = path.clone();
        Ok(decoded)
    }

    /// Save an animation to a file
    pub fn save_as(&self, path: &PathBuf) -> Result<(), ron::Error> {
        let encoded = ron::ser::to_string_pretty(self, PrettyConfig::default())?;
        std::fs::write(path, encoded)?;
        Ok(())
    }

    /// Save an animation to the file it was loaded from
    pub fn save(&self) -> Result<(), ron::Error> {
        self.save_as(&self.loaded_path)
    }
}
