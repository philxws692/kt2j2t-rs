# kt2j2t-rs

This tool was created since I wanted to convert the tests from
the [TINF23CS-kauma-tests](https://github.com/Sarsum/TINF23CS-kauma-tests/) repository to use them with `cargo test` and
the [json2tests-rs](https://github.com/niri81/json2tests-rs/) crate.

# Usage

In order to convert from a "Kauma-Tests" JSON file to a "json2tests" JSON file just run:

```sh
kt2jt transform -i <kauma_test_file> -o <output_file>
```

Optionally you can also add the `-p` argument in order to add a prefix, which is prepended to every UUID in order to
identify tests more easily later on or if you want to create [`nextest`](https://nexte.st/) profiles to only run
specific tests

The input file content as well as the output file content is validated against the JSON schemas defined in the
respective repositories.

# Setup

You can use one of the many installation ways found in the releases. If you want to have auto completion you can simply
execute

```sh
kt2j2t completion <shell> > path/to/your/completions/folder/completionfile
```