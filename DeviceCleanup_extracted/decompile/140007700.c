/*
 * func-name: sub_140007700
 * func-address: 0x140007700
 * callers: 0x140007738, 0x1400077d8
 * callees: 0x140001840, 0x140001878
 */

unsigned __int64 __fastcall sub_140007700(__int64 a1)
{
  __int64 v2; // [rsp+38h] [rbp+10h] BYREF

  sub_140001878(&v2, a1, 8);
  return sub_140001840(qword_140016FC8 - v2, 0x2710u, 0);
}
