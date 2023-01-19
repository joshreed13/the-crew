from http import HTTPStatus
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import subprocess

TIMEOUT = 5
SOLVER_EXECUTABLE = ["thecrewsolver.exe"]


class RequestHandler(BaseHTTPRequestHandler):
    def do_POST(self):
        if self.path == "/solve":
            try:
                inputLength = int(self.headers.get('content-length', 0))
                inputData = self.rfile.read(inputLength)
                proc = subprocess.run(SOLVER_EXECUTABLE, input=inputData,
                                      capture_output=True, text=False, timeout=TIMEOUT)
                if proc.returncode == 0:
                    self.send_response(HTTPStatus.OK)
                else:
                    self.send_response(HTTPStatus.BAD_GATEWAY)
                self.send_header("Content-Type", "application/json")
                self.send_header("Content-Length", str(len(proc.stdout)))
                self.end_headers()
                self.wfile.write(proc.stdout)
            except subprocess.TimeoutExpired:
                self.send_response(HTTPStatus.GATEWAY_TIMEOUT)
                self.end_headers()
            except:
                self.send_response(HTTPStatus.INTERNAL_SERVER_ERROR)
                self.end_headers()
        else:
            self.send_response(HTTPStatus.BAD_REQUEST)
            self.end_headers()


if __name__ == "__main__":
    port = 8000
    with ThreadingHTTPServer(("", port), RequestHandler) as httpd:
        print(f"Serving on port {port}...")
        httpd.serve_forever()
