#[cfg(test)]
mod tests {
    const EXIT_FAILURE: i32 = 1;

    use assert_cmd::Command;

    #[test]
    fn runs() {
        Command::cargo_bin("chfalse")
            .unwrap()
            .assert()
            .failure()
            .code(EXIT_FAILURE);
    }
}
