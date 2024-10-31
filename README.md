# `fnote`

[![Lint](https://github.com/blackboxaudio/fnote/actions/workflows/ci.lint.yml/badge.svg)](https://github.com/blackboxaudio/fnote/actions/workflows/ci.lint.yml)
[![Format](https://github.com/blackboxaudio/fnote/actions/workflows/ci.fmt.yml/badge.svg)](https://github.com/blackboxaudio/fnote/actions/workflows/ci.fmt.yml)
[![Version: v0.1.0](https://img.shields.io/badge/Version-v0.1.0-blue.svg)](https://github.com/blackboxaudio/fnote)
[![License](https://img.shields.io/badge/License-MIT-yellow)](https://github.com/blackboxaudio/fnote/blob/develop/LICENSE)

> CLI tool for converting musical notes, frequencies, and MIDI 🎵

## Overview

`fnote` is a simple tool to convert frequencies, music notes, and MIDI note numbers to and from each other.
It is easier to run this tool than to search for the information everytime.

## Getting Started 

Clone this repository:
```bash
git clone https://github.com/blackboxaudio/fnote
cd fnote/
```

Build and run the CLI tool:
```bash
cargo run --release -- midi 60

# MIDI: 60
# Note: C4
# Frequency: 261.63 Hz
```

## Using `fnote`

```
USAGE:
    fnote <command> <arg>
    
FLAGS:
    -h, --help      Prints help information
    -V, --version   Prints version information
    
ARGS:
    <command>       Command to execute, i.e. `midi`, `note`, or `freq`
    <arg>           Argument for command, e.g. frequency value (440) or music note (A4)
```
