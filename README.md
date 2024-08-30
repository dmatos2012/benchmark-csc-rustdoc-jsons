# Benchmark rustdoc json format
This assumes you are coming from `cargo-semver-checks` , so you can find all the local data in the folder: `{root_to_csc}/localdata/test_data/{lint_name}/{old,new}/rustdoc.json` by running the script `./scripts/regenerate_test_rustdocs.sh` in that repo.

## Adding files
To add files, just add them to the `LINTS`  with `lint_name`

## Adding the huge `aws_sdk_ec2.json`
Read the nice issue description from [csc](https://github.com/obi1kenobi/cargo-semver-checks/issues/885#issue-2485321240) to know how to generate the `aws_sdk_ec2.json`. Once generated, just copy it to the `test_data` so that its in the same place as the others:
```
mkdir -p <whatever_name>/old
cp aws_sdk_ec2.json <whatever_name>/old/rustdoc.json
```
Add it to the `LINTS` with `<whatever_name>` and should pick it up. 
> NOTE: Its about ~500MB so it will take a long time to benchmark, so maybe only add it at the end.

## Benchmark
`cargo bench`



