import sqlite3
import os

import numpy as np


class _Database:

    def __init__(self):
        print(os.path.abspath('.'))
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


def store_result(iid: int, cid: int, sid: str, tt: float) -> dict:
    with _Database() as db:
        db.cursor.execute(
            'INSERT INTO RESULTS (IMAGE_ID, CLASS_ID, SESSION_ID, TIME_TAKEN) VALUES (?, ?, ?, ?);',
            (iid, cid, sid, tt))