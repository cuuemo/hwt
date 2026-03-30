/*
 * func-name: sub_140007A40
 * func-address: 0x140007a40
 * callers: 0x1400047fc
 * callees: none
 */

__int64 __fastcall sub_140007A40(HWND a1)
{
  HWND v1; // rax
  struct tagRECT Rect; // [rsp+20h] [rbp-18h] BYREF

  v1 = (HWND)SendMessageW(a1, 0x101Fu, 0, 0);
  if ( GetWindowRect(v1, &Rect) )
    return (unsigned int)(Rect.bottom - Rect.top);
  else
    return 0;
}
