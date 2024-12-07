# Advent of Code 2024

## Advent of Code 2024 - Day 01

=== Part 1 ===
src/bin/01.rs:61 took 1.868417ms.
Result = 2057374

=== Part 2 ===
src/bin/01.rs:90 took 2.444833ms.
Result = 23177084

## Advent of Code 2024 - Day 02

=== Part 1 ===
src/bin/02.rs:40 took 2.264917ms.
Result = 356

=== Part 2 ===
src/bin/02.rs:73 took 4.106584ms.
Result = 413

## Advent of Code 2024 - Day 03

=== Part 1 ===
src/bin/03.rs:44 took 4.485958ms.
Result = 168539636

=== Part 2 ===
src/bin/03.rs:69 took 4.832041ms.
Result = 97529391

## Advent of Code 2024 - Day 04

=== Part 1 ===
src/bin/04.rs:85 took 12.548708ms.
Result = 2493

=== Part 2 ===
src/bin/04.rs:117 took 3.309375ms.
Result = 1890

## Advent of Code 2024 - Day 05

=== Part 1 ===
src/bin/05.rs:126 took 14.222584ms.
Result = 5374

=== Part 2 ===
thread 'main' `panicked` at src/bin/05.rs:156:5:
assertion `left == right` failed
left: 123
right: 0

## Advent of Code 2024 - Day 06

### Without Threads

Advent of Code 2024 - Day 06
=== Part 1 ===
src/bin/06.rs:240 took 4.124083ms.
Result = 5177

=== Part 2 ===
src/bin/06.rs:257 took 35.2666385s.
Result = 1686

### With Threads

=== Part 1 ===
src/bin/06-thread.rs:254 took 6.666833ms.
Result = 5177

=== Part 2 ===
src/bin/06-thread.rs:271 took 4.282960083s.
Result = 1686

## Usage

1. Create a new project from the template repository:

   - Using GitHub’s templating feature: Simply click the Use this template [button](https://github.com/new?template_name=advent-of-code-rust-template&template_owner=bravit) on the repository page, create a new repository, and then open it in [RustRover](https://www.jetbrains.com/rust/) by selecting _File | New | Project From Version Control…_.
   - Adding the template to RustRover: You can integrate the template directly into RustRover and use the regular New Project wizard.

2. Whenever you're ready to start solving a new day's puzzle:

   - Open the `bin` folder, copy and paste the `NN.rs` file into it, and give it the corresponding name (`01.rs`, `02.rs`, etc.).
   - In the `input` folder, create and fill the input data file (`01.txt`, `02.txt`, etc.).
   - Fill in the `DAY` constant in the freshly created file.
   - Run the current day's solution to check if it compiles (you can use the gutter icon next to the `main` function).
   - Fill in `<TEST-INPUT>`.
   - Write the expected answer for the test data in the `assert_eq` statement in _Part 1_.
   - Now you're ready to write your solution in the `part1` function (inside `main`).
   - Use `Shift+F10` (Win/Linux) or `Ctrl-R` (macOS) to re-run the same program.

3. When you're done with the first part of the puzzle, use folding to hide _Part 1_.

4. Uncomment _Part 2_, fill in the test data assertion, and start solving it.
