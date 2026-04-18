mod delete_manufacturer;
mod get_all_manufacturers;
mod get_manufacturer_by_id;
mod post_manufacturer;
mod put_manufacturer;

pub use delete_manufacturer::handle as delete_manufacturer;
pub use get_all_manufacturers::handle as get_all_manufacturers;
pub use get_manufacturer_by_id::handle as get_manufacturer_by_id;
pub use post_manufacturer::handle as post_manufacturer;
pub use put_manufacturer::handle as put_manufacturer;
