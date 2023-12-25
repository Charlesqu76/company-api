use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub name: String,
    pub name_eng: String,
    pub t_eng: String,
    pub image: String,
    pub introduction: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DelModel {
    pub t_eng: String,
    pub name_eng: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Types {
    pub t: String,
    pub t_eng: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DelTypes {
    pub t_eng: String,
}
