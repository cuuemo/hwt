/*
 * func-name: sub_140001E04
 * func-address: 0x140001e04
 * callers: 0x140003cec, 0x1400047fc
 * callees: 0x140001854
 */

BOOL __fastcall sub_140001E04(UINT item, int a2)
{
  MENUITEMINFOW mii; // [rsp+20h] [rbp-58h] BYREF

  sub_140001854(&mii.fMask, 0, 0x4Cu);
  mii.cbSize = 80;
  mii.fState = a2 != 0 ? 8 : 0;
  mii.fMask = 1;
  return SetMenuItemInfoW(hmenu, item, 0, &mii);
}
