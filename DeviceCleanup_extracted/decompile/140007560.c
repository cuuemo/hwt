/*
 * func-name: sub_140007560
 * func-address: 0x140007560
 * callers: 0x1400026cc
 * callees: none
 */

BOOL __fastcall sub_140007560(HWND a1, struct tagPOINT *a2)
{
  ClientToScreen(a1, a2);
  return ClientToScreen(a1, a2 + 1);
}
