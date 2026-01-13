use bitflags::bitflags;
bitflags! {
    #[derive(serde::Serialize, Debug)]
    pub struct PostScriptGlobalHintingOptions: u16 {
        /// Automatically generate Flex hints in T1 and OT
        const GENERATE_FLEX = 1 << 0;
    }
    #[derive(serde::Serialize, Debug)]
    pub struct PostScriptGlyphHintingOptions: u32 {
        /// Hint Replacement
        const HINT_REPLACEMENT = 1 << 29;
        /// Generate horizontal 3-stem
        const HORIZONTAL_3_STEM = 1 << 30;
        /// Generate vertical 3-stem
        const VERTICAL_3_STEM = 1 << 31;
    }
}

impl From<u16> for PostScriptGlobalHintingOptions {
    fn from(value: u16) -> Self {
        PostScriptGlobalHintingOptions::from_bits_truncate(value)
    }
}

impl From<u32> for PostScriptGlyphHintingOptions {
    fn from(value: u32) -> Self {
        PostScriptGlyphHintingOptions::from_bits_truncate(value)
    }
}
