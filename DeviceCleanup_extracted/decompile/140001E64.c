/*
 * func-name: sub_140001E64
 * func-address: 0x140001e64
 * callers: 0x140004704
 * callees: 0x1400086f4
 */

__int64 __fastcall sub_140001E64(HWND hRecipient)
{
  int cy; // eax
  HANDLE ImageW; // rdi
  int v4; // eax
  HANDLE v5; // rbx
  int SystemMetrics; // eax
  HMODULE v7; // rax
  HBITMAP v8; // rax
  HMODULE v9; // rax
  HMODULE v10; // rax
  HMODULE ModuleHandleW; // rax
  HMENU Menu; // rax
  _DWORD NotificationFilter[6]; // [rsp+30h] [rbp-18h] BYREF

  cy = sub_1400086F4(32);
  ImageW = LoadImageW(hInstance, (LPCWSTR)0x64, 1u, cy, cy, 0x8000u);
  v4 = sub_1400086F4(16);
  v5 = LoadImageW(hInstance, (LPCWSTR)0x64, 1u, v4, v4, 0x8000u);
  SendMessageW(hRecipient, 0x80u, 1u, (LPARAM)ImageW);
  SendMessageW(hRecipient, 0x80u, 0, (LPARAM)v5);
  SystemMetrics = GetSystemMetrics(71);
  if ( SystemMetrics >= 16 )
  {
    if ( SystemMetrics >= 26 )
    {
      if ( SystemMetrics >= 39 )
      {
        ModuleHandleW = GetModuleHandleW(0);
        v8 = (HBITMAP)LoadImageW(ModuleHandleW, (LPCWSTR)0xCB, 0, 39, 39, 0);
      }
      else
      {
        v10 = GetModuleHandleW(0);
        v8 = (HBITMAP)LoadImageW(v10, (LPCWSTR)0xCA, 0, 26, 26, 0);
      }
    }
    else
    {
      v9 = GetModuleHandleW(0);
      v8 = (HBITMAP)LoadImageW(v9, (LPCWSTR)0xC9, 0, 16, 16, 0);
    }
  }
  else
  {
    v7 = GetModuleHandleW(0);
    v8 = (HBITMAP)LoadImageW(v7, (LPCWSTR)0xC8, 0, 13, 13, 0);
  }
  hBitmapUnchecked = v8;
  Menu = GetMenu(hRecipient);
  SetMenuItemBitmaps(Menu, 0x9CA5u, 0, hBitmapUnchecked, hBitmapUnchecked);
  NotificationFilter[2] = 0;
  NotificationFilter[1] = 5;
  NotificationFilter[0] = 12;
  Handle = RegisterDeviceNotificationW(hRecipient, NotificationFilter, 4u);
  return 1;
}
