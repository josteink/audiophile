extern crate audiophile;

#[cfg(test)]
mod tests {
    use audiophile::MediaInfo;

    #[test]
    fn cd_quality_is_not_audiophile() {
        test_audio(false, 16, 44100);
    }

    #[test]
    fn bit24_quality_is_audiophile() {
        test_audio(true, 24, 44100);
    }

    #[test]
    fn rate48_quality_is_audiophile() {
        test_audio(true, 16, 48000);
    }

    fn test_audio(is_audiophile: bool, depth: u32, rate: u32) {
        let testee = MediaInfo {
            format: "test".to_string(),
            depth: depth,
            rate: rate,
        };

        assert_eq!(is_audiophile, testee.is_audiophile_grade_audio());
    }
}
