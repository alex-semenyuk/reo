// Copyright (c) 2022 Yegor Bugayenko
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::fs::File;
use predicates::prelude::*;
use tempfile::TempDir;
use anyhow::{Result};
use std::io::Write;
use glob::glob;
use predicates::prelude::predicate;

#[test]
fn prints_help() {
    assert_cmd::Command::cargo_bin("reo").unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("GMI to Rust")
                .and(predicate::str::contains("--home"))
        );
}

#[test]
fn prints_version() {
    assert_cmd::Command::cargo_bin("reo").unwrap()
        .arg("--version")
        .assert()
        .success();
}

#[test]
fn dataizes_simple_gmi() -> Result<()> {
    let tmp = TempDir::new()?;
    File::create(tmp.path().join("foo.gmi"))?.write_all(
        "
        ADD('$ν1');
        BIND('$ε2', 'ν0', '$ν1', 'foo');
        DATA('$ν1', 'ff ff');
        ".as_bytes()
    )?;
    assert_cmd::Command::cargo_bin("reo").unwrap()
        .arg(format!("--home={}", tmp.path().display()))
        .arg("dataize")
        .arg("foo")
        .assert()
        .success()
        .stdout("ff-ff\n");
    Ok(())
}

#[test]
fn dataizes_all_gmi_tests() -> Result<()> {
    for f in glob("gmi-tests/*.gmi")? {
        let p = f?;
        let path = p.as_path();
        assert_cmd::Command::cargo_bin("reo").unwrap()
            .arg(format!("--file={}", path.display()))
            .arg("--verbose")
            .arg("dataize")
            .arg("foo")
            .assert()
            .success()
            .stdout(predicate::str::contains("Dataization result is: "));
    }
    Ok(())
}
