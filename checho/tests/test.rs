#[cfg(test)]
mod tests {

    use assert_cmd::Command;
    use predicates::prelude::*;
    use std::fs;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn dies_no_args() -> TestResult {
        let mut cmd = Command::cargo_bin("checho")?;
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Usage"));
        Ok(())
    }

    #[test]
    fn runs() -> TestResult {
        let mut cmd = Command::cargo_bin("checho")?;
        cmd.arg("hello").assert().success();
        Ok(())
    }

    #[test]
    fn hello1() -> TestResult {
        let path = "tests/expected/hello1.txt";
        let expected = fs::read_to_string(path)?;
        let mut cmd = Command::cargo_bin("checho")?;
        cmd.arg("Hello there").assert().success().stdout(expected);
        Ok(())
    }
    #[test]
    fn hello1_n() -> TestResult {
        let path = "tests/expected/hello1.n.txt";
        let expected = fs::read_to_string(path)?;
        let mut cmd = Command::cargo_bin("checho")?;
        cmd.args(&["-n", "Hello there"])
            .assert()
            .success()
            .stdout(expected);
        Ok(())
    }
    #[test]
    fn hello2() -> TestResult {
        let path = "tests/expected/hello2.txt";
        let expected = fs::read_to_string(path)?;
        let mut cmd = Command::cargo_bin("checho")?;
        cmd.arg("Hello there").assert().success().stdout(expected);
        Ok(())
    }
    #[test]
    fn hello2_n() -> TestResult {
        let path = "tests/expected/hello2.n.txt";
        let expected = fs::read_to_string(path)?;
        let mut cmd = Command::cargo_bin("checho")?;
        cmd.args(&["-n", "Hello there"])
            .assert()
            .success()
            .stdout(expected);
        Ok(())
    }
}
