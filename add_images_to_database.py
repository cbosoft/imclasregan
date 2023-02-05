import os
from glob import glob

import cv2

from imclasregan_server.database import _Database


def add_image(name: str):
    image = cv2.imread(name, cv2.IMREAD_COLOR)
    assert len(image.shape) == 3
    assert image.shape[2] == 3
    height, width = image.shape[:2]
    data = image.data

    with _Database() as db:
        db.cursor.execute(
            'INSERT INTO IMAGES(name, data, width, height) VALUES (?, ?, ?, ?)',
            (name, data, width, height))


def remove_from_inbox(fn: str):
    os.rename(fn, fn.replace('inbox', 'processed_images'))


if __name__ == '__main__':
    for fn in glob('inbox/*'):
        add_image(fn)
        remove_from_inbox(fn)