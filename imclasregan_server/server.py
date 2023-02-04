import json
from http.server import SimpleHTTPRequestHandler

from .database import get_classes, get_image, store_result


class ImClasRegAnServer(SimpleHTTPRequestHandler):

    def __init__(self, *args):
        super().__init__(*args, directory='site')

    def do_POST(self, *args, **kwargs):
        content_length = int(self.headers['Content-Length']) # <--- Gets the size of data
        post_data = self.rfile.read(content_length).decode('utf-8')
        
        response_data = self.handle_command(**json.loads(post_data))

        self.send_response(200)
        self.send_header('Content-type', 'application/json')
        self.end_headers()
        self.wfile.write(json.dumps(response_data).encode('utf-8'))
    
    def handle_command(self, *, command: str, **kwargs) -> dict:
        if command == 'get_image':
            return get_image()
        elif command == 'get_classes':
            return get_classes()
        elif command == 'store_result':
            return store_result(**kwargs)
        else:
            return dict(error=True, message=f'unrecognised command \'{command}\'')
