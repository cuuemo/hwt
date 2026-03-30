/*
 * func-name: sub_1400052DC
 * func-address: 0x1400052dc
 * callers: 0x1400017a8
 * callees: 0x140002a74, 0x140002ae4, 0x1400047fc, 0x140006264, 0x140006b60, 0x140006d80, 0x1400072d8, 0x1400074d0, 0x1400074fc, 0x140007590, 0x14000885c, 0x140008ea0
 */

__int64 __fastcall sub_1400052DC(HINSTANCE a1, __int64 a2, __int64 a3, UINT a4)
{
  HANDLE ProcessHeap; // rax
  int v7; // eax
  HWND DesktopWindow; // rax
  struct tagMSG Msg; // [rsp+20h] [rbp-248h] BYREF
  WCHAR Filename[268]; // [rsp+50h] [rbp-218h] BYREF

  if ( !(unsigned int)sub_1400074FC() )
    sub_1400074D0();
  sub_140002A74();
  ProcessHeap = GetProcessHeap();
  hInstance = a1;
  hHeap = ProcessHeap;
  dword_1400107CC = sub_1400072D8();
  GetWindowsDirectoryW(&Buffer, 0x40u);
  GetSystemDirectoryW(&PathName, 0x40u);
  SetCurrentDirectoryW(&PathName);
  GetModuleFileNameW(0, &Parameters, 0x104u);
  sub_140007590(&Parameters, &pszPath);
  GetModuleFileNameA(0, MultiByteStr, 0x104u);
  PathRenameExtensionA(MultiByteStr, ".ini");
  GetModuleFileNameW(0, Filename, 0x104u);
  v7 = sub_140008EA0(Filename);
  if ( !v7 || v7 == -2146762487 )
  {
    sub_14000885C();
    sub_140006B60(&unk_140010640);
    sub_140006D80(&Parameters, &unk_1400106C0);
    sub_1400047FC(a4);
    hAccTable = LoadAcceleratorsW(hInstance, (LPCWSTR)0x66);
    while ( GetMessageW(&Msg, 0, 0, 0) )
    {
      if ( !Msg.hwnd && !Msg.message )
        Sleep(0);
      if ( !TranslateAcceleratorW(hDlg, hAccTable, &Msg) && !IsDialogMessageW(hDlg, &Msg) )
      {
        TranslateMessage(&Msg);
        DispatchMessageW(&Msg);
      }
    }
    sub_140002AE4();
    return 0;
  }
  else
  {
    DesktopWindow = GetDesktopWindow();
    sub_140006264(DesktopWindow, "Certificate check fail, EXE was manipulated", "DeviceCleanup", 16);
    return 255;
  }
}
