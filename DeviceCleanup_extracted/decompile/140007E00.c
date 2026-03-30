/*
 * func-name: sub_140007E00
 * func-address: 0x140007e00
 * callers: 0x140007eb4, 0x140008534
 * callees: none
 */

BOOL __fastcall sub_140007E00(HWND hWnd)
{
  int v2; // edi
  int v3; // ebx
  int SystemMetrics; // esi
  int v5; // eax
  LONG left; // r8d
  LONG top; // r9d
  struct tagRECT v9; // [rsp+40h] [rbp-18h] BYREF

  GetWindowRect(hWnd, &v9);
  v2 = v9.right - v9.left;
  v3 = v9.bottom - v9.top;
  SystemMetrics = GetSystemMetrics(0);
  v5 = GetSystemMetrics(1);
  left = v9.left;
  top = v9.top;
  if ( v9.right < 30 )
    left = 0;
  if ( v9.left > SystemMetrics - 30 )
    left = SystemMetrics - v2;
  if ( v9.bottom < 30 )
    top = 0;
  if ( v9.bottom > v5 - 30 )
    top = v5 - v3;
  return SetWindowPos(hWnd, 0, left, top, 0, 0, 5u);
}
