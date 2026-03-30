/*
 * func-name: sub_140008440
 * func-address: 0x140008440
 * callers: 0x140005518
 * callees: none
 */

BOOL __fastcall sub_140008440(HWND hWnd, HWND a2)
{
  HWND DesktopWindow; // rbx
  LONG SystemMetrics; // ebx
  LONG v5; // eax
  LONG right; // ecx
  LONG left; // r10d
  LONG top; // r11d
  LONG bottom; // r9d
  int v10; // eax
  struct tagRECT v12; // [rsp+40h] [rbp-28h] BYREF
  struct tagRECT Rect; // [rsp+50h] [rbp-18h] BYREF

  DesktopWindow = a2;
  if ( !a2 )
    DesktopWindow = GetDesktopWindow();
  GetWindowRect(hWnd, &Rect);
  GetWindowRect(DesktopWindow, &v12);
  SystemMetrics = GetSystemMetrics(0);
  v5 = GetSystemMetrics(1);
  right = v12.right;
  left = v12.left;
  top = v12.top;
  bottom = v12.bottom;
  if ( v12.left < 0 )
    left = 0;
  if ( v12.top < 0 )
    top = 0;
  v12.left = left;
  if ( v12.right > SystemMetrics )
    right = SystemMetrics;
  v12.top = top;
  if ( v12.bottom > v5 )
    bottom = v5;
  v12.right = right;
  v12.bottom = bottom;
  v10 = top + (bottom + Rect.top - Rect.bottom - top) / 2;
  if ( v10 < 0 )
    v10 = 0;
  return SetWindowPos(hWnd, 0, left + (right + Rect.left - Rect.right - left) / 2, v10, 0, 0, 5u);
}
