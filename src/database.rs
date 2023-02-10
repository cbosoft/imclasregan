use crate::reply::{ClassData, Reply};

pub fn store_regression(rid: i64, lid: i64, mid: i64, sid: &str, tt: f64) -> Reply {
    let conn = sqlite::Connection::open("./database.db").unwrap();
    let mut statement = conn
        .prepare("INSERT INTO CLASSIFICATIONRESULTS (REGRESSION_ID, IMAGE_ID_LESS, IMAGE_ID_MORE, SESSION_ID, TIME_TAKEN) VALUES (?, ?, ?, ?, ?);")
        .unwrap();

    statement.bind((1, rid)).unwrap();
    statement.bind((2, lid)).unwrap();
    statement.bind((3, mid)).unwrap();
    statement.bind((4, sid)).unwrap();
    statement.bind((4, tt)).unwrap();

    Reply::Ok
}

pub fn store_classification(cid: i64, iid: i64, sid: &str, tt: f64) -> Reply {
    let conn = sqlite::Connection::open("./database.db").unwrap();
    let mut statement = conn
        .prepare("INSERT INTO CLASSIFICATIONRESULTS (SESSION_ID, CLASS_ID, IMAGE_ID, TIME_TAKEN) VALUES (?, ?, ?, ?);")
        .unwrap();

    statement.bind((1, sid)).unwrap();
    statement.bind((2, cid)).unwrap();
    statement.bind((3, iid)).unwrap();
    statement.bind((4, tt)).unwrap();

    Reply::Ok
}

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
            message: "no images?!".to_string(),
        }
    }
}
