extern crate audiophile;

#[cfg(test)]
mod tests {
    #[test]
    fn cd_quality_is_not_audiophile() {
        let testee = audiophile::MediaInfo {
            format: "test".to_string(),
            depth: 16,
            rate: 44100,
        };
        
        assert_eq!(false, testee.is_audiophile_grade_audio());
    }

    #[test]
    fn bit24_quality_is_audiophile() {
        let testee = audiophile::MediaInfo {
            format: "test".to_string(),
            depth: 24,
            rate: 44100,
        };
        
        assert_eq!(true, testee.is_audiophile_grade_audio());
    }

    #[test]
    fn rate48_quality_is_audiophile() {
        let testee = audiophile::MediaInfo {
            format: "test".to_string(),
            depth: 16,
            rate: 48000,
        };
        
        assert_eq!(true, testee.is_audiophile_grade_audio());
    }
}
