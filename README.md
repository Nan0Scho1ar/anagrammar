# Anagrammar

Anagrammar is a Rust-based tool designed for constructing anagram pairs. It uses the `ratatui` library to provide a simple text-based user interface, allowing you to input two sentences and see their differences in terms of letter frequencies. The tool also suggests words that can be created from the remaining letters, helping you form anagrams.

## Features

- Two input fields to type sentences.
- A dynamic letter frequency display that shows the difference between the two sentences.
- A word suggestion box that generates words from the remaining letters, ordered from longest to shortest.

## Installation
### Prerequisites

- Rust (install it from [rust-lang.org](https://www.rust-lang.org/))

### Clone the repository
```bash
git clone https://github.com/Nan0Scho1ar/anagrammar.git
cd anagrammar
```

### Build and Run

To build the project, use the following command:
```bash
cargo build --release
```

Once the build is complete, you can run the program with:
```bash
cargo run
```

## Usage

1. Edit Mode: Press `e` to enter edit mode. While in edit mode, you can:
    - Type in the input fields.
    - Press `Tab` to switch between the two input fields.
2. Exit Edit Mode: Press `Esc` to leave edit mode.
3. Quit: After leaving edit mode, you can press `q` to quit the application.
4. Letter Frequency & Word Suggestions: As you edit the fields, the letter frequency differences and word suggestions will be updated in real time.

## How it Works

- The tool compares the two input sentences, counting the occurrence of each letter.
- The difference in the letter counts is displayed, showing which letters are excess in one sentence and deficient in the other.
- The word suggestions are generated from the remaining letters, helping you form anagrams by suggesting possible words from the unbalanced letters.

Example:

1. Input Sentence 1: `lounge chair`
2. Input Sentence 2: `I hunger coal`

The letter frequency difference and word suggestions are displayed accordingly.
## Contributing

If you'd like to contribute to the project, feel free to fork the repository, make changes, and open a pull request. Contributions are always welcome!
## License
This project is licensed under the GPL-3.0 License - see the LICENSE file for details.
