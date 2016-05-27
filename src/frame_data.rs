/// Holds a frame of either utf8 encoded or binary data.
#[derive(Debug)]
pub struct FrameData {
    pub utf8: Option<String>,
    pub binary: Option<Vec<u8>>
}

impl FrameData {
    pub fn from_utf8(utf8: String) -> FrameData {
        FrameData { utf8: Some(utf8), binary: None }
    }

    pub fn from_binary_buffer(binary: Vec<u8>) -> FrameData {
        FrameData { utf8: None, binary: Some(binary) }
    }

    pub fn is_utf8(&self) -> bool {
        self.utf8.is_some()
    }
}
