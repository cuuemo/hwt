/*
 * func-name: sub_140003C1C
 * func-address: 0x140003c1c
 * callers: 0x140004704
 * callees: 0x1400012f4, 0x1400018d4, 0x140002008
 */

BOOL __fastcall sub_140003C1C(HWND hWnd)
{
  void *v2; // rbx

  dword_14001078C = 1;
  sub_140002008();
  ShowWindow(hWnd, 0);
  v2 = (void *)qword_140010780;
  if ( qword_140010780 )
  {
    sub_1400012F4((__int64 *)qword_140010780);
    sub_1400018D4(v2);
  }
  DestroyMenu(hMenu);
  DestroyMenu(qword_140010638);
  DeleteObject(hBitmapUnchecked);
  if ( qword_140010770 )
    DeleteObject(qword_140010770);
  if ( Handle )
    UnregisterDeviceNotification(Handle);
  Sleep(0x96u);
  PostQuitMessage(0);
  EndDialog(hWnd, 0);
  DestroyWindow(hWnd);
  return SetupDiDestroyClassImageList(&ClassImageListData);
}
