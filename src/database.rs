use crate::reply::{ClassData, Reply};

/// Insert a regression result into the server. In a regression task, the user
/// is asked to choose an image from a pair which is more representative of a
/// given *quality* (e.g. in focus).
///
/// A regression result consists of
///  - `rid` - the id specifying the regression,
///  - `lid` - the id of the image which is less *quality*,
///  - `mid` - the id of the image which is more *quality*,
///  - `sid` - a UUID indicating which user session the result came from, and
///  - `tt` - the time taken in milliseconds for the user to make the choice.
///
/// Returns [Reply::Ok]
///
/// # Panics
/// Panics if the database cannot be opened.
pub fn store_regression(rid: i64, lid: i64, mid: i64, sid: &str, tt: f64) -> Reply {
    let conn = sqlite::Connection::open("./database.db").unwrap();
    let mut statement = conn
        .prepare("INSERT INTO REGRESSIONRESULTS (REGRESSION_ID, IMAGE_ID_LESS, IMAGE_ID_MORE, SESSION_ID, TIME_TAKEN) VALUES (?, ?, ?, ?, ?);")
        .unwrap();

    statement.bind((1, rid)).unwrap();
    statement.bind((2, lid)).unwrap();
    statement.bind((3, mid)).unwrap();
    statement.bind((4, sid)).unwrap();
    statement.bind((4, tt)).unwrap();

    let _ = statement.next();

    Reply::Ok
}

/// Insert a classification result into the server. In a classification task, the
/// user is asked to choose a fitting label for a given image.
///
/// A regression result consists of
///  - `cid` - the id specifying the chosen class label,
///  - `iid` - the id of the image,
///  - `sid` - a UUID indicating which user session the result came from, and
///  - `tt` - the time taken in milliseconds for the user to make the choice.
///
/// Returns [Reply::Ok]
///
/// # Panics
/// Panics if the database cannot be opened.
pub fn store_classification(cid: i64, iid: i64, sid: &str, tt: f64) -> Reply {
    let conn = sqlite::Connection::open("./database.db").unwrap();
    let mut statement = conn
        .prepare("INSERT INTO CLASSIFICATIONRESULTS (SESSION_ID, CLASS_ID, IMAGE_ID, TIME_TAKEN) VALUES (?, ?, ?, ?);")
        .unwrap();

    statement.bind((1, sid)).unwrap();
    statement.bind((2, cid)).unwrap();
    statement.bind((3, iid)).unwrap();
    statement.bind((4, tt)).unwrap();

    let _ = statement.next();

    Reply::Ok
}

/// Get an random image from the database and return it as [Reply::Image] if
/// there are images available, or as [Reply::Error] otherwise.
///
/// # Panics
/// Panics if the database cannot be opened.
pub fn get_image() -> Reply {
    let conn = sqlite::Connection::open("./database.db").unwrap();
    let mut statement = conn
        .prepare("SELECT * FROM IMAGES ORDER BY RANDOM() LIMIT 1;")
        .unwrap();

    if let Ok(sqlite::State::Row) = statement.next() {
        let data_rgb = statement.read::<Vec<u8>, _>("DATA").unwrap();
        let mut data_rgba: Vec<u8> = Vec::with_capacity(data_rgb.len() / 3 * 4);
        for i in 0..data_rgb.len() / 3 {
            for j in 0..3usize {
                data_rgba.push(data_rgb[i * 3 + j]);
            }
            data_rgba.push(255);
        }

        assert_eq!(data_rgba.len() % 4, 0);
        assert_eq!(data_rgb.len(), data_rgba.len() * 3 / 4);

        Reply::Image {
            iid: statement.read::<i64, _>("ID").unwrap(),
            width: statement.read::<i64, _>("WIDTH").unwrap(),
            height: statement.read::<i64, _>("HEIGHT").unwrap(),
            data: data_rgba,
        }
    } else {
        Reply::Error {
            message: "no images?!".to_string(),
        }
    }
}

/// Gets information on the class labels for the classification task and
/// returns it as [Reply::Classifications].
///
/// # Panics
/// Panics if the database cannot be opened.
pub fn get_classes() -> Reply {
    let conn = sqlite::Connection::open("./database.db").unwrap();
    let mut classes: Vec<ClassData> = Vec::new();
    let mut statement = conn.prepare("SELECT * FROM CLASSIFICATIONS;").unwrap();
    while let Ok(sqlite::State::Row) = statement.next() {
        let class = ClassData {
            cid: statement.read::<i64, _>("ID").unwrap(),
            name: statement.read::<String, _>("NAME").unwrap(),
            description: statement.read::<String, _>("DESCRIPTION").unwrap(),
        };
        classes.push(class);
    }
    Reply::Classifications(classes)
}

/// Gets information on the regression task specified by the name `kind`. Result
/// is returned as [Reply::Regression] if regression is found, or [Reply::Error] otherwise.
///
/// # Panics
/// Panics if the database cannot be opened.
pub fn get_regression<'a>(kind: &'a str) -> Reply {
    let conn = sqlite::Connection::open("./database.db").unwrap();
    let mut statement = conn
        .prepare("SELECT * FROM REGRESSIONS WHERE NAME=?;")
        .unwrap();
    statement.bind((1, kind)).unwrap();

    if let Ok(sqlite::State::Row) = statement.next() {
        Reply::Regression {
            rid: statement.read::<i64, _>("ID").unwrap(),
            name: statement.read::<String, _>("NAME").unwrap(),
            description: statement.read::<String, _>("DESCRIPTION").unwrap(),
            in_a_sentence: statement.read::<String, _>("IN_A_SENTENCE").unwrap(),
        }
    } else {
        Reply::Error {
            message: format!("Could not find information on regression task '{kind}'"),
        }
    }
}
