use rocket::serde::Serialize;

/// Struct containing information about a class for classification. Multiple
/// classes are specified in the database and the user is asked to choose between
/// them.
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ClassData {
    pub cid: i64,
    pub name: String,
    pub description: String,
}

/// Enum struct containing different kinds of reply that the server can send.
/// The response can be JSON serialised by serde (and indeed will be, when sent
/// back to the client).
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(untagged)]
pub enum Reply {
    /// Contains no information, just indicates nothing went wrong.
    Ok,

    /// Indicates something went wrong: [message](Reply::Error::message) gives some information.
    Error { message: String },

    /// Image data returned by server. This includes the [ID](Reply::Image::iid)
    /// of the image as specified in the database, the [bytes](Reply::Image::data)
    /// making up the image, and the [width](Reply::Image::width) and
    /// [height](Reply::Image::height) of the image. The data is in `RGB888` format.
    Image {
        iid: i64,
        data: Vec<u8>,
        width: i64,
        height: i64,
    },

    /// Information about a regression task. Includes the
    /// [ID](Reply::Regression::rid), [display name](Reply::Regression::name), a
    /// [description](Reply::Regression::description), and the adjective version
    /// of the name of the quality being regressed as
    /// [used in a sentence](Reply::Regression::in_a_sentence). For example, if
    /// the quality being regressed is "focus" or "burriness" then this would be
    /// "in-focus" as in, the user is asked "Select the image which is more **in-focus**".
    Regression {
        rid: i64,
        name: String,
        description: String,
        in_a_sentence: String,
    },

    /// Information about a classification task or, rather, information on the
    /// classes used to classify the images. This is a vector of [ClassData].
    Classifications(Vec<ClassData>),
}
