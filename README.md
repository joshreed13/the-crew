# The Crew Solver

This project contains a program to determine if games of The Crew are winnable. Alongside the solver, is a web app that
allows each player to submit which cards they have in their hand so the computer can solve the game without any one
player having to spoil the game.

## Subprojects

* **thecrewsolver** - Rust app that solves games
* **the-crew-backend** - Python backend for the web app
* **the-crew-web** - React frontend

## Usage

1. Use cargo to compile the solver (release mode is highly recommended as this is a CPU-intensive app)
```
cargo build --release
```

2. The `thecrewsolver\solverServer.py` python script wraps the rust app with an HTTP server. It will need to have the compiled `thecrewsolver.exe` in it's working directory.

3. The backend app will need to be given a URL path to the solver server in the `SOLVER_ENDPOINT` environment variable
```
$env:SOLVER_ENDPOINT="http://127.0.0.1:8000/solve"
python .\app.py
```

4. Start the web frontend:
```
npm start
```
