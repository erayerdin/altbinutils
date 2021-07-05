use assert_cmd::Command;
#[macro_use]
extern crate rstest;

#[fixture]
fn command() -> Command {
    Command::cargo_bin("rment").unwrap()
}
#[rstest]
fn no_entry(mut command: Command) {
    command.assert().failure();
}
