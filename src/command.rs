use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(tag = "command")]
pub enum Command<'r> {
    GetImage,
    GetClassifications,
    GetRegression {
        kind: &'r str,
    },
    StoreClassificationResult {
        cid: i64,
        iid: i64,
        sid: &'r str,
        tt: f64,
    },
    StoreRegressionResult {
        rid: i64,
        lid: i64,
        mid: i64,
        sid: &'r str,
        tt: f64,
    },
}
