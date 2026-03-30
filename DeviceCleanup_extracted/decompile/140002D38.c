/*
 * func-name: sub_140002D38
 * func-address: 0x140002d38
 * callers: 0x140003010, 0x1400036d4, 0x140003cec, 0x14000453c
 * callees: none
 */

HCURSOR __fastcall sub_140002D38(LPCWSTR lpCursorName, int a2)
{
  HCURSOR result; // rax
  struct tagPOINT Point; // [rsp+30h] [rbp+8h] BYREF

  hCursor = LoadCursorW(0, lpCursorName);
  SetClassLongPtrW(hDlg, -12, (int)hCursor);
  SetClassLongPtrW(hWnd, -12, (int)hCursor);
  if ( lpCursorName != (LPCWSTR)32650 )
    SetClassLongPtrW(hWnd, -12, (int)hCursor);
  result = SetCursor(hCursor);
  if ( a2 )
  {
    GetCursorPos(&Point);
    SetCursorPos(++Point.x, Point.y);
    SetCursorPos(--Point.x, Point.y);
    return SetCursor(hCursor);
  }
  return result;
}
