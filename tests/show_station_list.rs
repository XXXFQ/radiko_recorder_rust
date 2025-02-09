// tests/show_station_list.rs

use assert_cmd::Command;

#[test]
fn test_show_station_list() {
    let mut cmd = Command::cargo_bin("radiko_recorder").expect("Binary exists");
    cmd.args(&["--station-list", "--area_id", "JP13"])
       .assert()
       .success();
}