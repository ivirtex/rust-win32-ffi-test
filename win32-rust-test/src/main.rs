use windows_sys::{
    core::*,
    Win32::{
        Foundation::*, Graphics::Gdi::*, System::LibraryLoader::GetModuleHandleA,
        UI::WindowsAndMessaging::*,
    },
};

const WINDOWS_TO_CREATE: usize = 100;

fn main() {
    let window_class_name = s!("Test window");

    let mut timings = Vec::new();

    for _ in 0..WINDOWS_TO_CREATE {
        let start_time = std::time::Instant::now();

        unsafe {
            let instance = GetModuleHandleA(std::ptr::null());
            debug_assert!(instance != 0);

            let wc = WNDCLASSA {
                hCursor: LoadCursorW(0, IDC_ARROW),
                hInstance: instance,
                lpszClassName: window_class_name,
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(wnd_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hIcon: 0,
                hbrBackground: GetStockObject(WHITE_BRUSH),
                lpszMenuName: std::ptr::null(),
            };

            let atom = RegisterClassA(&wc);
            debug_assert!(atom != 0);

            let hwnd = CreateWindowExA(
                0,
                window_class_name,
                s!("This is a sample window"),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                0,
                0,
                instance,
                std::ptr::null(),
            );

            let mut message = std::mem::zeroed();

            while GetMessageA(&mut message, 0, 0, 0) != 0 {
                TranslateMessage(&message);
                DispatchMessageA(&message);
            }

            DestroyWindow(hwnd);
            UnregisterClassA(window_class_name, instance);
        }

        let elapsed = start_time.elapsed().as_millis();
        timings.push(elapsed);
    }

    // Calculate the average time taken to create a window
    let total_time: u128 = timings.iter().sum();
    let avg_ms_per_window = total_time / WINDOWS_TO_CREATE as u128;

    println!(
        "Average time to create a window: {} ms ({} windows)",
        avg_ms_per_window, WINDOWS_TO_CREATE
    );
}

extern "system" fn wnd_proc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                PostQuitMessage(0);

                0
            }
            WM_DESTROY => {
                PostQuitMessage(0);

                0
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
