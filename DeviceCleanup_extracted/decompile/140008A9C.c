/*
 * func-name: sub_140008A9C
 * func-address: 0x140008a9c
 * callers: 0x140005518
 * callees: none
 */

__int64 __fastcall sub_140008A9C(HDC a1, WCHAR *a2)
{
  struct tagRECT rc; // [rsp+30h] [rbp-18h] BYREF

  rc.left = 0;
  rc.top = 0;
  rc.right = 0;
  rc.bottom = 0;
  DrawTextExW(a1, a2, -1, &rc, 0xC40u, 0);
  return (unsigned int)rc.right;
}
