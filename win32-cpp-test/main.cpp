#include <windows.h>
#include <chrono>
#include <cstdio>
#include <numeric>
#include <vector>

const int WINDOWS_TO_CREATE = 100;

LRESULT CALLBACK WndProc(HWND hwnd, UINT msg, WPARAM wParam, LPARAM lParam) {
  switch (msg) {
    case WM_PAINT:
    case WM_DESTROY:
      PostQuitMessage(0);
      break;
    default:
      return DefWindowProc(hwnd, msg, wParam, lParam);
  }
  return 0;
}

int main() {
  auto windowClassName = "Test window";

  std::vector<std::chrono::milliseconds> timings;

  for (int i = 0; i < WINDOWS_TO_CREATE; i++) {
    auto start_time = std::chrono::high_resolution_clock::now();

    auto instance = GetModuleHandleA(nullptr);

    WNDCLASSA wc = {};
    wc.hCursor = LoadCursorA(nullptr, IDC_ARROW);
    wc.hInstance = instance;
    wc.lpszClassName = windowClassName;
    wc.style = CS_HREDRAW | CS_VREDRAW;
    wc.lpfnWndProc = WndProc;
    wc.cbClsExtra = 0;
    wc.cbWndExtra = 0;
    wc.hbrBackground = (HBRUSH)GetStockObject(WHITE_BRUSH);
    wc.lpszMenuName = nullptr;

    RegisterClassA(&wc);

    auto hwnd = CreateWindowExA(0, windowClassName, "This is a sample window",
                                WS_OVERLAPPEDWINDOW | WS_VISIBLE, CW_USEDEFAULT,
                                CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, 0,
                                0, instance, nullptr);

    MSG msg = {};

    while (GetMessageA(&msg, nullptr, 0, 0)) {
      TranslateMessage(&msg);
      DispatchMessageA(&msg);
    }

    DestroyWindow(hwnd);
    UnregisterClassA(windowClassName, instance);

    auto end_time = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(
        end_time - start_time);

    timings.push_back(duration);
  }

  auto total_time = std::reduce(timings.begin(), timings.end());
  auto avg_ms_per_window = total_time / WINDOWS_TO_CREATE;

  printf("Average time to create a window: %lld ms (%d windows)\n",
         avg_ms_per_window.count(), WINDOWS_TO_CREATE);

  return 0;
}
