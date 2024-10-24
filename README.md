# Parse Duration with Fallbacks

Very thin wrapper over the [`humantime`](https://github.com/tailhook/humantime/tree/master) library that provides convenience methods to parse time _either_ as a `humantime` compatible string _or_ as a number, falling back to a default time unit. Comes in handy for passing a `value_parser` function when using [`Clap`](https://github.com/clap-rs/clap) derive macros.

Supports the following convenience wrappers:

- `parse_duration_fallback_us`: falls back to `Microseconds`
- `parse_duration_fallback_ns`: falls back to `Nanoseconds`
- `parse_duration_fallback_ms`: falls back to `Milliseconds`
- `parse_duration_fallback_sec`: falls back to `Seconds`
- `parse_duration_fallback_min`: falls back to `Minutes`
- `parse_duration_fallback_hour`: falls back to `Hours`
- `parse_duration_fallback_day`: falls back to `Days`

## Examples

- using a duration string:
  ```rust
  let duration_string = "42h";
  match parse_duration_fallback_sec(duration_string) {
      Ok(duration) => {
          println!("{:?}", duration);
      }
      Err(e) => {
          println!("{:?}", e)
      }
  }
  ```
- using a number:
  ```rust
  let seconds = "1_000_000";
  match parse_duration_fallback_sec(seconds) {
      Ok(duration) => {
          println!("{:?}", duration);
      }
      Err(e) => {
          println!("{:?}", e)
      }
  }
  ```
- in a `Clap` macro:
  ```rust
  #[clap(long = "duration-argument", default_value = "100ms", value_parser = parse_duration_fallback_ms)]
  duration_argument: Duration
  ```
  and you can now use `./cli --duration-argument 10ms` or `./cli --duration-argument 10`

## License

Licensed under either of

* Apache License, Version 2.0, (./LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (./LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
