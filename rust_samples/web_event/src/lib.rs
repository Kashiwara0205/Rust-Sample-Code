pub enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

pub fn inspect(event: WebEvent) -> String {
    match event {
        WebEvent::PageLoad => "PageLoad".to_string(),
        WebEvent::PageUnload => "PageUnload".to_string(),
        WebEvent::KeyPress(c) => format!("KeyPress {}", c),
        WebEvent::Paste(s) => format!("Paste {}", s),
        WebEvent::Click { x, y } => {
            format!("clicked at x={}, y={}", x, y)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inspect_works() {
        assert_eq!(inspect(WebEvent::PageLoad), "PageLoad");
    }
}
