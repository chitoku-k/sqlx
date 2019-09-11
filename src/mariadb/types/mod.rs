use super::protocol::{FieldType, ParameterFlag};
use crate::{mariadb::MariaDb, types::TypeMetadata};

#[derive(Debug)]
pub struct MariaDbTypeMetadata {
    pub field_type: FieldType,
    pub param_flag: ParameterFlag,
}

impl TypeMetadata for MariaDb {
    type TypeMetadata = MariaDbTypeMetadata;
}
