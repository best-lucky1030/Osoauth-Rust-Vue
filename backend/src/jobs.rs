// modules this file will use
use lazy_static::lazy_static;
use std::fmt;
use std::string::ToString;
use std::collections::HashMap;

use oso::*;

// require codes to be executed at runtime in order to be initialized
lazy_static! {
//  HashMap will be evaluated once and stored behind a global static reference
    pub static ref DB: HashMap<usize, Job> = {
        let mut db = HashMap::with_capacity(4);
// Insert completed jobs into database
        db.insert(1, Job::new("Mr. John Cole", "Treating Patient From Flu", "Flat 2, Eastern New York", 
  "alice@example.com"));
        db.insert(2, Job::new("Ms. Gwen Hade", "Helping patient stay fit", "Flat 7, Eastern New York", 
  "alice@example.com"));
        db.insert(3, Job::new("Mr. Fahad Abdul", "Providing Vaccine for patient", "Flat 20, Western New York", 
  "alice@example.com"));
        db.insert(4, Job::new("Mrs. Jenny Cole", "Providing Cancer Drugs for patient", "Flat 2, Eastern New york", 
  "alice@example.com"));
        db
    };
}

// Declare public struct for DB structure
#[derive(PolarClass, Debug, Clone)]
pub struct Job {
    pub name: String,
    pub description: String,
    pub address: String,
    #[polar(attribute)]
    pub submitted_by: String,
}

// Define implementaion of record type
impl Job {
    pub fn new(name: &str, description: &str, address: &str, submitted_by: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            address: address.to_string(),
            submitted_by: submitted_by.to_string(),
        }
    }
}

// Format to display jobs
impl fmt::Display for Job {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Job(name='{}', description='{}', address='{}', submitted_by='{}')",
            self.name, &self.description, &self.address, &self.submitted_by
        )
    }
}
