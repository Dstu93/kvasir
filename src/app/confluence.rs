

use std::io::Error;

/// manages communication with confluence
pub trait ConfluenceConnector {

    fn add_page(&self, page: Page) -> Result<(),Error>;
    fn update_page(&self, page: Page) -> Result<(),Error>;
    fn get_page_by_id(&self, id: i64) -> Result<Option<Page>,Error>;
    fn get_page_by_title(&self, title: &str) -> Result<Option<Page>,Error>;
    fn search(&self, content: &str) -> Result<Option<Vec<Page>>, Error>;

}

pub struct ConfluenceConnectorImpl{

}

pub struct Page{

}