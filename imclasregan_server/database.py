import sqlite3
import os

import numpy as np


class _Database:

    def __init__(self):
        self.conn = sqlite3.connect('database.db')
        self.cursor = self.conn.cursor()

    def __enter__(self):
        return self
    
    def __exit__(self, _, __, ___):
        self.conn.close()


def get_image() -> dict:
    with _Database() as db:
        db.cursor.execute('SELECT * FROM IMAGES ORDER BY RANDOM() LIMIT 1;')
        res = db.cursor.fetchone()
    iid, name, data, width, height = res
    im = np.full((height, width, 4), 255, np.uint8)
    im[:, :, :3] = np.ndarray((height, width, 3), np.uint8, data)

    data = [int(b) for b in im.flatten()]

    assert len(data) == width*height*4, (len(data), width, height, width*height*4)
    return dict(iid=iid, name=name, data=data, width=width, height=height)


def get_classes() -> dict:
    with _Database() as db:
        db.cursor.execute('SELECT * FROM CLASSIFICATIONS;')
        results = db.cursor.fetchall()
    rv = dict(classes=[])
    for (cid, name, description) in results:
        classdata = dict(
            cid=cid, name=name, description=description
        )
        rv['classes'].append(classdata)
    return rv


def get_regression(kind: str) -> dict:
    with _Database() as db:
        db.cursor.execute('SELECT * FROM REGRESSIONS WHERE NAME=?;', (kind,))
        result = db.cursor.fetchone()
    if not result:
        return dict(error=True, message=f'did not understand regression kind: {kind}')
    else:
        id, name, description, in_a_sentence = result
        return dict(rid=id, name=name, description=description, in_a_sentence=in_a_sentence)


def store_result(kind: str, **kwargs) -> dict:
    if kind == 'classification':
        return store_classification_result(**kwargs)
    elif kind == 'regression':
        return store_regression_result(**kwargs)


def store_classification_result(iid: int, cid: int, sid: str, tt: float) -> dict:
    with _Database() as db:
        db.cursor.execute(
            'INSERT INTO CLASSIFICATIONRESULTS (IMAGE_ID, CLASS_ID, SESSION_ID, TIME_TAKEN) VALUES (?, ?, ?, ?);',
            (iid, cid, sid, tt))


def store_regression_result(rid: int, lid: int, mid: int, sid: str, tt: float) -> dict:
    with _Database() as db:
        db.cursor.execute(
            'INSERT INTO REGRESSIONRESULTS (REGRESSION_ID, IMAGE_ID_LESS, IMAGE_ID_MORE, SESSION_ID, TIME_TAKEN) VALUES (?, ?, ?, ?, ?);',
            (rid, lid, mid, sid, tt))