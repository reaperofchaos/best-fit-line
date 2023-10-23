# Semester Project - CPU Temps

The program calcuates a best fit equation for a collection of provided cpu core temperatures. This program calcuates the line first through a linear piecewise equation consisting of equations between two points for each interval of time.

//TODO: The program will use a linear approximation will be used to calculate a best fit line for all points.

# Requirements

## Language

This project is built using the Rust langauge (1.73.0)

## Installed Packages

temperature-parser 0.1.0
regex 1
lazy_static 1.4.0

# Compilation

The code can be compiled using Rust's built in build system, cargo by navigating to the semester-project folder.

```
cd semester-project
```

Creating an executable

```
cargo build --release
```

Running through cargo

```
cargo run
```

# Sample Execution & Output

## Execution

```
cargo run <fileName>
```

To execute using the provided sample input.

```
cargo run sample-data/sample_1.txt
```

## Output

Output rendered during execution of the program
![image](docs/images/output.png)
Multiple output files will be created where the program was executed that consist of the format

```
<input file's base name>-core-<column number>.txt
```

The contents of each file will be similar to the the lines in the program's console output separated by the core. Each core will have its own file. The output file for one of the cpu cores would look similar to the text below.

```
0 <= x <=	30 ; y = 	 61.0000 +	 0.6333 x ; interpolation
30 <= x <=	60 ; y = 	 98.0000 +	 -0.6000 x ; interpolation
60 <= x <=	90 ; y = 	 20.0000 +	 0.7000 x ; interpolation
90 <= x <=	120 ; y = 	 128.0000 +	 -0.5000 x ; interpolation

```

## Testing

Unit tests can be run through cargo.

```
cargo test
```

# Documentation

Accessible via the cargo command

```
cargo doc --open
```
