# serve

Tiny web server for hosting static files.

## Installation

1. Clone the repository.
2. Run `cargo install`.

## Usage

Serve files from the current working directory:
```
$ serve
```

Serve files from the target directory:
```
$ serve /path/to/target
```

Start the server on a target port:
```
$ serve --port 8080
```

Bind the server to the target host:
```
$ serve --host 192.168.1.123
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
