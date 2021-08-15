use std::thread::sleep;

use headless_chrome::{Browser, browser::LaunchOptionsBuilder, protocol::input::methods::DispatchKeyEvent};

// Number of windows to be opened, should be based on CPU core
const THREADS: u8 = 4;

fn main() {
    for _ in 0..THREADS {
        std::thread::spawn(|| -> Result<(), failure::Error> {
            let launch_options = LaunchOptionsBuilder::default()
                .headless(false)
                .window_size(Some((200,600)))
                .build()
                .unwrap();
        
            let browser = Browser::new(launch_options).unwrap();
            let tab = browser.wait_for_initial_tab().unwrap();
        
            tab.navigate_to("https://popcat.click")?;
            tab.wait_until_navigated()?;
       
            tab.wait_for_element(".cat-img")?;

            let enter = Some("Enter");
            let text = Some("\r");
            let event_type = "keyDown";
        
            loop {
                tab.call_method(DispatchKeyEvent {
                    event_type,
                    key: enter,
                    native_virtual_key_code: 13,
                    windows_virtual_key_code: 13,
                    code: enter,
                    text
                })?;
            }
        });
    };

    sleep(std::time::Duration::new(10000000000000, 0))
}
