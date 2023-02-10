use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ClassData {
    pub cid: i64,
    pub name: String,
    pub description: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(untagged)]
pub enum Reply {
    Ok,
    Error {
        message: String,
    },
    Image {
        iid: i64,
        data: Vec<u8>,
        width: i64,
        height: i64,
    },
    Regression {
        rid: i64,
        name: String,
        description: String,
        in_a_sentence: String,
    },
    Classifications(Vec<ClassData>),
}
