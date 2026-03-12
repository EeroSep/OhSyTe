use crate::events::Event;

pub mod test_provider;
pub mod text_file_provider;
pub mod csv_file_provider;
pub mod sqlite_provider;
pub mod web_provider;

pub use test_provider::TestProvider;
pub use text_file_provider::TextFileProvider;
pub use csv_file_provider::CsvFileProvider;
pub use sqlite_provider::SQLiteProvider;
pub use web_provider::WebProvider;

pub trait EventProvider {
    fn name(&self) -> String;
    fn get_events(&self, events: &mut Vec<Event>);
}
