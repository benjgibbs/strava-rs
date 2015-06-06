//! Strava API
//!

extern crate hyper;
extern crate rustc_serialize;


mod http;

pub mod resources;
pub mod accesstoken;
pub mod error;
pub mod segment;
pub mod paginate;

pub use resources::Athlete;
pub use accesstoken::AccessToken;
pub use resources::ResourceState;
