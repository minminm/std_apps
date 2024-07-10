use std::env::current_exe;
use std::path::Path;
use std::env::current_dir;
use std::env::split_paths;
use std::env::join_paths;
use std::env::args;
use std::env::args_os;
use std::env::vars;
use std::env::vars_os;

#[cfg_attr(any(target_os = "emscripten", target_env = "sgx"), ignore)]
fn test_self_exe_path() {
    let path = current_exe();
    assert!(path.is_ok());
    let path = path.unwrap();

    // Hard to test this function
    assert!(path.is_absolute());
}

fn test() {
    assert!((!Path::new("test-path").is_absolute()));

    #[cfg(not(target_env = "sgx"))]
    current_dir().unwrap();
}

#[cfg(windows)]
fn split_paths_windows() {
    use crate::path::PathBuf;

    fn check_parse(unparsed: &str, parsed: &[&str]) -> bool {
        split_paths(unparsed).collect::<Vec<_>>()
            == parsed.iter().map(|s| PathBuf::from(*s)).collect::<Vec<_>>()
    }

    assert!(check_parse("", &mut [""]));
    assert!(check_parse(r#""""#, &mut [""]));
    assert!(check_parse(";;", &mut ["", "", ""]));
    assert!(check_parse(r"c:\", &mut [r"c:\"]));
    assert!(check_parse(r"c:\;", &mut [r"c:\", ""]));
    assert!(check_parse(r"c:\;c:\Program Files\", &mut [r"c:\", r"c:\Program Files\"]));
    assert!(check_parse(r#"c:\;c:\"foo"\"#, &mut [r"c:\", r"c:\foo\"]));
    assert!(check_parse(r#"c:\;c:\"foo;bar"\;c:\baz"#, &mut [r"c:\", r"c:\foo;bar\", r"c:\baz"]));
}

// #[cfg(unix)]
fn split_paths_unix() {
    use std::path::PathBuf;

    fn check_parse(unparsed: &str, parsed: &[&str]) -> bool {
        split_paths(unparsed).collect::<Vec<_>>()
            == parsed.iter().map(|s| PathBuf::from(*s)).collect::<Vec<_>>()
    }

    assert!(check_parse("", &mut [""]));
    assert!(check_parse("::", &mut ["", "", ""]));
    assert!(check_parse("/", &mut ["/"]));
    assert!(check_parse("/:", &mut ["/", ""]));
    assert!(check_parse("/:/usr/local", &mut ["/", "/usr/local"]));
}

// #[cfg(unix)]
fn join_paths_unix() {
    use std::ffi::OsStr;

    fn test_eq(input: &[&str], output: &str) -> bool {
        &*join_paths(input.iter().cloned()).unwrap() == OsStr::new(output)
    }

    assert!(test_eq(&[], ""));
    assert!(test_eq(&["/bin", "/usr/bin", "/usr/local/bin"], "/bin:/usr/bin:/usr/local/bin"));
    assert!(test_eq(&["", "/bin", "", "", "/usr/bin", ""], ":/bin:::/usr/bin:"));
    assert!(join_paths(["/te:st"].iter().cloned()).is_err());
}

#[cfg(windows)]
fn join_paths_windows() {
    use crate::ffi::OsStr;

    fn test_eq(input: &[&str], output: &str) -> bool {
        &*join_paths(input.iter().cloned()).unwrap() == OsStr::new(output)
    }

    assert!(test_eq(&[], ""));
    assert!(test_eq(&[r"c:\windows", r"c:\"], r"c:\windows;c:\"));
    assert!(test_eq(&["", r"c:\windows", "", "", r"c:\", ""], r";c:\windows;;;c:\;"));
    assert!(test_eq(&[r"c:\te;st", r"c:\"], r#""c:\te;st";c:\"#));
    assert!(join_paths([r#"c:\te"st"#].iter().cloned()).is_err());
}

fn args_debug() {
    assert_eq!(
        format!("Args {{ inner: {:?} }}", args().collect::<Vec<_>>()),
        format!("{:?}", args())
    );
}

fn args_os_debug() {
    assert_eq!(
        format!("ArgsOs {{ inner: {:?} }}", args_os().collect::<Vec<_>>()),
        format!("{:?}", args_os())
    );
}

fn vars_debug() {
    assert_eq!(
        format!("Vars {{ inner: {:?} }}", vars().collect::<Vec<_>>()),
        format!("{:?}", vars())
    );
}

fn vars_os_debug() {
    assert_eq!(
        format!("VarsOs {{ inner: {:?} }}", vars_os().collect::<Vec<_>>()),
        format!("{:?}", vars_os())
    );
}

fn main() {
    // test_self_exe_path();
    test();

    #[cfg(windows)]
    {
        split_paths_windows();
        join_paths_windows();
    }

    // #[cfg(unix)]
    {
        split_paths_unix();
        join_paths_unix();
    }

    args_debug();
    args_os_debug();
    vars_debug();
    vars_os_debug();
}