# get-3gpp-spec

3GPP spec document downloader

## Usage

1. [Download the app](https://github.com/proj3rd/get-3gpp-spec/releases/latest)
1. Place it to one of your `PATH` folders (or add one) 
1. (Optional) Rename it to `get-3gpp-spec`
1. Run a command in the console as follows:

```sh
get-3gpp-spec 38.331
```

The above command downloads 3GPP TS 38.331 spec document with the highest version.

To specify Release:

```sh
get-3gpp-spec 38.331 -r 17
```

The above command downloads 3GPP TS 38.331 spec document with the highest version of Release 17.

To specify a quarter of a year:

```sh
get-3gpp-spec 38.331 -d 2024-06
```

The above command downloads 3GPP TS 38.331 spec document with the highest version published between June 2024 to Auguest 2024, i.e. 2nd quarter 2024.


Put all together:

```sh
get-3gpp-spec 38.331 -r 17 -d 2024-06
```

To see a list of specs instead of downloading one:

```sh
get-3gpp-spec 38.331 -l
```

## Support

If you feel this is useful, please consider to [buy me a coffee](https://buymeacoffee.com/somidad).

For any issue or feature request, please [let me know](https://github.com/proj3rd/get-3gpp-spec/issues).
