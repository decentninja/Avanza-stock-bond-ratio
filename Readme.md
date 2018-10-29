An application that calculates the instrument type ratios of your Avanza portfolio.

Will give you an output such as:
```
Your portfolio consists of
Branschfond          1215.9   1.5%
Aktier               2194.0   2.7%
Aktiefond           56087.5  69.2%
Räntefond           21606.9  26.6%
Total               81104.3  sek.
```

# Installation Instructions
1. Install Nodejs version > v10.12.0
2. Install Rust version > 1.30.1 (2018-09-20)
3. Clone the repo
4. The repo run `cargo install`
5. Run `avanza-additional-analysis`
6. Follow the on-screen instructions

# Development Notes
The application consists of two parts.
* A rust executable, that handles the analysis and the user-facing-command-line interface.
* A Nodejs script, that that calls Github user Fhqvst's Avanza module.

The Rust executable runs the Nodejs script as a child process.
The two parts talk with each other over a stdin/out/err readline interface.

# TODO
[ ] Complete all todos in the repo
[ ] Go over documentation once again, add whats missing from the todos
[ ] Make compiled binaries for all platsforms.
[ ] Add tests