use rocket::serde::Deserialize;

use super::database::Task;

/// Enum describing a command sent by the client. The command is sent in JSON
/// format and deserialised to this enum.
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(tag = "command")]
pub enum Command<'r> {
    /// Client requests an image, please.
    ///
    /// See [get_image](crate::database::get_image) for more
    /// information.
    GetImage { task: Task },

    /// Client requests information on the classes involved in the classification task.
    ///
    /// See [get_classes](crate::database::get_classes) for more
    /// information.
    GetClassifications,

    /// Client requests information on the regression task with the name indicated by `kind`.
    ///
    /// See [get_regression](crate::database::get_regression) for more
    /// information.
    GetRegression { kind: &'r str },

    /// The user has completed a classification annotation, and the client requests
    /// the information be stored.
    ///
    /// See [store_classification](crate::database::store_classification) for more
    /// information.
    StoreClassificationResult {
        cid: i64,
        iid: i64,
        sid: &'r str,
        tt: f64,
    },

    /// The user has completed a regression annotation, and the client requests
    /// the information be stored.
    ///
    /// See [store_multilabel_classification](crate::database::store_multilabel_classification) for more
    /// information.
    StoreMultilabelClassificationResult {
        cid: i64,
        iid: i64,
        sid: &'r str,
        tt: f64,
    },

    /// The user has completed a regression annotation and the client
    /// requests the information be stored.
    ///
    /// See [store_regression](crate::database::store_regression) for more
    /// information.
    StoreRegressionResult {
        rid: i64,
        lid: i64,
        mid: i64,
        sid: &'r str,
        tt: f64,
    },

    /// Ask for a summary of the database contents - how many images, how many annotations, etc
    GetDatasetSummary,
}
