import os
from glob import glob

import cv2

from imclasregan_server.database import _Database


def add_image(name: str):
    image = cv2.imread(name, cv2.IMREAD_COLOR)
    image = cv2.cvtColor(image, cv2.COLOR_BGR2RGB)
    assert len(image.shape) == 3
    assert image.shape[2] == 3
    height, width = image.shape[:2]
    data = image.data

    with _Database() as db:
        db.cursor.execute(
            'INSERT INTO IMAGES(name, data, width, height) VALUES (?, ?, ?, ?)',
            (name, data, width, height))
        db.conn.commit()


def remove_from_inbox(fn: str):
    os.rename(fn, fn.replace('inbox', 'processed_images'))


if __name__ == '__main__':
    for fn in glob('inbox/*'):
        add_image(fn)
        remove_from_inbox(fn)