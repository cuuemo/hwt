/*
 * func-name: sub_140006128
 * func-address: 0x140006128
 * callers: 0x140002b64, 0x140002e90, 0x140003cec, 0x140006264, 0x140007b74
 * callees: 0x140006d48
 */

__int64 __fastcall sub_140006128(HWND a1, const WCHAR *a2, const WCHAR *a3, int a4)
{
  HWND DialogParamW; // rbx
  HACCEL AcceleratorsW; // rdi
  int MessageW; // eax
  struct tagMSG Msg; // [rsp+30h] [rbp-38h] BYREF

  qword_140010E40 = a1;
  lstrcpyW(&word_140010E50, a3);
  lstrcpyW(word_140010F50, a2);
  dword_140014F50 = a4;
  sub_140006D48(word_140010F50, 10);
  DialogParamW = CreateDialogParamW(0, (LPCWSTR)0x7D0, 0, (DLGPROC)sub_140005518, 0);
  AcceleratorsW = LoadAcceleratorsW(0, (LPCWSTR)0x66);
  while ( 1 )
  {
    MessageW = GetMessageW(&Msg, 0, 0, 0);
    if ( !MessageW )
      break;
    if ( MessageW != -1
      && !TranslateAcceleratorW(DialogParamW, AcceleratorsW, &Msg)
      && !IsDialogMessageW(DialogParamW, &Msg) )
    {
      TranslateMessage(&Msg);
      DispatchMessageW(&Msg);
    }
  }
  if ( qword_140010E40 && qword_140010E40 != GetDesktopWindow() )
    EnableWindow(qword_140010E40, 1);
  if ( dword_140014F54 )
  {
    DeleteObject(wParam);
    wParam = 0;
    dword_140014F54 = 0;
  }
  DestroyWindow(DialogParamW);
  return 1;
}
