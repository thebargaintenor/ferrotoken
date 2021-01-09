# Ferrotoken

Yet another utility for created token images from templates.

No prebuilt binaries are currently available, so this will have to be built from source for your system.  No need for the nightly rust build - stable is fine.


```
USAGE:
    ferrotoken [OPTIONS] <INPUT> --output <output> --template <template>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m, --mask <mask>            Masking channel color (default FF00FF)
    -o, --output <output>        Output image file location
    -t, --template <template>    Template image location

ARGS:
    <INPUT>    Input image file location
```

Batch processing is on the roadmap, but I'm still sorting out how I want to design that behavior.  In the meantime it'll build one image at a time just fine.