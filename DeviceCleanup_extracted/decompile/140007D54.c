/*
 * func-name: sub_140007D54
 * func-address: 0x140007d54
 * callers: 0x1400047fc
 * callees: 0x140001854
 */

BOOL __fastcall sub_140007D54(HMENU hmenu, UINT item)
{
  MENUITEMINFOW mii; // [rsp+20h] [rbp-58h] BYREF

  mii.fType = 0;
  mii.cbSize = 80;
  mii.fMask = 1;
  mii.fState = 3;
  sub_140001854(&mii.wID, 0, 0x40u);
  return SetMenuItemInfoW(hmenu, item, 0, &mii);
}
