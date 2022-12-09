//! # underscore: `_`

pub mod underscore {
    pub fn surname() -> (&'static str, &'static str) {
        ("jordan", "michael")
    }

    pub fn applications() {
        // ignore some value
        let _ = "rust";

        // ignore some value returned by function
        let (_, surname) = surname();
        assert_eq!(surname, "michael");
    }
}
