use assert_cmd::Command; // библиотека для тестирования CLI

#[test]
fn test_multplie_inputs() {
    // Для всех выходов данных
    let test_cases = vec![
        ("1", "3\n-13\n-40\n"), // без пробелов как в основной программе main.rs
        ("2", "3\n-1\n2\n"),
        ("3", "38\n-22\n240\n"),
        ("4", "4002\n-3998\n8000\n"),
        ("5", "500030\n-499970\n15000000\n"),
    ];
    for (input, expected) in test_cases {
        let mut cmd = Command::cargo_bin("RustProgramm").unwrap();
        cmd.write_stdin(input).assert().success().stdout(expected);
    }
    /*
    // Для одного входа и выхода данных
    // запускаем main.rs как бинарник
    let mut cmd = Command::cargo_bin("RustProgramm").unwrap();
    // подаем входные данные в stdin
    cmd.write_stdin("1")
        .assert()
        .success()
        .stdout("3\n-13\n-40\n"); // ожидаемый результат
    */
}
