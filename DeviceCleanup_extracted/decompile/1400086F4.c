/*
 * func-name: sub_1400086F4
 * func-address: 0x1400086f4
 * callers: 0x140001e64, 0x140003010, 0x1400047fc, 0x140005518
 * callees: 0x1400085c4
 */

int __fastcall sub_1400086F4(int a1)
{
  int v2; // eax

  v2 = sub_1400085C4();
  return MulDiv(a1, v2, 96);
}
