/*
 * func-name: sub_140007530
 * func-address: 0x140007530
 * callers: 0x140002804, 0x1400047fc, 0x140005518, 0x140008aec
 * callees: none
 */

BOOL __fastcall sub_140007530(HWND a1, struct tagPOINT *a2)
{
  ScreenToClient(a1, a2);
  return ScreenToClient(a1, a2 + 1);
}
