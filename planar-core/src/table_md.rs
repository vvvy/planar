use serde_derive::{Serialize, Deserialize};

#[derive(Debug)]
pub struct TableDefKey<'r> {
    pub schema: &'r str,
    pub table: &'r str
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParquetMetadata {

} 

#[derive(Serialize, Deserialize, Debug)]
pub struct TableDef {
    pub location: String,
    pub pmd: ParquetMetadata
}
