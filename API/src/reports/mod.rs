mod controller;
pub(crate) mod model;
mod dto;
mod service;

pub use controller::get_report_by_category;
pub use controller::get_kpis;
pub use model::ReportByCategory;

