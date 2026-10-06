fn main() {
    // TEMPORARY smoke test for purpleclay/release-workflows#99: fail one leg
    // on the first attempt only, to check that re-running just that leg
    // completes the release.
    if std::env::var("TARGET").as_deref() == Ok("x86_64-apple-darwin")
        && std::env::var("GITHUB_RUN_ATTEMPT").as_deref() == Ok("1")
    {
        panic!("deliberate first-attempt failure for release-workflows#99");
    }
    built::write_built_file().unwrap();
}
