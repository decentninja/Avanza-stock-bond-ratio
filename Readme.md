Calculates the stock/bond ratio of your Avanza portfolio.

Will give you an output such as:
```
Your portfolio consists of
Branschfond          1215.9   1.5%
Aktier               2194.0   2.7%
Aktiefond           56087.5  69.2%
Räntefond           21606.9  26.6%
Total               81104.3
```

# Installation Instructions
Install nodejs version > v10.12.0
Rust 1.29.1 (2018-09-20)
Clone the repo
Run `cargo install`
Run `avanza-additional-analysis`

# Development Notes
The application consists of two parts.
* A rust executable, that handles the analysis and the userfacing commandline interface.
* A nodejs script that that calls github user fhqvst's Avanza module
These two parts talk with a readline interface.