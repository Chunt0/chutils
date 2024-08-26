#[cfg(test)]
mod tests {

    use assert_cmd::Command;

    #[test]
    fn runs() {
        Command::cargo_bin("tmp").unwrap().assert().success();
    }
}
