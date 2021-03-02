/// The main engine
mod engine;

/// The rendering methods
mod render {
    pub mod ambient;
    pub mod diffusion;
    pub mod intersection;
    pub mod specularity;
}

pub use engine::Engine;
