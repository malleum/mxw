use anyhow::anyhow;
use std::str::FromStr;

pub fn parse_scan_code(keystr: &str) -> Result<Key, anyhow::Error> {
    u8::from_str(keystr)
        .map(|value| RAW_KEYS.iter().find(|key| value == key.scan_code))?
        .try_into()
}

pub fn parse_key_code(keystr: &str) -> Result<Key, anyhow::Error> {
    u8::from_str(keystr)
        .map(|value| RAW_KEYS.iter().find(|key| value == key.key_code))?
        .try_into()
}

pub fn parse_code(keystr: &str) -> Result<Key, anyhow::Error> {
    RAW_KEYS.iter().find(|key| keystr == key.code).try_into()
}

#[derive(Clone)]
pub struct Key {
    pub scan_code: u8,
    pub key_code: u8,
    pub code: String,
    pub modifier: Option<u8>,
}

impl From<&RawKey> for Key {
    fn from(item: &RawKey) -> Self {
        Self {
            scan_code: item.scan_code,
            key_code: item.key_code,
            code: String::from(item.code),
            modifier: item.modifier,
        }
    }
}

struct RawKey {
    scan_code: u8,
    key_code: u8,
    code: &'static str,
    modifier: Option<u8>,
}

impl TryInto<Key> for Option<&RawKey> {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Key, Self::Error> {
        if let Some(v) = self {
            return Ok(Key::from(v));  // Remove the modifier check
        }

        Err(anyhow!("key code is invalid"))
    }
}

static RAW_KEYS: [RawKey; 90] = [
    RawKey {
        scan_code: 4,
        key_code: 65,
        code: "KeyA",
        modifier: None,
    },
    RawKey {
        scan_code: 5,
        key_code: 66,
        code: "KeyB",
        modifier: None,
    },
    RawKey {
        scan_code: 6,
        key_code: 67,
        code: "KeyC",
        modifier: None,
    },
    RawKey {
        scan_code: 7,
        key_code: 68,
        code: "KeyD",
        modifier: None,
    },
    RawKey {
        scan_code: 8,
        key_code: 69,
        code: "KeyE",
        modifier: None,
    },
    RawKey {
        scan_code: 9,
        key_code: 70,
        code: "KeyF",
        modifier: None,
    },
    RawKey {
        scan_code: 10,
        key_code: 71,
        code: "KeyG",
        modifier: None,
    },
    RawKey {
        scan_code: 11,
        key_code: 72,
        code: "KeyH",
        modifier: None,
    },
    RawKey {
        scan_code: 12,
        key_code: 73,
        code: "KeyI",
        modifier: None,
    },
    RawKey {
        scan_code: 13,
        key_code: 74,
        code: "KeyJ",
        modifier: None,
    },
    RawKey {
        scan_code: 14,
        key_code: 75,
        code: "KeyK",
        modifier: None,
    },
    RawKey {
        scan_code: 15,
        key_code: 76,
        code: "KeyL",
        modifier: None,
    },
    RawKey {
        scan_code: 16,
        key_code: 77,
        code: "KeyM",
        modifier: None,
    },
    RawKey {
        scan_code: 17,
        key_code: 78,
        code: "KeyN",
        modifier: None,
    },
    RawKey {
        scan_code: 18,
        key_code: 79,
        code: "KeyO",
        modifier: None,
    },
    RawKey {
        scan_code: 19,
        key_code: 80,
        code: "KeyP",
        modifier: None,
    },
    RawKey {
        scan_code: 20,
        key_code: 81,
        code: "KeyQ",
        modifier: None,
    },
    RawKey {
        scan_code: 21,
        key_code: 82,
        code: "KeyR",
        modifier: None,
    },
    RawKey {
        scan_code: 22,
        key_code: 83,
        code: "KeyS",
        modifier: None,
    },
    RawKey {
        scan_code: 23,
        key_code: 84,
        code: "KeyT",
        modifier: None,
    },
    RawKey {
        scan_code: 24,
        key_code: 85,
        code: "KeyU",
        modifier: None,
    },
    RawKey {
        scan_code: 25,
        key_code: 86,
        code: "KeyV",
        modifier: None,
    },
    RawKey {
        scan_code: 26,
        key_code: 87,
        code: "KeyW",
        modifier: None,
    },
    RawKey {
        scan_code: 27,
        key_code: 88,
        code: "KeyX",
        modifier: None,
    },
    RawKey {
        scan_code: 28,
        key_code: 89,
        code: "KeyY",
        modifier: None,
    },
    RawKey {
        scan_code: 29,
        key_code: 90,
        code: "KeyZ",
        modifier: None,
    },
    RawKey {
        scan_code: 30,
        key_code: 49,
        code: "Digit1",
        modifier: None,
    },
    RawKey {
        scan_code: 31,
        key_code: 50,
        code: "Digit2",
        modifier: None,
    },
    RawKey {
        scan_code: 32,
        key_code: 51,
        code: "Digit3",
        modifier: None,
    },
    RawKey {
        scan_code: 33,
        key_code: 52,
        code: "Digit4",
        modifier: None,
    },
    RawKey {
        scan_code: 34,
        key_code: 53,
        code: "Digit5",
        modifier: None,
    },
    RawKey {
        scan_code: 35,
        key_code: 54,
        code: "Digit6",
        modifier: None,
    },
    RawKey {
        scan_code: 36,
        key_code: 55,
        code: "Digit7",
        modifier: None,
    },
    RawKey {
        scan_code: 37,
        key_code: 56,
        code: "Digit8",
        modifier: None,
    },
    RawKey {
        scan_code: 38,
        key_code: 57,
        code: "Digit9",
        modifier: None,
    },
    RawKey {
        scan_code: 39,
        key_code: 48,
        code: "Digit0",
        modifier: None,
    },
    RawKey {
        scan_code: 40,
        key_code: 13,
        code: "Enter",
        modifier: None,
    },
    RawKey {
        scan_code: 41,
        key_code: 27,
        code: "Escape",
        modifier: None,
    },
    RawKey {
        scan_code: 42,
        key_code: 8,
        code: "Backspace",
        modifier: None,
    },
    RawKey {
        scan_code: 43,
        key_code: 9,
        code: "Tab",
        modifier: None,
    },
    RawKey {
        scan_code: 44,
        key_code: 32,
        code: "Space",
        modifier: None,
    },
    RawKey {
        scan_code: 45,
        key_code: 189,
        code: "Minus",
        modifier: None,
    },
    RawKey {
        scan_code: 46,
        key_code: 187,
        code: "Equal",
        modifier: None,
    },
    RawKey {
        scan_code: 47,
        key_code: 219,
        code: "BracketLeft",
        modifier: None,
    },
    RawKey {
        scan_code: 48,
        key_code: 221,
        code: "BracketRight",
        modifier: None,
    },
    RawKey {
        scan_code: 49,
        key_code: 220,
        code: "Backslash",
        modifier: None,
    },
    RawKey {
        scan_code: 51,
        key_code: 186,
        code: "Semicolon",
        modifier: None,
    },
    RawKey {
        scan_code: 52,
        key_code: 222,
        code: "Quote",
        modifier: None,
    },
    RawKey {
        scan_code: 53,
        key_code: 192,
        code: "Backquote",
        modifier: None,
    },
    RawKey {
        scan_code: 54,
        key_code: 188,
        code: "Comma",
        modifier: None,
    },
    RawKey {
        scan_code: 55,
        key_code: 190,
        code: "Period",
        modifier: None,
    },
    RawKey {
        scan_code: 56,
        key_code: 191,
        code: "Slash",
        modifier: None,
    },
    RawKey {
        scan_code: 57,
        key_code: 20,
        code: "CapsLock",
        modifier: None,
    },
    RawKey {
        scan_code: 58,
        key_code: 112,
        code: "F1",
        modifier: None,
    },
    RawKey {
        scan_code: 59,
        key_code: 113,
        code: "F2",
        modifier: None,
    },
    RawKey {
        scan_code: 60,
        key_code: 114,
        code: "F3",
        modifier: None,
    },
    RawKey {
        scan_code: 61,
        key_code: 115,
        code: "F4",
        modifier: None,
    },
    RawKey {
        scan_code: 62,
        key_code: 116,
        code: "F5",
        modifier: None,
    },
    RawKey {
        scan_code: 63,
        key_code: 117,
        code: "F6",
        modifier: None,
    },
    RawKey {
        scan_code: 64,
        key_code: 118,
        code: "F7",
        modifier: None,
    },
    RawKey {
        scan_code: 65,
        key_code: 119,
        code: "F8",
        modifier: None,
    },
    RawKey {
        scan_code: 66,
        key_code: 120,
        code: "F9",
        modifier: None,
    },
    RawKey {
        scan_code: 67,
        key_code: 121,
        code: "F10",
        modifier: None,
    },
    RawKey {
        scan_code: 68,
        key_code: 122,
        code: "F11",
        modifier: None,
    },
    RawKey {
        scan_code: 69,
        key_code: 123,
        code: "F12",
        modifier: None,
    },
    RawKey {
        scan_code: 70,
        key_code: 44,
        code: "PrintScreen",
        modifier: None,
    },
    RawKey {
        scan_code: 71,
        key_code: 145,
        code: "ScrollLock",
        modifier: None,
    },
    RawKey {
        scan_code: 72,
        key_code: 19,
        code: "Pause",
        modifier: None,
    },
    RawKey {
        scan_code: 73,
        key_code: 45,
        code: "Insert",
        modifier: None,
    },
    RawKey {
        scan_code: 74,
        key_code: 36,
        code: "Home",
        modifier: None,
    },
    RawKey {
        scan_code: 75,
        key_code: 33,
        code: "PageUp",
        modifier: None,
    },
    RawKey {
        scan_code: 76,
        key_code: 46,
        code: "Delete",
        modifier: None,
    },
    RawKey {
        scan_code: 77,
        key_code: 35,
        code: "End",
        modifier: None,
    },
    RawKey {
        scan_code: 78,
        key_code: 34,
        code: "PageDown",
        modifier: None,
    },
    RawKey {
        scan_code: 79,
        key_code: 39,
        code: "ArrowRight",
        modifier: None,
    },
    RawKey {
        scan_code: 80,
        key_code: 37,
        code: "ArrowLeft",
        modifier: None,
    },
    RawKey {
        scan_code: 81,
        key_code: 40,
        code: "ArrowDown",
        modifier: None,
    },
    RawKey {
        scan_code: 82,
        key_code: 38,
        code: "ArrowUp",
        modifier: None,
    },
    RawKey {
        scan_code: 83,
        key_code: 144,
        code: "NumLock",
        modifier: None,
    },
    RawKey {
        scan_code: 84,
        key_code: 111,
        code: "NumpadDivide",
        modifier: None,
    },
    RawKey {
        scan_code: 85,
        key_code: 106,
        code: "NumpadMultiply",
        modifier: None,
    },
    RawKey {
        scan_code: 86,
        key_code: 109,
        code: "NumpadSubtract",
        modifier: None,
    },
    RawKey {
        scan_code: 87,
        key_code: 107,
        code: "NumpadAdd",
        modifier: None,
    },
    RawKey {
        scan_code: 99,
        key_code: 110,
        code: "NumpadDecimal",
        modifier: None,
    },
    RawKey {
        scan_code: 101,
        key_code: 93,
        code: "ContextMenu",
        modifier: None,
    },
    RawKey {
        scan_code: 224,
        key_code: 17,
        code: "ControlLeft",
        modifier: Some(1),
    },
    RawKey {
        scan_code: 225,
        key_code: 16,
        code: "ShiftRight",
        modifier: Some(2),
    },
    RawKey {
        scan_code: 226,
        key_code: 18,
        code: "AltRight",
        modifier: Some(4),
    },
    RawKey {
        scan_code: 227,
        key_code: 91,
        code: "MetaLeft",
        modifier: Some(8),
    },
    RawKey {
        scan_code: 231,
        key_code: 92,
        code: "MetaRight",
        modifier: Some(128),
    },
];
