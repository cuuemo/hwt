/*
 * func-name: sub_140007A10
 * func-address: 0x140007a10
 * callers: 0x140002008
 * callees: none
 */

LRESULT __fastcall sub_140007A10(HWND a1)
{
  HWND v1; // rax

  v1 = (HWND)SendMessageW(a1, 0x101Fu, 0, 0);
  return SendMessageW(v1, 0x1200u, 0, 0);
}
