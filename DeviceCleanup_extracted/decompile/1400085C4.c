/*
 * func-name: sub_1400085C4
 * func-address: 0x1400085c4
 * callers: 0x140002008, 0x1400047fc, 0x1400086f4
 * callees: none
 */

__int64 sub_1400085C4()
{
  HWND DesktopWindow; // rsi
  HMODULE ModuleHandleW; // rax
  FARPROC GetDpiForWindow; // rax
  __int64 result; // rax
  HMODULE LibraryA; // rax
  HMODULE v5; // rbx
  FARPROC GetDpiForMonitor; // rdi
  HMONITOR v7; // rax
  HDC DC; // rdi
  unsigned int DeviceCaps; // ebx
  struct tagRECT Rect; // [rsp+20h] [rbp-18h] BYREF
  unsigned int v11; // [rsp+40h] [rbp+8h] BYREF
  char v12; // [rsp+48h] [rbp+10h] BYREF

  DesktopWindow = hDlg;
  if ( !hDlg )
    DesktopWindow = GetDesktopWindow();
  if ( dword_140010680 >= 4096 && dword_140010670 >= 14393 )
  {
    ModuleHandleW = GetModuleHandleW(L"user32.dll");
    if ( ModuleHandleW )
    {
      GetDpiForWindow = GetProcAddress(ModuleHandleW, "GetDpiForWindow");
      if ( GetDpiForWindow )
        return ((__int64 (__fastcall *)(HWND))GetDpiForWindow)(DesktopWindow);
    }
    goto LABEL_13;
  }
  if ( dword_140010680 < 1539 )
    goto LABEL_13;
  LibraryA = LoadLibraryA("Shcore.dll");
  v5 = LibraryA;
  if ( !LibraryA
    || (GetDpiForMonitor = GetProcAddress(LibraryA, "GetDpiForMonitor")) == 0
    || (GetWindowRect(DesktopWindow, &Rect), (v7 = MonitorFromRect(&Rect, 2u)) == 0)
    || (v11 = 0,
        ((void (__fastcall *)(HMONITOR, _QWORD, unsigned int *, char *))GetDpiForMonitor)(v7, 0, &v11, &v12),
        FreeLibrary(v5),
        (result = v11) == 0) )
  {
LABEL_13:
    DC = GetDC(DesktopWindow);
    DeviceCaps = GetDeviceCaps(DC, 88);
    ReleaseDC(DesktopWindow, DC);
    return DeviceCaps;
  }
  return result;
}
