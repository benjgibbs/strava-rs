//! Strava resource types for API v3
//!
//! Most member types are a 1:1 mapping except enumerations, resources, and
//! times
//!

use time::Timespec;

pub mod enums;
use resources::enums::ResourceState;
use resources::enums::FrameType;
use resources::enums::ActivityType;
use resources::enums::ClubType;
use resources::enums::SportType;
use resources::enums::WorkoutType;


/// Gear type able to represent bikes/shoes/etc.
///
/// Represents equipment used during an activity. Gear has summary and detail
/// representations.
///
/// See: http://strava.github.io/api/v3/gear/
#[allow(dead_code)]
pub struct Gear {
    id: String,
    primary: bool,
    name: String,
    distance: f32,
    brand_name: String,
    model_name: String,
    frame_type: FrameType,
    description: String,
    resource_state: ResourceState
}



/// Clubs represent groups of athletes on Strava.
///
/// They can be public or private. Only members of private clubs can access
/// their details. The object is returned in summary or detailed
/// representations.
///
/// See: http://strava.github.io/api/v3/clubs/
#[allow(dead_code)]
pub struct Club {
    id: i32,
    resource_state: ResourceState,
    name: String,
    profile_medium: String,
    profile: String,
    description: String,
    club_type: ClubType,
    sport_type: SportType,
    city: String,
    state: String,
    country: String,
    private: bool,
    member_count: i32
}

/// Athletes are Strava users, Strava users are athletes.
///
/// The object is returned in detailed, summary or meta representations.
///
/// See: http://strava.github.io/api/v3/athlete/
#[allow(dead_code)]
pub struct Athlete {
    id: i32,
    resource_state: ResourceState,
    firstname: String,
    lastname: String,
    profile_medium: String,
    profile: String,
    city: String,
    state: String,
    country: String,
    sex: String,
    friend: String,
    follower: String,
    premium: bool,
    created_at: Timespec,
    updated_at: Timespec,
    approve_followers: bool,
    follower_count: i32,
    friend_count: i32,
    mutual_friend_count: i32,
    date_preference: String,
    measurement_preference: String,
    email: String,
    ftp: i32,
    weight: f32,
    clubs: Vec<Club>,
    shoes: Vec<Gear>,
    bikes: Vec<Gear>
}

pub struct ActivityMap;


pub struct SegmentEffort;
pub struct Split;

#[allow(dead_code)]
pub struct Activity {
    id: i32,
    resource_state: ResourceState,
    external_id: String,
    upload_id: i32,
    athlete: Athlete,
    name: String,
    description: String,
    distance: f32,
    moving_time: i32,
    elapsed_time: i32,
    total_elevation_gain: f32,
    activity_type: ActivityType,
    start_date: Timespec,
    start_date_local: Timespec,
    timezone: String,
    start_latlng: (f32, f32),
    end_latlng: (f32, f32),
    location_city: String,
    location_state: String,
    location_country: String,
    achievement_count: i32,
    kudos_count: i32,
    comment_count: i32,
    athlete_count: i32,
    photo_count: i32,
    map: ActivityMap,
    trainer: bool,
    commute: bool,
    manual: bool,
    private: bool,
    flagged: bool,
    workout_type: i32,
    gear_id: String,
    gear: Gear,
    average_speed: f32,
    max_speed: f32,
    average_cadence: f32,
    average_temp: f32,
    average_watts: f32,
    weighted_average_watts: i32,
    kilojoules: f32,
    device_watts: bool,
    max_heartrate: i32,
    calories: f32,
    truncated: i32,
    has_kudoed: bool,
    segment_efforts: Vec<SegmentEffort>,
    splits_metric: Vec<Split>,
    splits_standard: Vec<Split>,
    best_efforts: Vec<SegmentEffort>
}
