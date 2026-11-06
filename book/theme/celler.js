// The book only offers the Rust theme (see book.toml), but readers may still
// have another theme saved from before. Forget it and reload once, so mdbook
// starts over with the Rust theme.
try {
    const theme = localStorage.getItem("mdbook-theme");
    if (theme !== null && theme !== "rust") {
        localStorage.removeItem("mdbook-theme");
        // Only reload if removing worked, to never end up in a reload loop.
        if (localStorage.getItem("mdbook-theme") === null) {
            location.reload();
        }
    }
} catch (e) {
    // Storage is unavailable, so there is no saved theme to clear.
}
