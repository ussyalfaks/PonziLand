pub mod event;
pub mod events;
pub mod land;
pub mod land_stake;

pub type Database = sqlx::PgPool;

pub use event::Repository as EventRepository;
pub use land::Repository as LandRepository;
pub use land_stake::Repository as LandStakeRepository;
